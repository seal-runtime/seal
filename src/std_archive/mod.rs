use mluau::prelude::*;
use crate::prelude::*;

use crate::std_io::format;

use std::fs;

pub mod extract;
pub mod options;
mod entry;
pub mod libraries;

use archive::{ArchiveEntry, ArchiveFormat};
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

struct Archive {
    entries: Vec<ArchiveEntry>,
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
    fn into_userdata(self, luau: &Lua) -> LuaValueResult {
        ok_userdata(self, luau)
    }
    fn from_value(value: Option<LuaValue>, function_name: &'static str) -> LuaResult<LuaUserDataRef<Self>> {
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
}
impl Borrowable for Archive {
    fn type_name() -> &'static str {
        "Archive"
    }
}
impl LuaUserData for Archive {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "Archive");
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "paths", 
            |luau, this, _: LuaValue| -> LuaValueResult {
                let paths: Vec<&str> = this
                    .entries
                    .iter()
                    .map(|entry| entry.path())
                    .collect();

                ok_table(luau.create_sequence_from(paths))
            }
        );
        methods.add_method(
            "get",
            |luau, this, value: LuaValue| -> LuaValueResult {
                let function_name = "Archive:get(path_or_index: string | number)";
                let which = PathOrIndex::from_value(value, function_name)?;
                let index = match which {
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
                    return ok_table(entry::to_table(entry, luau));
                }

                Ok(LuaNil)
            }
        );
        methods.add_method(
            "info",
            |luau, this, value: LuaValue| -> LuaValueResult {
                let function_name = "Archive:info(path_or_index: string | number)";
                let which = PathOrIndex::from_value(value, function_name)?;
                let index = match which {
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
                    return ok_table(entry::to_info_table(entry, luau));
                }

                Ok(LuaNil)
            }
        );
        methods.add_method_mut(
            "insert",
            |_luau, this, mut multivalue: LuaMultiValue| -> LuaEmptyResult {
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
                let entry = match entry::from_value(&entry) {
                    Ok(entry) => entry,
                    Err(err) => {
                        return wrap_err!("{}: unable to create entry for ArchiveEntry at index {} due to err: {}", function_name, index, err);
                    }
                };

                let entries = &mut this.entries;
                if index - 1 > entries.len() {
                    return wrap_err!("{}: index in archive is out of range (got {}, expected within 1 to {} inclusive)", function_name, index, entries.len());
                }

                // convert from luau 1 based to 0 based
                if index > 0 {
                    index -= 1;
                } else {
                    return wrap_err!("{}: index should be 1 or greater (got {})", function_name, index);
                }

                entries.insert(index, entry);
                Ok(())
            }
        );
        methods.add_method_mut(
            "append",
            |_luau, this, multivalue: LuaMultiValue| -> LuaEmptyResult {
                let function_name = "Archive:append(entry: ArchiveEntry, ...ArchiveEntry)";
                for (index, entry) in multivalue.iter().enumerate() {
                    let entry = match entry::from_value(entry) {
                        Ok(entry) => entry,
                        Err(err) => {
                            return wrap_err!("{}: cannot create ArchiveEntry from value passed at index {} due to err: {}", function_name, index, err);
                        }
                    };
                    this.entries.push(entry);
                }
                Ok(())
            }
        );
        methods.add_method_mut(
            "remove",
            |_luau, this, mut multivalue: LuaMultiValue| -> LuaEmptyResult {
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
                if index - 1 > len {
                    return wrap_err!("{}: index in archive is out of range (got {}, expected within 1 to {} inclusive)", function_name, index, len);
                }

                // convert from luau 1 based to 0 based
                if index > 0 {
                    index -= 1;
                } else {
                    return wrap_err!("{}: index should be 1 or greater (got {})", function_name, index);
                }

                let entries = &mut this.entries;
                entries.remove(index);

                Ok(())
            }
        );
        methods.add_method(
            "serialize",
            |luau, this, mut multivalue: LuaMultiValue| -> LuaValueResult {
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

                let builder = archive::ArchiveBuilder::new();
                match builder.build(&this.entries, format) {
                    Ok(contents) => {
                        ok_buffy(contents, luau)
                    },
                    Err(err) => {
                        wrap_err!("{} unable to serialize to {} due to err: {}", function_name, format.name(), err)
                    }
                }
            }
        );
        methods.add_meta_method(
            "__len",
            |luau, this, _: LuaValue| -> LuaValueResult {
                this.entries.len().into_lua(luau)
            }
        );
        methods.add_method(
            "display",
            |luau, this, mut multivalue: LuaMultiValue| -> LuaValueResult {
                let paths = this
                    .entries
                    .iter()
                    .map(|entry| {
                        match entry {
                            ArchiveEntry::Directory { path, .. } => format!("Directory: {}", path),
                            ArchiveEntry::File { path, .. } => format!("File: {}", path),
                            ArchiveEntry::Symlink { path, target, .. } => format!("Symlink: {} -> {}", path, target),
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
        )
    }
}

use crate::std_fs::{validate_path, entry::wrap_io_read_errors_empty};

fn expect_pathlike(value: Option<LuaValue>, function_name: &'static str) -> LuaResult<String> {
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

pub fn archive_extract(
    multivalue: &mut LuaMultiValue,
    format: ArchiveFormat,
    function_name: &'static str
) -> LuaEmptyResult {
    let path = match multivalue.pop_front() {
        Some(LuaValue::String(path)) => validate_path(&path, function_name)?,
        Some(LuaNil) | None => {
            return wrap_err!("{}: expected path to be a string or Pathlike, got nothing or nil", function_name);
        }
        Some(other) => {
            return wrap_err!("{}: expected path to be a string or Pathlike, got {:?}", function_name, other);
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

    let contents = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(err) => {
            return wrap_io_read_errors_empty(err, function_name, &path);
        }
    };

    let entries = extract::contents(
        contents,
        Some(&path),
        &options,
        format,
        function_name
    )?;

    extract::write_to_disk(
        &entries,
        destination,
        options,
        format,
        function_name
    )?;

    Ok(())
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
        entries
    }.into_userdata(luau)
}

pub fn archive_writefile(
    multivalue: &mut LuaMultiValue,
    format: ArchiveFormat,
    function_name: &'static str,
) -> LuaEmptyResult {
    let path = expect_pathlike(multivalue.pop_front(), function_name)?;
    let archive = Archive::from_value(multivalue.pop_front(), function_name)?;

    let options = multivalue.pop_front().unwrap_or(LuaNil);
    let options = ArchiveOptions::from_value(options, function_name)?;
    let bytes = match options.builder().build(&archive.entries, format) {
        Ok(builded) => builded,
        Err(err) => {
            return wrap_err!("{}: unable to compress archive due to err: {}", function_name, err);
        }
    };

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
        entries
    }.into_userdata(luau)
}

pub fn archive_create(luau: &Lua, _: LuaValue) -> LuaValueResult {
    Archive {
        entries: Vec::new(),
    }.into_userdata(luau)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_value("zip", libraries::Zip::create(luau)?)?
        .with_value("tar", libraries::Tar::create(luau)?)?
        .with_value("ar", libraries::Ar::create(luau)?)?
        .with_value("deb", libraries::Deb::create(luau)?)?
        .with_value("sevenz", libraries::Sevenz::create(luau)?)?
        .with_function_and_signature("create", archive_create, signatures::STD_ARCHIVE_CREATE)?
        .build_readonly()
}