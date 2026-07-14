use mluau::prelude::*;
use crate::prelude::*;

pub mod extract;
pub mod zip;
pub mod options;
mod entry;

use archive::{ArchiveEntry, ArchiveFormat};

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
            "read",
            |luau, this, value: LuaValue| -> LuaValueResult {
                let function_name = "Archive:read(path_or_index: string | number)";
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

                if let Some(entry) = this.entries.get(index) 
                    && let Some(data) = entry.data()
                {
                    return ok_string(data, luau);
                }

                Ok(LuaNil)
            }
        );
        methods.add_method_mut(
            "add",
            |_luau, this, multivalue: LuaMultiValue| -> LuaEmptyResult {
                let function_name = "Archive:add(entry: ArchiveEntry, ...ArchiveEntry)";
                for (index, entry) in multivalue.iter().enumerate() {
                    // let entry = entry::from_value(value, function_name);
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

                let format = if format.eq_ignore_ascii_case("zip") {
                    ArchiveFormat::Zip
                } else {
                    todo!()
                };

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
    }
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_value("zip", zip::create(luau)?)?
        .build_readonly()
}