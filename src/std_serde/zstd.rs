use std::io::Cursor;

use crate::prelude::*;
use mluau::prelude::*;

struct ZstdOptions {
    checksum: bool,
    content_size: bool,
    dictid: bool,
    window_log: Option<u32>,
}

impl ZstdOptions {
    fn default() -> Self {
        Self {
            checksum: false,
            content_size: true,
            dictid: false,
            window_log: None,
        }
    }

    fn from_table(t: LuaTable, function_name: &'static str) -> LuaResult<Self> {
        let default = Self::default();

        let checksum = match t.raw_get("checksum")? {
            LuaValue::Boolean(b) => b,
            LuaNil => default.checksum,
            other => {
                return wrap_err!("{}: expected checksum to be a boolean or nil (defaults to false), got: {:?}", function_name, other);
            }
        };

        let content_size = match t.raw_get("content_size")? {
            LuaValue::Boolean(b) => b,
            LuaNil => default.content_size,
            other => {
                return wrap_err!("{}: expected content_size to be a boolean or nil (defaults to true), got: {:?}", function_name, other);
            }
        };

        let dictid = match t.raw_get("dictid")? {
            LuaValue::Boolean(b) => b,
            LuaNil => default.dictid,
            other => {
                return wrap_err!("{}: expected dictid to be a boolean or nil (defaults to false), got: {:?}", function_name, other);
            }
        };

        let window_log = match t.raw_get("window_log")? {
            LuaValue::Integer(i) if i >= 0 => Some(i as u32),
            LuaNil => default.window_log,
            other => {
                return wrap_err!("{}: expected window_log to be a non-negative integer or nil, got: {:?}", function_name, other);
            }
        };

        Ok(Self {
            checksum,
            content_size,
            dictid,
            window_log,
        })
    }
}


fn zstd_compress(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "zstd.compress(data: string | buffer, level: number, options: ZstdOptions?)";
    let bytes = match multivalue.pop_front() {
        Some(LuaValue::Buffer(buffy)) => {
            buffy.to_vec()
        },
        Some(LuaValue::String(s)) => {
            s.as_bytes().to_owned()
        },
        Some(LuaNil) | None => {
            return wrap_err!("{} was called without required argument 'data' (expected string | buffer)");
        },
        Some(other) => {
            return wrap_err!("{}: expected data to be string | buffer, got: {:?}", function_name, other);
        }
    };

    let level = match multivalue.pop_front() {
        Some(LuaValue::Number(f)) => f as i32,
        Some(LuaValue::Integer(i)) => i as i32,
        Some(LuaNil) | None => 0,
        Some(other) => {
            return wrap_err!("{}: expected level to be 0, 1, 2, 3, or nil/unspecified, got: {:?}", function_name, other);
        }
    };

    let zstd_options = match multivalue.pop_front() {
        Some(LuaValue::Table(t)) => Some(ZstdOptions::from_table(t, function_name)?),
        Some(LuaNil) | None => Some(ZstdOptions::default()),
        Some(other) => {
            return wrap_err!("{}: expected options to be a ZstdOptions table, got: {:?}", function_name, other);
        }
    };

    let compressed = match zstd_options {
        Some(options) => {
            use zstd::stream::Encoder;
            use std::io::Write;
            let mut encoder = match Encoder::new(Vec::new(), level) {
                Ok(enc) => enc,
                Err(err) => {
                    return wrap_err!("{}: unable to create zstd encoder: {}", function_name, err);
                }
            };

            if let Err(err) = encoder.include_checksum(options.checksum) {
                return wrap_err!("{}: can't set include_checksum: {}", function_name, err);
            };
            if let Err(err) = encoder.include_contentsize(options.content_size) {
                return wrap_err!("{}: can't set include_contentsize: {}", function_name, err);
            }
            if options.content_size {
                let pledged_size: u64 = match bytes.len().try_into() {
                    Ok(u) => u,
                    Err(err) => {
                        return wrap_err!("{}: can't set pledged_size because encoded length doesn't fit into u64: {}", function_name, err);
                    }
                };
                if let Err(err) = encoder.set_pledged_src_size(Some(pledged_size)) {
                    return wrap_err!("{}: can't set pledged_size (content_size set to true) due to err: {}", function_name, err);
                }
            }
            if let Err(err) = encoder.include_dictid(options.dictid) {
                return wrap_err!("{}: can't set include_dictid: {}", function_name, err);
            }

            if let Some(window_log) = options.window_log
                && let Err(err) = encoder.window_log(window_log) 
            {
                return wrap_err!("{}: invalid window_log value {}: {}", function_name, window_log, err);
            }

            if let Err(err) = encoder.write_all(&bytes) {
                return wrap_err!("{}: unable to write bytes into zstd encoder: {}", function_name, err);
            }

            match encoder.finish() {
                Ok(compressed) => compressed,
                Err(err) => {
                    return wrap_err!("{}: unable to finish zstd compression: {}", function_name, err);
                }
            }
        },
        None => match zstd::encode_all(Cursor::new(bytes), level) {
            Ok(compressed) => compressed,
            Err(err) => {
                return wrap_err!("{}: unable to compress bytes with default options: {}", function_name, err);
            }
        }
    };

    ok_buffy(compressed, luau)
}

fn zstd_decompress(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "zstd.decompress(compressed: buffer)";

    let bytes = match value {
        LuaValue::Buffer(buffy) => {
            buffy.to_vec()
        },
        other => {
            return wrap_err!("{}: expected 'compressed' to be a buffer, got: {:?}", function_name, other);
        }
    };

    let decompressed = match zstd::decode_all(Cursor::new(bytes)) {
        Ok(decompressed) => decompressed,
        Err(err) => {
            return wrap_err!("{}: unable to decompress bytes: {}", function_name, err);
        }
    };

    ok_buffy(decompressed, luau)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("compress", zstd_compress)?
        .with_function("decompress", zstd_decompress)?
        .build_readonly()
}