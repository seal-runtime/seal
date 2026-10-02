use std::borrow::Cow;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::SystemTime;

use archive::{ArchiveBuilder, ArchiveEntry, ArchiveFormat};
use mluau::prelude::*;
use crate::prelude::*;
use crate::std_fs::entry::{wrap_io_read_errors, wrap_io_read_errors_empty};
use crate::std_fs::file_size::FileSize;
use crate::std_io::colors;
use crate::std_time::datetime::DateTime;
use crate::userdata::{
    SealLock, SealUserData as LuaUserDataMut, SealUserDataFields as LuaUserDataFieldsMut, SealUserDataMethods as LuaUserDataMethodsMut
};

enum ArchiveOutputPath {
    As(PathBuf),
    KeepChildren(usize),
}

#[derive(PartialEq)]
pub(super) enum EntryType {
    File,
    Directory,
    Symlink
}
const _VALID_ENTRY_TYPES: &str = "\"File\", \"Directory\", or \"Symlink\"";
impl EntryType {
    fn _from_value(value: LuaValue) -> LuaResult<Self> {
        let s = match value {
            LuaValue::String(s) => s.as_bytes().to_vec(),
            other => {
                return wrap_err!("expected entry.type to be a string ({}), got: {:?}", _VALID_ENTRY_TYPES, other);
            }
        };
        let t = if s.eq_ignore_ascii_case(b"file") {
            Self::File
        } else if s.eq_ignore_ascii_case(b"directory") {
            Self::Directory
        } else if s.eq_ignore_ascii_case(b"symlink") {
            Self::Symlink
        } else {
            let displayable = bstr::BStr::new(&s);
            return wrap_err!("expected entry.type to be {}, got: {}", _VALID_ENTRY_TYPES, displayable);
        };
        Ok(t)
    }
}

impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            EntryType::File => "File",
            EntryType::Directory => "Directory",
            EntryType::Symlink => "Symlink"
        })
    }
}

pub(super) enum Entries {
    One(WrappedArchiveEntry),
    Many(Vec<WrappedArchiveEntry>)
}
impl Entries {
    fn expect_one(self) -> WrappedArchiveEntry {
        match self {
            Self::One(entry) => entry,
            _ => {
                panic!("this should not be called on Entries with Many entries");
            }
        }
    }
    fn into_value(self, luau: &Lua) -> LuaValueResult {
        match self {
            Self::One(entry) => entry.into_userdata(luau),
            Self::Many(entries) => {
                ok_table(luau.create_sequence_from(entries.into_iter().map(|x| x.into_mut())))
            }
        }
    }
    pub fn from_userdata(ud: LuaAnyUserData, index: usize, function_name: &'static str) -> LuaResult<Self> {
        match WrappedArchiveEntry::cloned(ud) {
            Ok(entry) => Ok(Entries::One(entry)),
            Err(type_name) => {
                wrap_err!("{}: entry passed at index {} is the incorrect kind of userdata (expected ArchiveEntry or {{ ArchiveEntry }}, got: {})", function_name, index, type_name)
            }
        }
    }
    pub fn from_table(t: LuaTable, multivalue_index: usize, function_name: &'static str) -> LuaResult<Self> {
        let mut entries = Vec::new();
        for (index, pair) in t.pairs().enumerate() {
            let (got_index, element): (LuaValue, LuaValue) = pair?;
            let index = {
                let LuaValue::Integer(index) = got_index else {
                    return wrap_err!("{}: expected array like table (with whole number keys) or ArchiveEntry at argument index {}, but key in the table at index {} is not a whole number (got {:?})", function_name, multivalue_index, index, got_index);
                };
                int_to_usize(index, function_name, "array-like table index")?
            };
            let LuaValue::UserData(ud) = element else {
                return wrap_err!("{}: expected value of array-like table at argument index {} at in the table at index {} to be an ArchiveEntry (from @std/archive/entry), got: {:?}", function_name, multivalue_index, index, element);
            };
            let entry = Self::from_userdata(ud, index, function_name)?.expect_one();
            entries.push(entry);
        }
        Ok(Self::Many(entries))
    }
}

