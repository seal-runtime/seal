use mluau::prelude::*;
use crate::userdata::SealLock;
use crate::{prelude::*, std_archive::entry::EntryType};
use crate::userdata::{
    SealUserData as LuaUserDataMut,
    SealUserDataFields as LuaUserDataFieldsMut,
    SealUserDataMethods as LuaUserDataMethodsMut
};

use crate::std_io::format;
use std::borrow::Cow;
use std::fs;

pub mod extract;
pub mod options;
pub mod entry;
pub mod libraries;
pub mod compression_level;

use entry::{WrappedArchiveEntry, Entries};
use archive::{ArchiveEntry, ArchiveFormat, ArchiveBuilder};
use options::ArchiveOptions;

enum PathOrIndex {
    Path(String),
    Index(usize)
}
impl PathOrIndex {
    fn from_value(value: LuaValue, function_name: &'static str) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => {
                Ok(Self::Path(s.to_string_lossy()))
            },
            LuaValue::Number(f) => {
                let idx = float_to_usize(f, function_name, "index")?;
                Ok(Self::Index(idx))
            },
            LuaValue::Integer(i) => {
                let idx = int_to_usize(i, function_name, "index")?;
                Ok(Self::Index(idx))
            },
            other => {
                wrap_err!("{}: expected first argument to be a string (path) or number (index) in the archive, got: {:?}", function_name, other)
            }
        }
    }
}

#[derive(Clone)]
struct Archive {
    entries: Vec<WrappedArchiveEntry>,
}
impl Archive {
    fn index_of_path(&self, path: &str) -> Option<usize> {
        for (index, entry) in self.entries.iter().enumerate() {
            if entry.path() == path {
                return Some(index);
            }
        }
        None
    }
    fn from_value(value: Option<LuaValue>, function_name: &'static str) -> LuaResult<LuaUserDataRef<SealLock<Self>>> {
        match value {
            Some(LuaValue::UserData(ud)) => {
                Archive::expect_borrowed(ud, "archive", function_name)
            },
            Some(LuaNil) | None => {
                wrap_err!("{}: called without required argument 'archive'", function_name)
            },
            Some(other) => {
                wrap_err!("{}: expected archive to be an Archive from any @std/archive library, got: {:?}", function_name, other)
            }
        }
    }

    /// see comment in build_with_dummies
    fn dummy(t: EntryType) -> ArchiveEntry {
        match t {
            EntryType::Directory => ArchiveEntry::directory("dummy"),
            EntryType::File => ArchiveEntry::file("dummy", "very dummy"),
            EntryType::Symlink => ArchiveEntry::symlink("from dummy", "to dummy"),
        }
    }

    /// replace dummies with real entries when self.entries is full of dummies
    fn swap_out_dummies(&mut self, actual_entries: &mut Vec<ArchiveEntry>) {
        // now put the real entries back
        let mut index = self.entries.len() - 1;
        while let Some(actual_entry) = actual_entries.pop() {
            // panic safety = we know that moved.len == entries.len, therefore same indices should exist in both
            let entry_to_replace = &mut self.entries[index];
            let _dummy_to_drop = entry_to_replace.swap(actual_entry);
            index = index.saturating_sub(1);
        }
    }

    pub fn build_with_dummies(&mut self, builder: ArchiveBuilder, format: ArchiveFormat, function_name: &'static str) -> LuaResult<Vec<u8>> {
        // this is the worst idea i've ever had. instead of having to clone possibly huge ArchiveEntries
        // within the WrappedArchiveEntry, we just allocate a bunch of empty dummies, swap them in so the type
        // remains safe, and then swap them out after builder.build is done building the entries
        let mut moved_entries: Vec<ArchiveEntry> = Vec::with_capacity(self.entries.len());
        for entry in self.entries.iter_mut() {
            let what = entry.whatami();
            let actual_entry = entry.swap(Self::dummy(what));
            moved_entries.push(actual_entry);
        }

        let result = builder.build(&moved_entries, format);
        // ensure we put the real entries back before erroring/returning in case user pcalled this
        // (we don't want users to see dummies + lose state of archive)
        self.swap_out_dummies(&mut moved_entries);

        let serialized = match result {
            Ok(contents) => contents,
            Err(archive::ArchiveError::InvalidCompressionLevel(reason)) => {
                return wrap_err!("{}: invalid CompressionLevel for this kind of archive/single-file format; {}", function_name, reason);
            },
            Err(err) => {
                return wrap_err!("{} unable to serialize to {} due to err: {}", function_name, format.name(), err);
            }
        };
        
        Ok(serialized)
    }

