use mluau::{AsChunk, prelude::*};
use std::borrow::Cow;

pub const MAX_TABLE_SIZE: usize = 134_217_728;
pub use crate::{std_io::colors as colors, wrap_err, table_helpers::TableBuilder};

/// Chunk of Luau code, either sourcecode (valid utf8) or bytecode (never valid utf8)
/// this is needed because passing invalid bytecode to luau.load causes segfaults at runtime
/// If we apply any transformations on code before luau.load we need to ensure only src
/// gets transformed and not bytecode. This newtype wrapper implements `AsChunk` to handle any such
/// transformations in one place.
pub enum Chunk {
    Src(String),
    Bytecode(Vec<u8>)
}
impl AsChunk for Chunk {
    fn source<'a>(&self) -> std::io::Result<std::borrow::Cow<'a, [u8]>>
    where
        Self: 'a 
    {
        Ok(match self {
            Chunk::Src(src) => {
                let src = temp_transform_luau_src(src); // <<>> HACK
                Cow::Owned(src.as_bytes().to_owned())
            },
            Chunk::Bytecode(bytecode) => Cow::Owned(bytecode.to_owned()),
        })
    }
}

pub type LuaValueResult = LuaResult<LuaValue>;
pub type LuaEmptyResult = LuaResult<()>;
pub type LuaMultiResult = LuaResult<LuaMultiValue>;

// wraps returns of stdlib::create functions with Ok(LuaValue::Table(t))
pub fn ok_table(t: LuaResult<LuaTable>) -> LuaValueResult {
    Ok(LuaValue::Table(t?))
}

pub fn ok_function(f: fn(&Lua, LuaValue) -> LuaValueResult, luau: &Lua) -> LuaValueResult {
    Ok(LuaValue::Function(luau.create_function(f)?))
}

pub fn ok_function_multi(f: fn(&Lua, LuaMultiValue) -> LuaMultiResult, luau: &Lua) -> LuaValueResult {
    Ok(LuaValue::Function(luau.create_function(f)?))
}

pub fn ok_function_mut<F, I, Fn>(f: Fn, luau: &Lua) -> LuaValueResult
where
    F: FromLuaMulti + 'static,
    I: IntoLuaMulti + 'static,
    Fn: FnMut(&Lua, F) -> LuaResult<I> + 'static,
{
    Ok(LuaValue::Function(luau.create_function_mut(f)?))
}

pub fn ok_string<S: AsRef<[u8]>>(s: S, luau: &Lua) -> LuaValueResult {
    Ok(LuaValue::String(luau.create_string(s)?))
}

pub fn ok_buffy<B: AsRef<[u8]>>(b: B, luau: &Lua) -> LuaValueResult {
    Ok(LuaValue::Buffer(luau.create_buffer(b)?))
}

pub fn ok_userdata<S: LuaUserData + Send + 'static>(u: S, luau: &Lua) -> LuaValueResult {
    Ok(LuaValue::UserData(luau.create_userdata(u)?))
}

pub fn pop_self(multivalue: &mut LuaMultiValue, function_name: &'static str) -> LuaEmptyResult {
    match multivalue.pop_front() {
        Some(LuaValue::Table(_s)) => Ok(()),
        Some(other) => {
            wrap_err!("{} expected to be called with self, got: {:?}; did you forget to use methodcall syntax (:)?", function_name, other)
        },
        None => {
            wrap_err!("{} incorrectly called with zero arguments, expected self", function_name)
        }
    }
}

pub struct DebugInfo {
    pub source: String,
    pub line: String,
    pub function_name: String,
}
impl DebugInfo {
    /// returns location info from the luau function that called the current (presumably rust) function
    pub fn from_caller(luau: &Lua, function_name: &'static str) -> LuaResult<Self> {
        const SLN_SRC: &str = r#"
            local source, line, function_name = debug.info(3, "sln")
            return {
                source = source,
                line = line,
                function_name = if function_name == "" then "top level" else function_name,
            }
        "#;
        let chunk = Chunk::Src(SLN_SRC.to_owned());
        let LuaValue::Table(info) = luau.load(chunk).set_name("gettin da debug info").eval()? else { // <<>> HACK
            return wrap_err!("{}: can't get debug info", function_name);
        };
        let source = match info.raw_get("source")? {
            LuaValue::String(s) => s.to_string_lossy(),
            LuaNil => String::from("<SOURCE NOT FOUND>"),
            other => {
                return wrap_err!("{}: expected source to be a string, got: {:?}", function_name, other);
            }
        };
        let line = match info.raw_get("line")? {
            LuaValue::Integer(n) => n.to_string(),
            LuaNil => String::from("<LINE NOT FOUND>"),
            other => {
                return wrap_err!("{}: expected line, got: {:?}", function_name, other);
            }
        };
        let caller_function_name = match info.raw_get("function_name")? {
            LuaValue::String(s) => s.to_string_lossy(),
            LuaNil => String::from("<FUNCTION NAME NOT FOUND>"),
            other => {
                return wrap_err!("{}: expected function_name to be a string, got: {:?}", function_name, other);
            }
        };

        Ok(Self { source, line, function_name: caller_function_name })
    }
}