#[derive(Clone)]
pub struct WrappedArchiveEntry {
    inner: Rc<RefCell<ArchiveEntry>>
}
impl WrappedArchiveEntry {
    pub fn new(entry: ArchiveEntry) -> Self {
        Self {
            inner: Rc::new(RefCell::new(entry))
        }
    }
    pub fn file(path: impl AsRef<Path>, contents: impl Into<Vec<u8>>) -> Self {
        Self::new(ArchiveEntry::file(path, contents))
    }
    pub fn directory(path: impl AsRef<Path>) -> Self {
        Self::new(ArchiveEntry::directory(path))
    }
    pub fn symlink(path: impl AsRef<Path>, target: impl AsRef<Path>) -> Self {
        Self::new(ArchiveEntry::symlink(path, target))
    }
    pub fn path(&self) -> String {
        let t = self.inner.borrow();
        t.path().to_owned()
    }
    pub fn from_entries(mut entries: Vec<ArchiveEntry>) -> Vec<Self> {
        let mut new_entries = Vec::with_capacity(entries.len());
        while let Some(next) = entries.pop() {
            new_entries.insert(0, Self::new(next));
        }
        new_entries
    }
    pub fn swap(&mut self, with: ArchiveEntry) -> ArchiveEntry {
        self.inner.replace(with)
    }
    fn set_mtime(&mut self, mtime: SystemTime) {
        if let Ok(mut borr) = self.inner.try_borrow_mut() {
            borr.set_mtime(mtime);
        }
    }
    fn set_mode(&mut self, mode: u32) {
        if let Ok(mut borr) = self.inner.try_borrow_mut() {
            borr.set_mode(mode);
        }
    }
    fn with_metadata_from_disk<P: AsRef<Path>>(mut self, path: P) -> Result<Self, std::io::Error> {
        let metadata = std::fs::metadata(&path)?;
        let modified = metadata.modified().ok();

        #[cfg_attr(target_os = "windows", expect(unused_mut))]
        #[cfg_attr(target_family = "unix", expect(unused_assignments))]
        let mut unix_mode = None;
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            unix_mode = Some(metadata.mode());
        }

        if let Some(modified) = modified {
            self.set_mtime(modified);
        }

        if let Some(mode) = unix_mode {
            self.set_mode(mode);
        }