    fn extract_with_dummies(
        &mut self,
        destination: String,
        options: ArchiveOptions,
        format: Option<ArchiveFormat>,
        function_name: &'static str
    ) -> LuaEmptyResult {
        // we need a &[ArchiveEntry] to pass to extract::write_to_disk, but the ArchiveEntries inside
        // this WrappedArchiveEntry are held under Rc<RefCell<ArchiveEntry>>
        // so to get around this we temporarily swap out all the archive entries in `this` with dummies
        let mut actual_entries = Vec::with_capacity(self.entries.len());
        for wrapped in self.entries.iter_mut() {
            let what = wrapped.whatami();
            let entry = wrapped.swap(Self::dummy(what));
            actual_entries.push(entry);
        }

        let result = extract::write_to_disk(&actual_entries, destination, options, format, function_name);
        // always get rid of dummies before returning/erroring so archive state doesn't get clobbered if extraction fails and this function was pcalled
        self.swap_out_dummies(&mut actual_entries);

        result
    }
}
impl BorrowableMut for Archive {}

// methods exposed to Luau
impl Archive {
    fn paths(luau: &Lua, this: &Archive, _: LuaValue) -> LuaValueResult {
        let paths: Vec<String> = this
            .entries
            .iter()
            .map(|entry| entry.path())
            .collect();

        ok_table(luau.create_sequence_from(paths))
    }

    fn get(luau: &Lua, this: &Archive, value: LuaValue) -> LuaValueResult {
        let function_name = "Archive:get(path_or_index: string | number)";
        let which = PathOrIndex::from_value(value, function_name)?;
        let index = match which {
            PathOrIndex::Index(0) => return Ok(LuaNil), // 0 is never a valid 1-based index
            PathOrIndex::Index(idx) => idx - 1, // luau tables use index 1 we use index 0
            PathOrIndex::Path(path) => {
                let idx = this.index_of_path(&path);
                let Some(idx) = idx else {
                    return Ok(LuaNil);
                };
                idx
            }
        };

        if let Some(entry) = this.entries.get(index) {
            return entry.clone().into_userdata(luau);
        }

        Ok(LuaNil)
    }

    fn expect(luau: &Lua, this: &Archive, value: LuaValue) -> LuaValueResult {
        let function_name = "Archive:expect(path_or_index: string | number)";
        let which = PathOrIndex::from_value(value, function_name)?;
        let index = match which {
            PathOrIndex::Index(0) => {
                return wrap_err!("{}: no entry at index 0 (indices are 1-based)", function_name);
            },
            PathOrIndex::Index(idx) => idx - 1, // luau tables use index 1 we use index 0
            PathOrIndex::Path(path) => {
                let Some(idx) = this.index_of_path(&path) else {
                    return wrap_err!("{}: no entry with path '{}' in this archive", function_name, path);
                };
                idx
            }
        };

        if let Some(entry) = this.entries.get(index) {
            return entry.clone().into_userdata(luau);
        }

        wrap_err!("{}: no entry at index {} (archive has {} entries)", function_name, index + 1, this.entries.len())
    }

    fn insert(_luau: &Lua, this: &mut Archive, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
        let function_name = "Archive:insert(index: number, entry: ArchiveEntry)";
        let mut index = match multivalue.pop_front() {
            Some(LuaValue::Integer(i)) => int_to_usize(i, function_name, "index")?,
            Some(LuaValue::Number(f)) => float_to_usize(f, function_name, "index")?,
            Some(LuaNil) | None => {
                return wrap_err!("{}: expected index to be a positive whole number, got nothing or nil", function_name);
            },
            Some(other) => {
                return wrap_err!("{}: expected index to be a number, got: {:?}", function_name, other);
            }
        };

        let Some(entry) = multivalue.pop_front() else {
            return wrap_err!("{}: called without required argument entry (expected ArchiveEntry table)", function_name);
        };
        let entry = match entry {
            LuaValue::UserData(ud) => {
                match WrappedArchiveEntry::cloned(ud) {
                    Ok(entry) => entry,
                    Err(typ) => {
                        return wrap_err!("{}: expected passed entry to be an ArchiveEntry from @std/archive/entry, got the wrong type of userdata: {}", function_name, typ);
                    }
                }
            },
            other => {
                return wrap_err!("{}: expected entry to be an ArchiveEntry userdata from @std/archive/entry, got: {:?}", function_name, other);
            }
        };

        let entries = &mut this.entries;
        let len = entries.len();
        if index == 0 {
            return wrap_err!("{}: index should be 1 or greater (got {})", function_name, index);
        }
        if index > len + 1 {
            return wrap_err!("{}: index in archive is out of range (got {}, expected within 1 to {} inclusive)", function_name, index, len + 1);
        }

        // convert from luau 1 based to 0 based
        index -= 1;

        entries.insert(index, entry);

        Ok(())
    }

