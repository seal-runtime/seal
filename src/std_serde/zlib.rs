use core::slice::SlicePattern;

use crate::prelude::*;
use mluau::prelude::*;

use std::io::{Read, Write, Cursor};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;

fn zlib_compress(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "zlib.compress(data: string | buffer, level: \"Default\" | \"Fast\" | \"Best\"?)";
    
    let data = match multivalue.pop_front() {
        Some(LuaValue::Buffer(buffy)) => {
            buffy.to_vec()
        },
        Some(LuaValue::String(s)) => {
            s.as_bytes().to_owned()
        },
        Some(LuaNil) | None => {
            return wrap_err!("{} called without required argument 'data'", function_name)
        },
        Some(other) => {
            return wrap_err!("{}: expected data to be string or buffer, got: {:?}", function_name, other);
        }
    };

    let compression_level = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => {
            match s.as_bytes().as_slice() {
                b"Default" => Compression::default(),
                b"Fast" => Compression::fast(),
                b"Best" => Compression::best(),
                _ => {
                    return wrap_err!("{}: unexpected compression level (got {})", function_name, s.display());
                }
            }
        },
        Some(LuaNil) | None => Compression::default(),
        Some(other) => {
            return wrap_err!("{}: expected compression level to be \"Default\", \"Fast\", \"Best\", or nil, got: {:?}", function_name, other);
        }
    };

    let mut encoder = ZlibEncoder::new(Vec::new(), compression_level);
    if let Err(err) = encoder.write_all(&data) {
        return wrap_err!("{}: unable to write to zlib encoder: {}", function_name, err);
    }
    let bytes = match encoder.finish() {
        Ok(finished) => finished,
        Err(err) => {
            return wrap_err!("{}: unable to finish zlib encoder encoding: {}", function_name, err);
        }
    };

    ok_buffy(bytes, luau)
}

fn zlib_decompress(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "zlib.decompress(compressed: buffer)";

    let encoded = match value {
        LuaValue::Buffer(buffy) => {
            buffy.to_vec()
        },
        other => {
            return wrap_err!("{}: expected compressed data to be a buffer, got: {:?}", function_name, other);
        }
    };

    let mut decoder = ZlibDecoder::new(Cursor::new(encoded));
    
    let mut decompressed = Vec::new();
    if let Err(err) = decoder.read_to_end(&mut decompressed) {
        return wrap_err!("{}: unable to decompress due to err: {}", function_name, err);
    }

    ok_buffy(decompressed, luau)    
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("compress", zlib_compress)?
        .with_function("decompress", zlib_decompress)?
        .build_readonly()
}