        Ok(self)
    }

    /// Reads a single file at `disk_path` into an entry whose archive path is exactly `archive_path`
    fn from_file_on_disk(disk_path: &Path, archive_path: &Path) -> Result<Self, std::io::Error> {
        let contents = std::fs::read(disk_path)?;
        Self::file(archive_path, contents).with_metadata_from_disk(disk_path)
    }

    fn trim_path_components(path: &Path, keep_children: usize) -> PathBuf {
        // When we make archive entries from file we need to ensure that we don't save
        // the entire absolute path of the file as the 'path' because that creates archive
        // with absolute paths (which even we reject by default)
        let buffy = std::path::PathBuf::new();

        // get the last n components of the path (usually 1 unless nested dirs)
        // and basically 'collect' (fold) them into a PathBuf that we alloc above
        let components = &mut path
            .components()
            .rev()
            .take(keep_children)
            .try_fold(buffy, |mut acc, comp| {
                if let std::path::Component::Normal(c) = comp {
                    acc.push(c);
                    std::ops::ControlFlow::Continue(acc)
                } else {
                    std::ops::ControlFlow::Break(acc)
                }
            });

        match components {
            std::ops::ControlFlow::Continue(b) => {
                // components are backwards so we have to reverse them again
                // unfortunately can't do this easily without allocing another PathBuf
                b.components().rev().collect::<PathBuf>()
            },
            std::ops::ControlFlow::Break(_) => {
                std::path::PathBuf::from(path
                    .file_name()
                    .unwrap_or(std::ffi::OsStr::new("<path not found>"))
                    .to_owned()
                )
            }
        }
    }

    fn from_file_on_disk_trimmed(path: &Path, keep_children: usize) -> Result<Self, std::io::Error> {
        let contents = std::fs::read(path)?;
        let archive_path = Self::trim_path_components(path, keep_children);
        Self::file(&archive_path, contents).with_metadata_from_disk(path)
    }
    
    /// Recursively walks `path` on disk, pushing one `Directory` entry for path (in case dir is empty)
    /// and then goes though each child and creates archive entries for each child, joining the child's name
    /// to this directory's reference `archive_path` to ensure file at `path/a/b.txt` gets the path
    /// `archive_path/a/b.txt`, not just `archive_path` (previous bug)
    fn from_directory_on_disk(
        disk_path: &Path,
        entries: &mut Vec<Self>,
        archive_path: &Path,
    ) -> Result<(), std::io::Error> {
        let directory_entry = Self::directory(archive_path).with_metadata_from_disk(disk_path)?;
        entries.push(directory_entry);

        for entry in std::fs::read_dir(disk_path)? {
            let entry = entry?;
            let disk_path = entry.path();
            #[allow(clippy::disallowed_methods, reason = "entry.file_name() cannot start with root sep")]
            let child_archive_path = archive_path.join(entry.file_name());

            if disk_path.is_dir() {
                Self::from_directory_on_disk(&disk_path, entries, &child_archive_path)?;
            } else if disk_path.is_file() {
                let f = match Self::from_file_on_disk(&disk_path, &child_archive_path) {
                    Ok(e) => e,
                    Err(err) => {
                        return Err(std::io::Error::other(format!("failed to handle path '{}' due to err: {}", disk_path.display(), err)));
                    }
                };
                entries.push(f);
            }
        }

        Ok(())
    }

    pub(super) fn whatami(&self) -> EntryType {
        let inner = match self.inner.try_borrow() {
            Ok(inner) => inner,
            Err(_) => {
                panic!("whatami() was called while self.inner was already mutably borrowed; this is a logic bug. \
                Reorder the call so that self/this.whatami() gets called *before* this.inner.borrow_mut()")
            }
        };
        if inner.is_file() {
            EntryType::File
        } else if inner.is_directory() {
            EntryType::Directory
        } else {
            EntryType::Symlink
        }
    }

    pub fn symlink_target(&self) -> Option<String> {
        let inner = self.inner.borrow();
        inner.symlink_target().map(|tar| tar.to_owned())
    }

    fn from_path_on_disk(path: &Path, out_path: Option<ArchiveOutputPath>) -> LuaResult<Entries> {
        let function_name = "entry.read(path: Pathlike, as: (Pathlike | number)?)";
        if path.is_file() {
            let result = match out_path {
                Some(ArchiveOutputPath::As(out)) => Self::from_file_on_disk(path, &out),
                Some(ArchiveOutputPath::KeepChildren(n)) => Self::from_file_on_disk_trimmed(path, n),
                None => Self::from_file_on_disk_trimmed(path, 1),
            };
            match result {
                Ok(entry) => Ok(Entries::One(entry)),
                Err(err) => {
                    wrap_io_read_errors(err, function_name, path)
                }
            }
        } else if path.is_dir() {
            let archive_path = match out_path {
                Some(ArchiveOutputPath::As(out)) => out,
                Some(ArchiveOutputPath::KeepChildren(n)) => Self::trim_path_components(path, n),
                None => Self::trim_path_components(path, 1),
            };

            let mut entries = Vec::new();
            if let Err(err) = Self::from_directory_on_disk(path, &mut entries, &archive_path) {
                return wrap_io_read_errors(err, function_name, path);
            }
            Ok(Entries::Many(entries))
        } else {
            wrap_err!("expected path to file or directory, got path to something else. If you want to add a symlink, construct it via entry.create.symlink")
        }
    }

    fn entry_expect_self(luau: &Lua, this: &WrappedArchiveEntry, what: EntryType, function_name: &'static str) -> LuaValueResult {
        let inner = this.inner.borrow();
        let success = match what {
            EntryType::File => inner.is_file(),
            EntryType::Directory => inner.is_directory(),
            EntryType::Symlink => inner.is_symlink(),
        };
        if success {
            this.clone().into_userdata(luau)
        } else {
            wrap_err!("{}: expected self to be a {}, but self is actually a {}", function_name, what, this.whatami())
        }
    }

    fn entry_try_self(luau: &Lua, this: &WrappedArchiveEntry, what: EntryType) -> LuaValueResult {
        let inner = this.inner.borrow();
        let success = match what {
            EntryType::File => inner.is_file(),
            EntryType::Directory => inner.is_directory(),
            EntryType::Symlink => inner.is_symlink(),
        };
        if success {
            this.clone().into_userdata(luau)
        } else {
            Ok(LuaNil)
        }
    }

    fn entry_try_file(luau: &Lua, this: &WrappedArchiveEntry, _: LuaValue) -> LuaValueResult {
        Self::entry_try_self(luau, this, EntryType::File)
    }

    fn entry_expect_file(luau: &Lua, this: &WrappedArchiveEntry, _: LuaValue) -> LuaValueResult {
        let function_name = "ArchiveEntry:expect_file()";
        Self::entry_expect_self(luau, this, EntryType::File, function_name)
    }

    fn entry_try_directory(luau: &Lua, this: &WrappedArchiveEntry, _: LuaValue) -> LuaValueResult {
        Self::entry_try_self(luau, this, EntryType::Directory)
    }

    fn entry_expect_directory(luau: &Lua, this: &WrappedArchiveEntry, _: LuaValue) -> LuaValueResult {
        let function_name = "ArchiveEntry:expect_directory()";
        Self::entry_expect_self(luau, this, EntryType::Directory, function_name)
    }

    fn entry_try_symlink(luau: &Lua, this: &WrappedArchiveEntry, _: LuaValue) -> LuaValueResult {
        Self::entry_try_self(luau, this, EntryType::Symlink)
    }

    fn entry_expect_symlink(luau: &Lua, this: &WrappedArchiveEntry, _: LuaValue) -> LuaValueResult {
        let function_name = "ArchiveEntry:expect_symlink()";
        Self::entry_expect_self(luau, this, EntryType::Symlink, function_name)
    }

    fn entry_get_modified(luau: &Lua, this: &WrappedArchiveEntry, _: LuaValue) -> LuaValueResult {
        let function_name = "ArchiveEntry:modified()";
        let inner = this.inner.borrow();
        if let Some(modified) = inner.mtime() {
            ok_userdata_mut(DateTime::from_system_time(modified, function_name)?, luau)
        } else {
            Ok(LuaNil)
        }
    }

    fn entry_set_modified(_luau: &Lua, this: &mut WrappedArchiveEntry, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
        let function_name = "ArchiveEntry:set_modified(dt: DateTime)";
        let dt = match multivalue.pop_front() {
            Some(LuaValue::UserData(ud)) => match ud.borrow::<SealLock<DateTime>>() {
                Some(dt) => dt.borrow()._to_system_time(),
                None => {
                    return wrap_err!("{}: expected dt to be a DateTime userdata, got the wrong type of userdata {:?}", function_name, ud.type_name());
                }
            },
            Some(LuaNil) | None => {
                return wrap_err!("{}: expected dt to be a DateTime, got nothing or nil", function_name);
            },
            Some(other) => {
                return wrap_err!("{}: expected dt to be a DateTime userdata, got: {:?}", function_name, other);
            }
        };
        this.set_mtime(dt);
        Ok(())
    }

    fn entry_get_unix_mode(luau: &Lua, this: &WrappedArchiveEntry, _: LuaValue) -> LuaValueResult {
        let inner = this.inner.borrow();
        match inner.mode() {
            Some(mode) => mode.into_lua(luau),
            None => Ok(LuaNil),
        }
    }

    fn entry_set_unix_mode(_luau: &Lua, this: &mut WrappedArchiveEntry, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
        let function_name = "ArchiveEntry:set_unix_mode(mode: number)";
        let mode = match multivalue.pop_front() {
            Some(LuaValue::Integer(i)) => int_to_u32(i, function_name, "mode")?,
            Some(LuaValue::Number(f)) => float_to_u32(f, function_name, "mode")?,
            Some(LuaNil) | None => {
                return wrap_err!("{}: expected mode to be a positive whole number, got nothing or nil", function_name);
            },
            Some(other) => {
                return wrap_err!("{}: expected mode to be a number, got: {:?}", function_name, other);
            }
        };
        this.set_mode(mode);
        Ok(())
    }

    fn file_get_size(luau: &Lua, this: &WrappedArchiveEntry, _: LuaValue) -> LuaValueResult {
        let inner = this.inner.borrow();
        if let Some(data) = inner.data() {
            FileSize::from_bytes(data.len() as u64).into_userdata(luau)
        } else {
            Ok(LuaNil)
        }
    }

    fn file_get_contents(luau: &Lua, this: &WrappedArchiveEntry, _: LuaValue) -> LuaValueResult {
        let function_name = "ArchiveEntry:contents()";
        let inner = this.inner.borrow();
        if !inner.is_file() {
            return wrap_err!("{}: self must be a File, got: {}", function_name, this.whatami());
        }

        let contents = inner.data().expect("data should be present for ArchiveEntry::File");
        ok_string(contents, luau)
    }

    fn file_compress(luau: &Lua, this: &WrappedArchiveEntry, mut multivalue: LuaMultiValue) -> LuaValueResult {
        let function_name = "ArchiveEntryFile:compress(format: SingleFileFormat)";
        let format = match multivalue.pop_front() {
            Some(LuaValue::String(s)) => s.to_string_lossy(),
            Some(LuaNil) | None => {
                return wrap_err!("{}: expected a format, got nothing or nil", function_name);
            },
            Some(other) => {
                return wrap_err!("{}: expected format to be a string, got: {:?}", function_name, other);
            }
        };
        let format = archive_format_from_str(&format, function_name)?;

        let inner = this.inner.borrow();
        if !inner.is_file() {
            return wrap_err!("{}: self must be a File, got: {}", function_name, this.whatami());
        }
        let path = inner.path().to_owned();
        let data = inner.data().expect("data should be present for ArchiveEntry::File").to_owned();

        let builder = ArchiveBuilder::new();
        let compressed = match builder.build_single_file(path, data, format) {
            Ok(bytes) => bytes,
            Err(err) => {
                return wrap_err!("{}: unable to compress to {} due to err: {}", function_name, format.name(), err);
            }
        };

        ok_buffy(compressed, luau)
    }

    fn file_extract(_luau: &Lua, this: &WrappedArchiveEntry, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
        let function_name = "ArchiveEntryFile:extract(destination: string)";
        let destination = super::expect_pathlike(multivalue.pop_front(), function_name)?;

        let inner = this.inner.borrow();
        if !inner.is_file() {
            return wrap_err!("{}: self must be a File, got: {}", function_name, this.whatami());
        }
        let data = inner.data().expect("data should be present for ArchiveEntry::File");
        let mode = inner.mode();
        let mtime = inner.mtime();

        let path = Path::new(&destination);
        if let Some(parent) = path.parent() 
            && let Err(err) = std::fs::create_dir_all(parent)
        {
            return wrap_io_read_errors_empty(err, function_name, parent);
        }

        if let Err(err) = std::fs::write(path, data) {
            return wrap_io_read_errors_empty(err, function_name, path);
        }

        super::extract::apply_mode(path, mode, function_name)?;
        super::extract::apply_mtime(path, mtime, function_name)?;

        Ok(())
    }

    fn file_set_contents(_luau: &Lua, this: &mut WrappedArchiveEntry, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
        let function_name = "ArchiveEntry:set_contents(new: string | buffer)";
        if this.whatami() != EntryType::File {
            return wrap_err!("{}: self must be a File, got: {}", function_name, this.whatami());
        }
        let mut inner = this.inner.borrow_mut();

        let data = match multivalue.pop_front() {
            Some(LuaValue::String(s)) => s.as_bytes().to_owned(),
            Some(LuaValue::Buffer(buffy)) => buffy.to_vec(),
            Some(LuaNil) | None => {
                return wrap_err!("{}: new contents should be string or buffer but was called with zero arguments", function_name);
            },
            Some(other) => {
                return wrap_err!("{}: expected new contents to be a string or buffer, got: {:?}", function_name, other);
            }
        };

        inner.set_data(data);

        Ok(())
    }

    fn symlink_get_target(luau: &Lua, this: &WrappedArchiveEntry, _: LuaValue) -> LuaValueResult {
        let function_name = "ArchiveEntrySymlink:target()";
        let inner = this.inner.borrow();
        let sym = if inner.is_symlink() {
            inner.symlink_target().expect("must be symlink")
        } else {
            return wrap_err!("{} is not a symlink", function_name);
        };
        ok_string(sym, luau)
    }

    fn symlink_set_target(_luau: &Lua, this: &mut WrappedArchiveEntry, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
        let function_name = "ArchiveEntrySymlink:set_target(target: string)";
        let target = expect_stringy_path(multivalue.pop_front(), function_name)?;
        if this.whatami() != EntryType::Symlink {
            return wrap_err!("{}: self must be a Symlink, got: {}", function_name, this.whatami());
        }
        let mut inner = this.inner.borrow_mut();
        inner.set_symlink_target(target);
        Ok(())
    }

    fn entry_get_path(luau: &Lua, this: &WrappedArchiveEntry, _: LuaValue) -> LuaValueResult {
        this.path().into_lua(luau)
    }

    fn entry_set_path(_luau: &Lua, this: &mut WrappedArchiveEntry, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
        let function_name = "ArchiveEntry:set_path(path: string)";
        let path = expect_stringy_path(multivalue.pop_front(), function_name)?;
        let mut inner = this.inner.borrow_mut();
        inner.set_path(path);
        Ok(())
    }

    fn entry_display(luau: &Lua, this: &WrappedArchiveEntry, _: LuaMultiValue) -> LuaValueResult {
        let function_name = "ArchiveEntry:display()";
        let what = this.whatami();
        let inner = this.inner.borrow();

        let no_color = colors::are_disabled();
        let colorize = |text: String, color: &str| -> String {
            if no_color {
                text
            } else {
                format!("{}{}{}", color, text, colors::RESET)
            }
        };

        let path = colorize(format!("\"{}\"", inner.path()), colors::GREEN);
        let mut parts = vec![format!("{}: {}", what, path)];

        match what {
            EntryType::File => {
                let size = inner.data().map(|data| data.len() as u64).unwrap_or(0);
                parts.push(colorize(FileSize::from_bytes(size).display(), colors::MAGENTA));
            },
            EntryType::Symlink => {
                let target = inner.symlink_target().expect("symlink must have symlink target");
                let target = colorize(format!("\"{}\"", target), colors::GREEN);
                parts.push(format!("target: {}", target));
            },
            EntryType::Directory => {}
        }

        if let Some(mtime) = inner.mtime() {
            let dt = DateTime::from_system_time(mtime, function_name)?;
            let formatted = dt.format("%Y-%m-%d %H:%M [%Z]", function_name)?;
            parts.push(format!("modified: {}", formatted));
        }

        let label = colorize("ArchiveEntry".to_string(), colors::BOLD_MAGENTA);

        ok_string(format!("{}<{}>", label, parts.join(", ")), luau)
    }
}