    fn append(_luau: &Lua, this: &mut Archive, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
        let function_name = "Archive:append(entry: ArchiveEntry, ...ArchiveEntry)";
        if multivalue.is_empty() {
            return wrap_err!("{}: expected at least one ArchiveEntry entry from @std/archive/entry (but 0 were passed)", function_name);
        }
        let mut index = 0;
        while let Some(entry) = multivalue.pop_front() {
            index += 1;
            let entries = match entry {
                LuaValue::UserData(ud) => Entries::from_userdata(ud, index, function_name)?,
                LuaValue::Table(t) => Entries::from_table(t, index, function_name)?,
                other => {
                    return wrap_err!("{}: expected argument at index {} to be an ArchiveEntry userdata or an array-like table of them, got: {:?}", function_name, index, other);
                }
            };
            match entries {
                Entries::Many(entries) => this.entries.extend(entries),
                Entries::One(entry) => this.entries.push(entry),
            }
        }
        Ok(())
    }

    fn remove(_luau: &Lua, this: &mut Archive, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
        let function_name = "Archive:remove(index: number)";
        let mut index = match multivalue.pop_front() {
            Some(LuaValue::Integer(i)) => int_to_usize(i, function_name, "index")?,
            Some(LuaValue::Number(f)) => float_to_usize(f, function_name, "index")?,
            Some(LuaNil) | None => {
                return wrap_err!("{}: expected index to be a positive whole number, got nothing or nil", function_name);
            },
            Some(other) => {
                return wrap_err!("{}: expected index to be a number, got: {:?}", function_name, other);
            }
        };

        let len = this.entries.len();
        if index == 0 {
            return wrap_err!("{}: index should be 1 or greater (got {})", function_name, index);
        }
        if index > len {
            return wrap_err!("{}: index in archive is out of range (got {}, expected within 1 to {} inclusive)", function_name, index, len);
        }

        // convert from luau 1 based to 0 based
        index -= 1;

        let entries = &mut this.entries;
        entries.remove(index);

        Ok(())
    }

    fn serialize(luau: &Lua, this: &mut Archive, mut multivalue: LuaMultiValue) -> LuaValueResult {
        let function_name = "Archive:serialize(format: ArchiveFormat, options: ArchiveOptions?)";
        let format = match multivalue.pop_front() {
            Some(LuaValue::String(s)) => {
                s.to_string_lossy()
            },
            Some(LuaNil) | None => {
                return wrap_err!("{}: expected a format, got nothing or nil", function_name);
            }
            Some(other) => {
                return wrap_err!("{}: unexpected format, got: {:?}", function_name, other);
            },
        };

        let format = entry::archive_format_from_str(&format, function_name)?;

        let options = multivalue.pop_front().unwrap_or(LuaNil);
        let options = ArchiveOptions::from_value(options, function_name)?;

        let serialized = this.build_with_dummies(options.builder(), format, function_name)?;

        ok_buffy(serialized, luau)
    }

    fn extract(_luau: &Lua, this: &mut Archive, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
        let function_name = "Archive:extract(output_path: Pathlike | string, options: ArchiveOptions?)";

        let destination = expect_pathlike(multivalue.pop_front(), function_name)?;
        let value = multivalue.pop_front().unwrap_or(LuaNil);
        let options = ArchiveOptions::from_value(value, function_name)?;

        this.extract_with_dummies(destination, options, None, function_name)
    }

