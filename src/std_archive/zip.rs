use mluau::prelude::*;
use crate::{prelude::*, std_fs::validate_path};

use archive::ArchiveFormat;
use super::Archive;
use super::options::ArchiveOptions;

fn zip_extract(_luau: &Lua, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
    let function_name = "zip.extract(path: Pathlike, destination: Pathlike, options: ArchiveOptions?)";

    super::extract::from_file(&mut multivalue, ArchiveFormat::Zip, function_name)?;

    Ok(())
}

fn zip_read(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "zip.readfile(path: Pathlike, options: ArchiveOptions?)";

    let path = match multivalue.pop_front() {
        Some(LuaValue::String(path)) => validate_path(&path, function_name)?,
        Some(LuaNil) | None => {
            return wrap_err!("{}: called without required argument 'path'", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected path to be a string or pathlike, got: {:?}", function_name, other);
        }
    };

    let contents = match std::fs::read(&path) {
        Ok(contents) => contents,
        Err(err) => {
            return wrap_err!("{}: unable to read archive due to err: {}", function_name, err);
        }
    };
    let options = multivalue.pop_front().unwrap_or(LuaNil);
    let options = ArchiveOptions::from_value(options, function_name)?;

    let entries = super::extract::contents(
        contents,
        &path,
        &options,
        ArchiveFormat::Zip,
        function_name
    )?;

    Archive {
        entries
    }.into_userdata(luau)
}

fn zip_writefile(_luau: &Lua, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
    let function_name = "zip.writefile(path: Pathlike, archive: Archive)";

    let path = match multivalue.pop_front() {
        Some(LuaValue::String(path)) => validate_path(&path, function_name)?,
        Some(LuaNil) | None => {
            return wrap_err!("{}: called without required argument 'path'", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected path to be a string or pathlike, got: {:?}", function_name, other);
        }
    };

    let archive = match multivalue.pop_front() {
        Some(LuaValue::UserData(ud)) => {
            Archive::expect_borrowed(ud, "archive", function_name)?
        },
        Some(LuaNil) | None => {
            return wrap_err!("{}: called without required argument 'archive'", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected archive to be an Archive from any @std/archive library, got: {:?}", function_name, other);
        }
    };

    let options = multivalue.pop_front().unwrap_or(LuaNil);
    let options = ArchiveOptions::from_value(options, function_name)?;
    
    super::extract::write_to_disk(&archive.entries, path, options, ArchiveFormat::Zip, function_name)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function_and_signature("extract", zip_extract, signatures::STD_ARCHIVE_ZIP_EXTRACT)?
        .with_function_and_signature("readfile", zip_read, signatures::STD_ARCHIVE_ZIP_READFILE)?
        .with_function_and_signature("writefile", zip_writefile, signatures::STD_ARCHIVE_ZIP_WRITEFILE)?
        .build_readonly()
}