impl LuaUserDataMut for WrappedArchiveEntry {
    fn type_name<'a>() -> Cow<'a, str> {
        Cow::Borrowed("ArchiveEntry")
    }

    fn add_fields<F: LuaUserDataFieldsMut<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "ArchiveEntry");
        fields.add_field_method_get(
            "type",
            |luau, this: &WrappedArchiveEntry| -> LuaValueResult {
                ok_string(this.whatami().to_string(), luau)
            }
        );
    }
    fn add_methods<M: LuaUserDataMethodsMut<Self>>(methods: &mut M) {
        methods.add_method("try_file", WrappedArchiveEntry::entry_try_file);
        methods.add_method("expect_file", WrappedArchiveEntry::entry_expect_file);
        methods.add_method("try_directory", WrappedArchiveEntry::entry_try_directory);
        methods.add_method("expect_directory", WrappedArchiveEntry::entry_expect_directory);
        methods.add_method("try_symlink", WrappedArchiveEntry::entry_try_symlink);
        methods.add_method("expect_symlink", WrappedArchiveEntry::entry_expect_symlink);
        methods.add_method("size", WrappedArchiveEntry::file_get_size);
        methods.add_method("contents", WrappedArchiveEntry::file_get_contents);
        methods.add_method("compress", WrappedArchiveEntry::file_compress);
        methods.add_method("extract", WrappedArchiveEntry::file_extract);
        methods.add_method("path", WrappedArchiveEntry::entry_get_path);
        methods.add_method_mut("set_path", WrappedArchiveEntry::entry_set_path);
        methods.add_method("target", WrappedArchiveEntry::symlink_get_target);
        methods.add_method_mut("set_target", WrappedArchiveEntry::symlink_set_target);
        methods.add_method("display", WrappedArchiveEntry::entry_display);
        methods.add_method_mut("replace_contents", WrappedArchiveEntry::file_set_contents);
        methods.add_method("modified", WrappedArchiveEntry::entry_get_modified);
        methods.add_method_mut("set_modified", WrappedArchiveEntry::entry_set_modified);
        methods.add_method("unix_mode", WrappedArchiveEntry::entry_get_unix_mode);
        methods.add_method_mut("set_unix_mode", WrappedArchiveEntry::entry_set_unix_mode);
    }
}
impl BorrowableMut for WrappedArchiveEntry {
}