    fn len(luau: &Lua, this: &Archive, _: LuaValue) -> LuaValueResult {
        this.entries.len().into_lua(luau)
    }

    fn iter(luau: &Lua, this: &Archive, _: LuaValue) -> LuaValueResult {
        // entries are Rc<RefCell<ArchiveEntry>> under the hood, so cloning this Vec is cheap
        let entries = this.entries.clone();
        let index = std::cell::Cell::new(0usize);

        let iterator = luau.create_function(move |luau, _: LuaMultiValue| -> LuaResult<LuaMultiValue> {
            let i = index.get();
            let Some(entry) = entries.get(i) else {
                return Ok(LuaMultiValue::from_vec(vec![LuaNil]));
            };
            index.set(i + 1);

            Ok(LuaMultiValue::from_vec(vec![
                ((i + 1) as i64).into_lua(luau)?,
                entry.clone().into_userdata(luau)?,
            ]))
        })?;

        iterator.into_lua(luau)
    }

    fn display(luau: &Lua, this: &Archive, mut multivalue: LuaMultiValue) -> LuaValueResult {
        let paths = this
            .entries
            .iter()
            .map(|entry| {
                let what = entry.whatami();
                let path = entry.path();
                match what {
                    EntryType::Directory => {
                        format!("Directory ({})", path)
                    },
                    EntryType::File => {
                        format!("File ({})", path)
                    },
                    EntryType::Symlink => {
                        let target = entry.symlink_target().expect("symlink must have symlink target");
                        format!("Symlink ({} -> {})", path, target)
                    }
                }
            });

        let paths = luau.create_sequence_from(paths)?;

        multivalue.push_front(paths.into_lua(luau)?);
        multivalue.swap(1, 2); // move depth behind format options

        format!(
            "Archive {}",
            format::pretty(luau, multivalue)?,
        ).into_lua(luau)
    }
}

impl LuaUserDataMut for Archive {
    fn type_name<'a>() -> Cow<'a, str> {
        Cow::Borrowed("Archive")
    }
    fn add_fields<F: LuaUserDataFieldsMut<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "Archive");
    }
    fn add_methods<M: LuaUserDataMethodsMut<Self>>(methods: &mut M) {
        methods.add_method("paths", Archive::paths);
        methods.add_method("get", Archive::get);
        methods.add_method("expect", Archive::expect);
        methods.add_method_mut("insert", Archive::insert);
        methods.add_method_mut("append", Archive::append);
        methods.add_method_mut("remove", Archive::remove);
        methods.add_method_mut("serialize", Archive::serialize);
        methods.add_method_mut("extract", Archive::extract);
        methods.add_meta_method("__len", Archive::len);
        methods.add_meta_method("__iter", Archive::iter);
        methods.add_method("display", Archive::display);
    }
}

use crate::std_fs::{validate_path, entry::wrap_io_read_errors_empty};

pub fn expect_pathlike(value: Option<LuaValue>, function_name: &'static str) -> LuaResult<String> {
    match value {
        Some(LuaValue::String(path)) => validate_path(&path, function_name),
        Some(LuaNil) | None => {
            wrap_err!("{}: called without required argument 'path'", function_name)
        },
        Some(other) => {
            wrap_err!("{}: expected path to be a string or pathlike, got: {:?}", function_name, other)
        }
    }
}

/// fast case of archive_extract; user provided path on disk so we can stream it 
/// directly to output file without materializing ArchiveEntry instances in memory
fn extract_streaming(
    path: String,
    destination: String,
    options: ArchiveOptions,
    format: ArchiveFormat,
    function_name: &'static str
) -> LuaEmptyResult {
    let contents = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(err) => {
            return wrap_io_read_errors_empty(err, function_name, &path);
        }
    };

    extract::stream_to_disk(
        &contents,
        Some(&path),
        destination,
        &options,
        format,
        function_name
    )?;

    Ok(())
}

enum PathOrArchive {
    Path(String),
    Archive(Archive),
}

