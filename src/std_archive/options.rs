use mluau::prelude::*;
use crate::prelude::*;

use crate::std_fs::file_size::{FileSize, GIGABYTE, MEGABYTE};

const MAX_FILE_SIZE: u64 = 500 * MEGABYTE;
const MAX_TOTAL_SIZE: u64 = 4 * GIGABYTE;

pub struct ArchiveOptions {
    pub allow_symlinks: bool,
    pub max_file_size: FileSize,
    pub max_total_size: FileSize,
    pub allow_unsafe_path_traversals: bool,
}
impl ArchiveOptions {
    pub fn extractor(&self) -> archive::ArchiveExtractor {
        archive::ArchiveExtractor::new()
            .with_max_file_size(self.max_file_size.as_bytes() as usize)
            .with_max_total_size(self.max_total_size.as_bytes() as usize)
            .allow_unsafe_path_traversals(self.allow_unsafe_path_traversals)
    }
    pub fn builder(&self) -> archive::ArchiveBuilder {
        archive::ArchiveBuilder::new()
            .allow_unsafe_path_traversals(self.allow_unsafe_path_traversals)
    }
    fn default() -> Self {
        Self {
            max_file_size: FileSize::from_bytes(MAX_FILE_SIZE),
            max_total_size: FileSize::from_bytes(MAX_TOTAL_SIZE),
            allow_symlinks: false,
            allow_unsafe_path_traversals: false,
        }
    }
    pub fn from_value(value: LuaValue, function_name: &'static str) -> LuaResult<Self> {
        let t = match value {
            LuaValue::Table(t) => t,
            LuaNil => {
                return Ok(Self::default());
            },
            other => {
                return wrap_err!("{}: expected options to be an ArchiveOptions table, got: {:?}", function_name, other);
            }
        };

        fn get_filesize_opt(t: &LuaTable, param: &'static str, function_name: &'static str) -> LuaResult<Option<FileSize>> {
            let got = match t.raw_get(param)? {
                LuaValue::UserData(ud) => {
                    Some(FileSize::expect_cloned_or_nil(ud, param, function_name)?)
                },
                LuaNil => None,
                other => {
                    return wrap_err!("{}: expected options.{} to be a FileSize (from @std/fs/filesize) or nil, got: {:?}", function_name, param, other);
                }
            };
            Ok(got)
        }

        fn get_opt_bool(t: &LuaTable, param: &'static str, function_name: &'static str) -> LuaResult<Option<bool>> {
            match t.raw_get(param)? {
                LuaValue::Boolean(b) => Ok(Some(b)),
                LuaValue::Nil => Ok(None),
                other => {
                    wrap_err!("{}: expected options.{} to be a boolean (default false), got: {:?}", function_name, param, other)
                }
            }
        }

        let max_file_size = get_filesize_opt(&t, "max_file_size", function_name)?
            .unwrap_or(FileSize::from_bytes(MAX_FILE_SIZE));
        let max_total_size = get_filesize_opt(&t, "max_total_size", function_name)?
            .unwrap_or(FileSize::from_bytes(MAX_TOTAL_SIZE));
        
        let allow_symlinks = get_opt_bool(&t, "allow_symlinks", function_name)?
            .unwrap_or(false);
        let allow_unsafe_path_traversals = get_opt_bool(&t, "allow_unsafe_path_traversals", function_name)?
            .unwrap_or(false);

        Ok(Self {
            max_file_size,
            max_total_size,
            allow_symlinks,
            allow_unsafe_path_traversals
        })
    }
}