/// archive crate expects paths to be String in the end anyway
fn expect_stringy_path(value: Option<LuaValue>, function_name: &'static str) -> LuaResult<String> {
    let s = match value {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{}: expected path to be a string, got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected path to be a string, got {:?}", function_name, other);
        }
    };
    Ok(s)
}

fn entry_create_file(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "entry.create.from(path: string, contents: string | buffer)";

    let path = expect_stringy_path(multivalue.pop_front(), function_name)?;

    let contents = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.as_bytes().to_owned(),
        Some(LuaValue::Buffer(buffy)) => buffy.to_vec(),
        Some(LuaNil) | None => {
            return wrap_err!("{}: expected contents to be a string or buffer, got nothing or nil", function_name);
        }
        Some(other) => {
            return wrap_err!("{}: expected contents to be a string or buffer, got {:?}", function_name, other);
        }
    };

    WrappedArchiveEntry::file(path, contents).into_userdata(luau)
}

fn entry_create_directory(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "entry.create.directory(path: string)";

    let path = expect_stringy_path(multivalue.pop_front(), function_name)?;

    WrappedArchiveEntry::directory(path).into_userdata(luau)
}

fn entry_create_symlink(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "entry.create.symlink(path: string, target: string)";

    let path = expect_stringy_path(multivalue.pop_front(), function_name)?;

    let target = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{}: expected target to be a string, got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected target to be a string, got {:?}", function_name, other);
        }
    };

    WrappedArchiveEntry::symlink(path, target).into_userdata(luau)
}