pub fn archive_extract(
    multivalue: &mut LuaMultiValue,
    format: ArchiveFormat,
    function_name: &'static str
) -> LuaEmptyResult {
    let path_or_archive = match multivalue.pop_front() {
        Some(LuaValue::String(path)) => PathOrArchive::Path(validate_path(&path, function_name)?),
        Some(LuaValue::UserData(ud)) => {
            let arch = Archive::expect_cloned(ud, "path or archive", function_name)?;
            PathOrArchive::Archive(arch)
        }
        Some(LuaNil) | None => {
            return wrap_err!("{}: expected first argument to be a path (string or Pathlike) or Archive instance; got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected first argument to be a path (string or Pathlike) or Archive instance; got {:?}", function_name, other);
        }
    };

    let destination = match multivalue.pop_front() {
        Some(LuaValue::String(path)) => validate_path(&path, function_name)?,
        Some(LuaNil) | None => {
            return wrap_err!("{}: expected destination to be a string or Pathlike, got nothing or nil", function_name);
        }
        Some(other) => {
            return wrap_err!("{}: expected destination to be a string or Pathlike, got {:?}", function_name, other);
        }
    };

    let options = multivalue.pop_front().unwrap_or(LuaNil);
    let options = ArchiveOptions::from_value(options, function_name)?;

    match path_or_archive {
        PathOrArchive::Path(path) => {
            extract_streaming(path, destination, options, format, function_name)
        },
        PathOrArchive::Archive(mut archive) => {
            archive.extract_with_dummies(destination, options, Some(format), function_name)
        }
    }

    
}

pub fn archive_readfile(
    luau: &Lua,
    multivalue: &mut LuaMultiValue,
    format: ArchiveFormat,
    function_name: &'static str,
) -> LuaValueResult {
    let path = expect_pathlike(multivalue.pop_front(), function_name)?;

    let contents = match std::fs::read(&path) {
        Ok(contents) => contents,
        Err(err) => {
            return wrap_err!("{}: unable to read archive due to err: {}", function_name, err);
        }
    };

    let options = multivalue.pop_front().unwrap_or(LuaNil);
    let options = ArchiveOptions::from_value(options, function_name)?;

    let entries = extract::contents(
        contents,
        Some(&path),
        &options,
        format,
        function_name
    )?;

    Archive {
        entries: WrappedArchiveEntry::from_entries(entries)
    }.into_userdata(luau)
}

pub fn archive_writefile(
    multivalue: &mut LuaMultiValue,
    format: ArchiveFormat,
    function_name: &'static str,
) -> LuaEmptyResult {
    let path = expect_pathlike(multivalue.pop_front(), function_name)?;
    let archive = Archive::from_value(multivalue.pop_front(), function_name)?;
    // cloning the archive sucks but not completely because archive contains a Vec of Rc pointers, cloning it
    // doesn't mean we clone all the data inside each ArchiveEntry
    let mut archive = match archive.0.try_borrow_mut() {
        Ok(a) => a.clone(),
        Err(_) => return wrap_err!("cannot borrow Archive while its being mutated!")
    };

    let options = multivalue.pop_front().unwrap_or(LuaNil);
    let options = ArchiveOptions::from_value(options, function_name)?;

    let bytes = archive.build_with_dummies(options.builder(), format, function_name)?;

    if let Err(err) = fs::write(&path, bytes) {
        return wrap_io_read_errors_empty(err, function_name, path);
    }
    
    Ok(())
}

