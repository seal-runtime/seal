use mluau::prelude::*;
use crate::{std_fs::pathlib::normalize_path, *};
use std::{fs, io};

const RESERVED_ALIASES: [&str; 3] = ["@std", "@interop", "@internal"];

#[inline(always)]
fn is_reserved(path: &str) -> bool {
    RESERVED_ALIASES.iter().any(|alias| path.starts_with(alias))
}

pub fn require(luau: &Lua, path: LuaValue) -> LuaValueResult {
    let path = match path {
        LuaValue::String(path) => path.to_string_lossy(),
        other => {
            return wrap_err!("require expected a string path (like \"@std/json\" or \"./relative_file\"), got: {:#?}", other);
        }
    };

    if is_reserved(&path) {
        get_standard_library(luau, path)
    } else {
        let path = resolve_path(luau, path)?;
        // must use globals.get() due to safeenv
        let Ok(LuaValue::Table(require_cache)) = luau.globals().get("_REQUIRE_CACHE") else {
            return wrap_err!("require: you changed the type of or removed _REQUIRE_CACHE you goober why would you do that? do you want to seal the world burn?");
        };
        let cached_result: Option<LuaValue> = require_cache.raw_get(path.clone())?;

        if let Some(cached_result) = cached_result {
            Ok(cached_result)
        } else {
            let data = match fs::read_to_string(&path) {
                Ok(data) => data,
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::NotFound => {
                            return wrap_err!("require: no such file or directory for resolved path {}", path);
                        },
                        _other => {
                            return wrap_err!("require: error reading file: {}", err);
                        }
                    }
                }
            };

            let chunk = Chunk::Src(data);
            let result: LuaValue = luau.load(chunk).set_name(&path).eval()?;
            require_cache.raw_set(path.clone(), result)?;
            // this is pretty cursed but let's just read the data we just wrote to the cache to get a new LuaValue
            // that can be returned without breaking the borrow checker or cloning
            let result = require_cache.raw_get(path.to_owned())?;
            Ok(result)
        }
    }
}

fn get_standard_library(luau: &Lua, path: String) -> LuaValueResult {
    match path.as_str() {
        "@std/fs" => ok_table(std_fs::create(luau)),
        "@std/fs/path" => ok_table(std_fs::pathlib::create(luau)),
        "@std/fs/file" => ok_table(std_fs::filelib::create(luau)),
        "@std/fs/dir" => ok_table(std_fs::dirlib::create(luau)),

        "@std/env" => ok_table(std_env::create(luau)),
        "@std/env/vars" => ok_table(std_env::vars::create(luau)),

        "@std/err" => ok_table(std_err::create(luau)),

        "@std/io" => ok_table(std_io::create(luau)),
        "@std/io/input" => ok_table(std_io::input::create(luau)),
        "@std/io/output" => ok_table(std_io::output::create(luau)),
        "@std/io/colors" => ok_table(colors::create(luau)),
        "@std/io/clear" => ok_function(std_io::output::clear, luau),
        "@std/io/format" => ok_table(std_io::format::create(luau)),
        "@std/io/prompt" => ok_table(std_io::prompt::create(luau)),
        "@std/colors" => ok_table(colors::create(luau)),

        "@std/time" => ok_table(std_time::create(luau)),
        "@std/datetime" => ok_table(std_time::datetime::create(luau)),
        "@std/time/datetime" => ok_table(std_time::datetime::create(luau)),

        "@std/process" => ok_table(std_process::create(luau)),

        "@std/serde" => ok_table(std_serde::create(luau)),
        "@std/serde/base64" => ok_table(std_serde::base64::create(luau)),
        "@std/serde/toml" => ok_table(std_serde::toml::create(luau)),
        "@std/serde/yaml" => ok_table(std_serde::yaml::create(luau)),
        "@std/serde/json" => ok_table(std_json::create(luau)),
        "@std/serde/hex" => ok_table(std_serde::hex::create(luau)),
        "@std/serde/lz4" => ok_table(std_serde::lz4::create(luau)),
        "@std/serde/zstd" => ok_table(std_serde::zstd::create(luau)),
        "@std/serde/zlib" => ok_table(std_serde::zlib::create(luau)),
        "@std/serde/url" => ok_table(std_serde::url::create(luau)),
        "@std/json" => ok_table(std_json::create(luau)),

        "@std/net" => ok_table(std_net::create(luau)),
        "@std/net/http" => ok_table(std_net::http::create(luau)),
        "@std/net/http/server" => ok_table(std_net::serve::create(luau)),
        "@std/net/request" => ok_function(std_net::http::request, luau),
        "@std/net/websocket" => ok_table(std_net::websocket::create(luau)),

        "@std/crypt" => ok_table(std_crypt::create(luau)),
        "@std/crypt/aes" => ok_table(std_crypt::create_aes(luau)),
        "@std/crypt/rsa" => ok_table(std_crypt::create_rsa(luau)),
        "@std/crypt/hash" => ok_table(std_crypt::create_hash(luau)),
        "@std/crypt/password" => ok_table(std_crypt::create_password(luau)),

        "@internal/str" => ok_table(std_str_internal::create(luau)),
        "@std/str" => ok_table(load_std_str(luau)),

        "@std/semver" => ok_table(load_std_semver(luau)),

        "@std/thread" => ok_table(std_thread::create(luau)),

        "@std/luau" => ok_table(std_luau::create(luau)),

        "@std/args" => ok_table(std_args::create(luau)),

        "@std" => {
            ok_table(TableBuilder::create(luau)?
                .with_value("fs", std_fs::create(luau)?)?
                .with_value("str", load_std_str(luau)?)?
                .with_value("semver", load_std_semver(luau)?)?
                .with_value("env", std_env::create(luau)?)?
                .with_value("io", std_io::create(luau)?)?
                .with_value("colors", colors::create(luau)?)?
                .with_value("format", std_io::format::create(luau)?)?
                .with_value("time", std_time::create(luau)?)?
                .with_value("datetime", std_time::datetime::create(luau)?)?
                .with_value("process", std_process::create(luau)?)?
                .with_value("serde", std_serde::create(luau)?)?
                .with_value("json", std_json::create(luau)?)?
                .with_value("net", std_net::create(luau)?)?
                .with_value("crypt", std_crypt::create(luau)?)?
                .with_value("thread", std_thread::create(luau)?)?
                .with_value("luau", std_luau::create(luau)?)?
                .build_readonly()
            )
        },
        "@interop" => ok_table(interop::create(luau)),
        "@interop/standalone" => ok_table(interop::create_standalone(luau)),
        "@interop/mlua" => ok_table(interop::create_mlua(luau)),

        "@internal/setup" => ok_table(setup::create_internal(luau)),

        "@internal/reserved_aliases" => RESERVED_ALIASES.into_lua(luau),
        other => {
            wrap_err!("program required an unexpected standard library: {}", other)
        }
    }
}