/// safely convert i64 to usize while handling common problems like negatives and out of ranges
pub fn int_to_usize(i: i64, function_name: &str, parameter_name: &'static str) -> LuaResult<usize> {
    if i.is_negative() {
        return wrap_err!("{}: {} represents a byte offset or countable number and cannot be negative (got {})", function_name, parameter_name, i);
    }
    match usize::try_from(i) {
        Ok(u) => Ok(u),
        Err(err) => {
            wrap_err!("{}: {} can't safely be converted from i64 to usize because {}", function_name, parameter_name, err)
        }
    }
}

pub fn int_to_u64(i: i64, function_name: &'static str, parameter_name: &'static str) -> LuaResult<u64> {
    if i.is_negative() {
        return wrap_err!("{}: {} must be positive (got: {})", function_name, parameter_name, i);
    }
    match u64::try_from(i) {
        Ok(u) => Ok(u),
        Err(err) => {
            wrap_err!("{}: {} can't safely be converted from i64 to u64 because {}", function_name, parameter_name, err)
        }
    }
}

/// safely convert float param to usize, giving a good error reason if it didn't successfully convert
pub fn float_to_usize(f: f64, function_name: &'static str, parameter_name: &'static str) -> LuaResult<usize> {
    let truncated = f.trunc();
    if truncated.is_nan() || truncated.is_infinite() {
        wrap_err!("{}: {} cannot be NaN nor infinite", function_name, parameter_name)
    } else if truncated.is_sign_negative() {
        wrap_err!("{}: {} represents a byte offset and cannot be negative (got: {})", function_name, parameter_name, truncated)
    } else if truncated > usize::MAX as f64 {
        wrap_err!("{}: expected {} to be convertible into usize, however provided float is too big to fit (got: {})", function_name, parameter_name, f)
    } else if truncated == f {
        // SAFETY: we just checked nan/infinite/size/negative right above
        let i = unsafe { truncated.to_int_unchecked() };
        int_to_usize(i, function_name, parameter_name)
    } else {
        wrap_err!("{} expected {} to be an integer number, unexpectedly got a float: {}", function_name, parameter_name, f)
    }
}

/// safely convert float param to u64, giving a good error reason if conversion wasn't successful
pub fn float_to_u64(f: f64, function_name: &'static str, parameter_name: &'static str) -> LuaResult<u64> {
    let truncated = f.trunc();
    if truncated.is_nan() || truncated.is_infinite() {
        wrap_err!("{}: {} cannot be NaN nor infinite", function_name, parameter_name)
    } else if truncated.is_sign_negative() {
        wrap_err!("{}: {} cannot be negative (got: {})", function_name, parameter_name, f)
    } else if truncated > u64::MAX as f64 {
        wrap_err!("{} expected {} to be convertible to u64, but provided float is too big to fit (got: {})", function_name, parameter_name, f)
    } else if truncated == f {
        // SAFETY: just checked nan/infinite/size/negative right above
        let u: u64 = unsafe { truncated.to_int_unchecked() };
        Ok(u)
    } else {
        wrap_err!("{} expected {} to be an integer, unexpectedly got a float: {}", function_name, parameter_name, f)
    }
}

/// Creates table with capacity, clamping upper capacity to `MAX_TABLE_SIZE` for safety
pub fn create_table_with_capacity(luau: &Lua, n_array: usize, n_records: usize) -> LuaResult<LuaTable> {
    let n_array = std::cmp::min(n_array, MAX_TABLE_SIZE);
    let n_records = std::cmp::min(n_records, MAX_TABLE_SIZE);
    // SAFETY: luau.create_table_with_capacity will abort if `capacity` exceeds MAX_TABLE_SIZE (throwing Rust cannot catch foreign exceptions)
    // We clamp `good_prealloc_guess` to MAX_TABLE_SIZE to guarantee safety.
    // This API should be marked unsafe... but isn't.. so we explicitly treat it as unsafe here.
    #[allow(unused_unsafe)]
    unsafe { luau.create_table_with_capacity(n_array, n_records) }
}

use std::str;