pub fn archive_load(
    luau: &Lua,
    multivalue: &mut LuaMultiValue,
    format: ArchiveFormat,
    function_name: &'static str,
) -> LuaValueResult {
    let contents = match multivalue.pop_front() {
        Some(LuaValue::Buffer(buffy)) => buffy.to_vec(),
        Some(LuaValue::String(s)) => s.as_bytes().to_owned(),
        Some(LuaNil) | None => {
            return wrap_err!("{}: called without required argument 'bytes' (expected buffer, got nothing or nil)", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected bytes to be a buffer, got: {:?}", function_name, other);
        }
    };

    let options = multivalue.pop_front().unwrap_or(LuaNil);
    let options = ArchiveOptions::from_value(options, function_name)?;

    let entries = extract::contents(
        contents,
        None,
        &options,
        format,
        function_name
    )?;

    Archive {
        entries: WrappedArchiveEntry::from_entries(entries)
    }.into_userdata(luau)
}

pub fn archive_create(luau: &Lua, _: LuaValue) -> LuaValueResult {
    Archive {
        entries: Vec::new(),
    }.into_userdata(luau)
}

/// unlike per-format `<lib>.load`, this takes the format by name, including the single-file
/// formats (`Gz`, `Bz2`, `Xz`, `Lz4`, `Zst`) that otherwise have no library of their own.
fn archive_load_any(
    luau: &Lua,
    mut multivalue: LuaMultiValue,
) -> LuaValueResult {
    let function_name = "archive.load(bytes: buffer, format: ArchiveFormat, options: ArchiveOptions?)";

    let contents = match multivalue.pop_front() {
        Some(LuaValue::Buffer(buffy)) => buffy.to_vec(),
        Some(LuaValue::String(s)) => s.as_bytes().to_owned(),
        Some(LuaNil) | None => {
            return wrap_err!("{}: called without required argument 'bytes' (expected buffer, got nothing or nil)", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected bytes to be a buffer, got: {:?}", function_name, other);
        }
    };

    let format = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => entry::archive_format_from_str(&s.to_string_lossy(), function_name)?,
        Some(LuaNil) | None => {
            return wrap_err!("{}: called without required argument 'format' (expected an ArchiveFormat string)", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected format to be an ArchiveFormat string, got: {:?}", function_name, other);
        }
    };

    let options = multivalue.pop_front().unwrap_or(LuaNil);
    let options = ArchiveOptions::from_value(options, function_name)?;

    let entries = extract::contents(
        contents,
        None,
        &options,
        format,
        function_name
    )?;

    Archive {
        entries: WrappedArchiveEntry::from_entries(entries)
    }.into_userdata(luau)
}

/// unlike per-format `<lib>.extract`, this takes the format by name, including the single-file
/// formats (`Gz`, `Bz2`, `Xz`, `Lz4`, `Zst`) that otherwise have no library of their own.
fn archive_extract_any(_luau: &Lua, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
    let function_name = "archive.extract(path_or_archive: string | Archive, destination: string, format: ArchiveFormat, options: ArchiveOptions?)";

    let path_or_archive = match multivalue.pop_front() {
        Some(LuaValue::String(path)) => PathOrArchive::Path(validate_path(&path, function_name)?),
        Some(LuaValue::UserData(ud)) => {
            let arch = Archive::expect_cloned(ud, "path or archive", function_name)?;
            PathOrArchive::Archive(arch)
        }
        Some(LuaNil) | None => {
            return wrap_err!("{}: expected first argument to be a path (string or Pathlike) or Archive instance; got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected first argument to be a path (string or Pathlike) or Archive instance; got {:?}", function_name, other);
        }
    };

    let destination = match multivalue.pop_front() {
        Some(LuaValue::String(path)) => validate_path(&path, function_name)?,
        Some(LuaNil) | None => {
            return wrap_err!("{}: expected destination to be a string or Pathlike, got nothing or nil", function_name);
        }
        Some(other) => {
            return wrap_err!("{}: expected destination to be a string or Pathlike, got {:?}", function_name, other);
        }
    };

    let format = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => entry::archive_format_from_str(&s.to_string_lossy(), function_name)?,
        Some(LuaNil) | None => {
            return wrap_err!("{}: called without required argument 'format' (expected an ArchiveFormat string)", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected format to be an ArchiveFormat string, got: {:?}", function_name, other);
        }
    };

    let options = multivalue.pop_front().unwrap_or(LuaNil);
    let options = ArchiveOptions::from_value(options, function_name)?;

    match path_or_archive {
        PathOrArchive::Path(path) => {
            extract_streaming(path, destination, options, format, function_name)
        },
        PathOrArchive::Archive(mut archive) => {
            archive.extract_with_dummies(destination, options, Some(format), function_name)
        }
    }
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_value("zip", libraries::Zip::create(luau)?)?
        .with_value("tar", libraries::Tar::create(luau)?)?
        .with_value("ar", libraries::Ar::create(luau)?)?
        .with_value("deb", libraries::Deb::create(luau)?)?
        .with_value("entry", entry::create(luau)?)?
        .with_value("sevenz", libraries::Sevenz::create(luau)?)?
        .with_function_and_signature("create", archive_create, signatures::STD_ARCHIVE_CREATE)?
        .with_function_and_signature("load", archive_load_any, signatures::STD_ARCHIVE_LOAD)?
        .with_function_and_signature("extract", archive_extract_any, signatures::STD_ARCHIVE_EXTRACT)?
        .build_readonly()
}