const STD_STR_SRC: &str = include_str!("../std_str.luau");
fn load_std_str(luau: &Lua) -> LuaResult<LuaTable> {
    let chunk = Chunk::Src(STD_STR_SRC.to_owned());
    luau.load(chunk).set_name("std/str").eval::<LuaTable>()
}

const STD_SEMVER_SRC: &str = include_str!("../std_semver.luau");
fn load_std_semver(luau: &Lua) -> LuaResult<LuaTable> {
    let chunk = Chunk::Src(STD_SEMVER_SRC.to_owned());
    luau.load(chunk).set_name("std/semver").eval::<LuaTable>() // <<>> HACK
}

const RESOLVER_SRC: &str = include_str!("./resolver.luau");
pub fn get_resolver(luau: &Lua) -> LuaResult<LuaTable> {
    let chunk = Chunk::Src(RESOLVER_SRC.to_owned());
    let LuaValue::Table(resolver) = luau.load(chunk).eval()? else {
        panic!("require resolver didnt return table??");
    };
    Ok(resolver)
}

fn cached_resolver(luau: &Lua) -> LuaResult<LuaFunction> {
    let f = luau.named_registry_value::<Option<LuaFunction>>("require.resolver.resolve")?;
    if let Some(resolve) = f {
        Ok(resolve)
    } else {
        let chunk = Chunk::Src(RESOLVER_SRC.to_owned());
        let LuaValue::Table(resolver) = luau.load(chunk).eval()? else {
            panic!("require resolver didnt return table??");
        };
        let LuaValue::Function(resolve) = resolver.raw_get("resolve")? else {
            panic!("require resolver.resolve not a function??");
        };

        luau.set_named_registry_value("require.resolver.resolve", &resolve)?;

        Ok(resolve)
    }
}

fn resolve_path(luau: &Lua, path: String) -> LuaResult<String> {
    let resolve = cached_resolver(luau)?;
    match resolve.call::<LuaValue>(path.to_owned()) {
        Ok(LuaValue::Table(result_table)) => {
            if let LuaValue::String(path) = result_table.raw_get("path")? {
                Ok(path.to_string_lossy())
            } else if let LuaValue::String(err) = result_table.raw_get("err")? {
                wrap_err!("require: {}", err.to_string_lossy())
            } else {
                panic!("require: resolve() returned an unexpected table {:#?}", result_table);
            }
        },
        Ok(_other) => {
            panic!("require: resolve() returned something that isn't a string or err table; this shouldn't be possible");
        },
        Err(err) => {
            panic!("require: resolve() broke? this shouldn't happen; err: {}", err);
        }
    }
}

fn _get_require_cache(luau: &Lua) -> LuaResult<LuaTable> {
    // must use globals.get() due to safeenv
    let require_cache = match luau.globals().get("_REQUIRE_CACHE")? {
        LuaValue::Table(t) => t,
        other => {
            return wrap_err!("expected globals._REQUIRE_CACHE, got: {:?}", other);
        }
    };
    Ok(require_cache)
}

/// luau's require semantics classify meow.luau and meow/init.luau as the same thing
/// to get a reliable chunk name we want to get the absolute path and make sure we can figure out
/// if it's a dir w/ init.luau or not
pub fn get_chunk_name_for_module(path: &str, function_name: &'static str) -> LuaResult<Option<String>> {
    let path = match std::path::absolute(path) {
        Ok(path) => path,
        Err(err) => {
            return wrap_err!("{} can't figure out an absolute path for '{}' (we're trying to get a chunk_name). Can you verify that both file exists and your current directory exists (maybe another program removed your current directory, try reloading your editor or cd-ing out in back in)? err: {}", function_name, &path, err);
        }
    };

    if path.is_file() && path.exists() && let Some(path) = path.to_str() {
        Ok(Some(normalize_path(path)))
    } else if path.is_dir() {
        let possible_init_path = path.join("init.luau");
        if possible_init_path.exists() && let Some(init_path) = possible_init_path.to_str() {
            Ok(Some(normalize_path(init_path)))
        } else {
            wrap_err!("{}: directory at '{}' missing its init.luau, cannot assign it a chunk_name", function_name, path.display())
        }
    } else {
        Ok(None)
    }
}