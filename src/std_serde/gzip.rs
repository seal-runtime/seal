use mluau::prelude::*;
use crate::prelude::*;
use archive::extractor::ArchiveExtractor;
use archive::builder::ArchiveBuilder;
use crate::std_fs::file_size::FileSize;

fn expect_content(value: Option<LuaValue>, function_name: &'static str) -> LuaResult<Vec<u8>> {
    let bytes = match value {
        Some(LuaValue::String(s)) => s.as_bytes().to_owned(),
        Some(LuaValue::Buffer(buffy)) => buffy.to_vec(),
        Some(LuaNil) | None => {
            return wrap_err!("{}: called without required argument 'content'; expected string or buffer, got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected content to be a string or buffer, got {:?}", function_name, other);
        }
    };
    Ok(bytes)
}

fn compress(contents: &[u8], function_name: &'static str) -> LuaResult<Vec<u8>> {
    let compressed = match ArchiveBuilder::new().build_single_file("data", contents, archive::ArchiveFormat::Gz) {
        Ok(contents) => contents,
        Err(err) => {
            return wrap_err!("{}: unable to compress data into gzip due to err: {}", function_name, err);
        }
    };
    Ok(compressed)
}

fn decompress(contents: &[u8], path: Option<&str>, max_size: Option<u64>, function_name: &'static str) -> LuaResult<Vec<u8>> {
    let mut extractor = ArchiveExtractor::new();
    if let Some(size) = max_size {
        extractor = extractor.with_max_file_size(size as usize);
    }
    let decompressed = match extractor.extract(contents, archive::ArchiveFormat::Gz) {
        Ok(entry) => entry[0].data().expect("this can't be empty").to_owned(),
        Err(err) => {
            if let Some(path) = path {
                return wrap_err!("{}: unable to decompress gzip file at '{}' due to err: {}", function_name, path, err);
            } else {
                return wrap_err!("{}: unable to decompress gzip data due to err: {}", function_name, err);
            }
        }
    };
    Ok(decompressed)
}

fn gzip_compress(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "gzip.compress(content: string | buffer)";
    let content = expect_content(Some(value), function_name)?;
    let compressed = compress(&content, function_name)?;
    ok_buffy(compressed, luau)
}

fn gzip_decompress(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "gzip.decompress(content: string | buffer)";
    let content = expect_content(multivalue.pop_front(), function_name)?;
    let max_size = match multivalue.pop_front() {
        Some(LuaValue::UserData(ud)) => {
            let size = FileSize::expect_cloned_or_nil(ud, "max_size", function_name)?;
            Some(size.as_bytes())
        },
        Some(LuaNil) | None => None,
        Some(other) => {
            return wrap_err!("{}: expected max_size to be a FileSize from @std/fs/filesize or nil (defaults to 1 GB), got: {:?}", function_name, other);
        }
    };

    let decompressed = decompress(&content, None, max_size, function_name)?;
    ok_buffy(decompressed, luau)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function_and_signature("compress", gzip_compress, signatures::STD_SERDE_GZIP_COMPRESS)?
        .with_function_and_signature("decompress", gzip_decompress, signatures::STD_SERDE_GZIP_DECOMPRESS)?
        .build_readonly()
}