fn entry_read(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "entry.read(path: Pathlike, as: (Pathlike | number)?)";

    let path = super::expect_pathlike(multivalue.pop_front(), function_name)?;
    let out_path = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => {
            Some(ArchiveOutputPath::As(PathBuf::from(s.to_string_lossy())))
        },
        Some(LuaValue::Integer(i)) => {
            Some(ArchiveOutputPath::KeepChildren(int_to_usize(i, function_name, "as")?))
        },
        Some(LuaValue::Number(f)) => {
            Some(ArchiveOutputPath::KeepChildren(float_to_usize(f, function_name, "as")?))
        },
        Some(LuaNil) | None => None,
        Some(other) => {
            return wrap_err!("{}: expected as to be a string or number, got: {:?}", function_name, other);
        }
    };

    WrappedArchiveEntry::from_path_on_disk(Path::new(&path), out_path)?.into_value(luau)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_value("create", TableBuilder::create(luau)?
            .with_function("file", entry_create_file)?
            .with_function("directory", entry_create_directory)?
            .with_function("symlink", entry_create_symlink)?
            .build_readonly()?
        )?
        .with_function("read", entry_read)?
        .build_readonly()
}

pub(super) fn archive_format_from_str(f: &str, function_name: &'static str) -> LuaResult<ArchiveFormat> {
    let formats = [
        ("zip", ArchiveFormat::Zip),
        ("tar", ArchiveFormat::Tar),
        ("ar", ArchiveFormat::Ar),
        ("deb", ArchiveFormat::Deb),
        ("targz", ArchiveFormat::TarGz),
        ("tar.gz", ArchiveFormat::TarGz),
        ("tarbz2", ArchiveFormat::TarBz2),
        ("tar.bz2", ArchiveFormat::TarBz2),
        ("tarxz", ArchiveFormat::TarXz),
        ("tar.xz", ArchiveFormat::TarXz),
        ("tarzst", ArchiveFormat::TarZst),
        ("tar.zst", ArchiveFormat::TarZst),
        ("tarlz4", ArchiveFormat::TarLz4),
        ("tar.lz4", ArchiveFormat::TarLz4),
        ("gz", ArchiveFormat::Gz),
        ("bz2", ArchiveFormat::Bz2),
        ("xz", ArchiveFormat::Xz),
        ("lz4", ArchiveFormat::Lz4),
        ("zst", ArchiveFormat::Zst),
        ("sevenz", ArchiveFormat::SevenZ),
        ("7z", ArchiveFormat::SevenZ),
    ];

    for (tag, format) in formats {
        if f.eq_ignore_ascii_case(tag) {
            return Ok(format);
        }
    }

    wrap_err!("{}: format '{}' is not one of the archive/single-file formats that we expected", function_name, f)
}