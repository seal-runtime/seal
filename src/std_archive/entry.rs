use archive::{ArchiveEntry, ArchiveFormat};
use mluau::prelude::*;
use crate::prelude::*;

enum EntryType {
    File,
    Directory,
    Symlink
}
const VALID_ENTRY_TYPES: &str = "\"File\", \"Directory\", or \"Symlink\"";
impl EntryType {
    fn from_value(value: LuaValue) -> LuaResult<Self> {
        let s = match value {
            LuaValue::String(s) => s.as_bytes().to_vec(),
            other => {
                return wrap_err!("expected entry.type to be a string ({}), got: {:?}", VALID_ENTRY_TYPES, other);
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
            return wrap_err!("expected entry.type to be {}, got: {}", VALID_ENTRY_TYPES, displayable);
        };
        Ok(t)
    }
}

/// stupid wrapper to make ArchiveEntry from luau tables
/// we omit function_name here because caller should not use ? to propagate errors from this function
pub(super) fn from_value(value: &LuaValue) -> LuaResult<ArchiveEntry> {
    let t = match value {
        LuaValue::Table(t) => t,
        other => {
            return wrap_err!("expected entry to be an ArchiveEntry table (type: {}), got: {:?}", VALID_ENTRY_TYPES, other);
        }
    };

    fn get_path(t: &LuaTable, key: &'static str) -> LuaResult<String> {
        match t.raw_get(key)? {
            LuaValue::String(s) => Ok(s.to_string_lossy()),
            other => {
                wrap_err!("expected entry.{} to be a string, got: {:?}", key, other)
            }
        }
    }

    let typ = EntryType::from_value(t.raw_get("type")?)?;
    let entry = match typ {
        EntryType::File => {
            let path = get_path(t, "path")?;

            let content = match t.raw_get("content")? {
                LuaValue::String(s) => s.as_bytes().to_vec(),
                LuaValue::Buffer(b) => b.to_vec(),
                other => {
                    return wrap_err!("expected entry.content to be a string or buffer, got: {:?}", other);
                }
            };
        
            ArchiveEntry::file(path, content)
        },
        EntryType::Directory => {
            let path = get_path(t, "path")?;
            ArchiveEntry::directory(path)
        },
        EntryType::Symlink => {
            let path = get_path(t, "path")?;
            let target = get_path(t, "target")?;
            ArchiveEntry::symlink(path, target)
        }
    };

    Ok(entry)
}

pub(super) fn to_table(entry: &ArchiveEntry, luau: &Lua) -> LuaResult<LuaTable> {
    match entry {
        ArchiveEntry::Directory { path } => {
            TableBuilder::create(luau)?
                .with_value("type", "Directory")?
                .with_value("path", path.to_owned())?
                .build()
        },
        ArchiveEntry::File { path, data } => {
            TableBuilder::create(luau)?
                .with_value("type", "File")?
                .with_value("path", path.to_owned())?
                .with_value("content", ok_string(data, luau)?)?
                .build()
        },
        ArchiveEntry::Symlink { path, target } => {
            TableBuilder::create(luau)?
                .with_value("type", "Symlink")?
                .with_value("path", path.to_owned())?
                .with_value("target", target.to_owned())?
                .build()
        }
    }
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