// WARNING: AI GENERATED WILL BE REMOVED ONCE MLUAU UPDATES
// HACK: strip Luau generic call syntax <<...>> before function calls,
// while preserving UTF-8 and leaving all comment forms untouched.
pub fn temp_transform_luau_src<S: AsRef<str>>(chunk: S) -> String {
    // Get bytes from the chunk (Cow<[u8]>), then decode as UTF-8.
    // let cow = match chunk.source() {
    //     Ok(cow) => cow,
    //     Err(_) => return String::new(),
    // };
    // let bytes = cow.as_ref();
    // let src_str = match str::from_utf8(bytes) {
    //     Ok(s) => s,
    //     Err(_) => return String::from_utf8_lossy(bytes).into_owned(),
    // };
    let src_str = chunk.as_ref();

    let mut out = String::with_capacity(src_str.len());
    let mut chars = src_str.chars().peekable();

    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut block_eq_count = 0;

    while let Some(c) = chars.next() {
        // Inside a line comment: copy verbatim until newline
        if in_line_comment {
            out.push(c);
            if c == '\n' {
                in_line_comment = false;
            }
            continue;
        }

        // Inside a block/doc comment: copy verbatim until closing ]=...]
        if in_block_comment {
            out.push(c);
            if c == ']' {
                // check for ]=...]
                let mut temp = chars.clone();
                let mut eq_seen = 0;
                while temp.peek() == Some(&'=') {
                    temp.next();
                    eq_seen += 1;
                }
                if eq_seen == block_eq_count && temp.peek() == Some(&']') {
                    // consume '=' signs and final ']'
                    for _ in 0..eq_seen {
                        out.push(chars.next().unwrap());
                    }
                    out.push(chars.next().unwrap());
                    in_block_comment = false;
                }
            }
            continue;
        }

        // Detect start of comments
        if c == '-' && chars.peek() == Some(&'-') {
            out.push(c);
            out.push(chars.next().unwrap()); // consume second '-'

            if chars.peek() == Some(&'[') {
                // lookahead for --[=*[ 
                let mut temp = chars.clone();
                temp.next(); // consume '['
                let mut eq_count = 0;
                while temp.peek() == Some(&'=') {
                    temp.next();
                    eq_count += 1;
                }
                if temp.peek() == Some(&'[') {
                    // it's a block/doc comment
                    in_block_comment = true;
                    block_eq_count = eq_count;
                } else {
                    in_line_comment = true;
                }
            } else {
                in_line_comment = true;
            }
            continue;
        }

        // Detect and skip << ... >> outside comments, supporting nested << >>
        if c == '<' && chars.peek() == Some(&'<') {
            chars.next(); // consume second '<'
            let mut depth = 1;
            while let Some(c2) = chars.next() {
                if c2 == '<' && chars.peek() == Some(&'<') {
                    chars.next();
                    depth += 1;
                } else if c2 == '>' && chars.peek() == Some(&'>') {
                    chars.next();
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
            continue;
        }

        // Normal character
        out.push(c);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_simple_single_line() {
        let src = "foo<<int>>(123)";
        let expected = "foo(123)";
        assert_eq!(temp_transform_luau_src(src), expected);
    }

    #[test]
    fn strips_multiple_type_params_single_line() {
        let src = "bar<<A,B>>{\"x\"}";
        let expected = "bar{\"x\"}";
        assert_eq!(temp_transform_luau_src(src), expected);
    }

    #[test]
    fn strips_multiline_block() {
        let src = r#"
local result = new<< {
  create: <T>(self: T) -> string,
} >>()
"#;
        let expected = r#"
local result = new()
"#;
        assert_eq!(temp_transform_luau_src(src), expected);
    }

    #[test]
    fn strips_nested_multiline_block() {
        let src = r#"
local result = new<< {
  inner: <<X>>(self: X) -> (),
} >>("arg")
"#;
        let expected = r#"
local result = new("arg")
"#;
        assert_eq!(temp_transform_luau_src(src), expected);
    }

    #[test]
    fn preserves_documentation_comment_with_equals() {
        let src = r#"
--[=[ This is a doc comment with <<notype>> inside ]=]
local x = foo<<T>>()
"#;
        let expected = r#"
--[=[ This is a doc comment with <<notype>> inside ]=]
local x = foo()
"#;
        assert_eq!(temp_transform_luau_src(src), expected);
    }

    #[test]
    fn preserves_nested_equals_doc_comment() {
        let src = r#"
--[==[ Another doc comment
with <<stuff>> inside ]==]
local y = bar<<U>>("test")
"#;
        let expected = r#"
--[==[ Another doc comment
with <<stuff>> inside ]==]
local y = bar("test")
"#;
        assert_eq!(temp_transform_luau_src(src), expected);
    }

    #[test]
    fn preserves_line_comment_with_generics() {
        let src = r#"
-- this is a comment <<notype>>
local z = baz<<V>>() 
"#;
        let expected = r#"
-- this is a comment <<notype>>
local z = baz() 
"#;
        assert_eq!(temp_transform_luau_src(src), expected);
    }

    #[test]
    fn preserves_block_comment_with_generics() {
        let src = r#"
--[[ block comment <<ignored>> ]]
local w = qux<<W>>(42)
"#;
        let expected = r#"
--[[ block comment <<ignored>> ]]
local w = qux(42)
"#;
        assert_eq!(temp_transform_luau_src(src), expected);
    }
}
