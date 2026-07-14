use archive::ArchiveEntry;
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