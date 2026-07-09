use aho_corasick::AhoCorasick;
use unicode_segmentation::UnicodeSegmentation;

use std::io::Cursor;
use unicode_reader::Graphemes;

use mluau::prelude::*;
use crate::prelude::*;

fn split_graphemes(luau: &Lua, s_bytes: &[u8]) -> LuaResult<LuaTable> {
    // we want to assume most graphemes are normal ascii but some could be multibyte, and we want to error on the side
    // of not preallocating more than we need, so 3/4th of the byte len is a good compromise
    let good_prealloc_guess = std::cmp::min(s_bytes.len() * 3 / 4, MAX_TABLE_SIZE); // exceeding max table size causes abort
    let result = luau.create_table_with_capacity(good_prealloc_guess, 0)?;
    // check if whole string is valid utf-8
    if let Ok(s_str) = std::str::from_utf8(s_bytes) {
        for grapheme in s_str.graphemes(true) {
            result.raw_push(luau.create_string(grapheme)?)?;
        }
    } else {
        // uh oh we have a mix of graphemes and invalid utf8
        for chunk in s_bytes.utf8_chunks() {
            for grapheme in chunk.valid().graphemes(true) {
                result.raw_push(luau.create_string(grapheme)?)?;
            }
            if !chunk.invalid().is_empty() {
                result.raw_push(luau.create_string(chunk.invalid())?)?;
            }
        }
    }
    Ok(result)
}

enum SplitMode {
    Default,
    Around,
    Before,
    After,
}

fn split_separators(
    luau: &Lua,
    s_bytes: &[u8],
    multivalue: LuaMultiValue,
    function_name: &'static str,
    mode: SplitMode
) -> LuaResult<LuaTable> {
    let mut separators = Vec::new();
    for (index, value) in multivalue.iter().enumerate() {
        match value {
            LuaValue::String(sep) => {
                separators.push(sep.as_bytes().to_owned());
            },
            other => {
                return wrap_err!("{}: separator at index {} is not a string; got: {:?}", function_name, index, other);
            }
        }
    }
    // if we're splitting by separators, it's probably a comma, \n, or something else
    // we likely won't have as many results as we would when splitting by graphemes, so we preallocate less
    let good_prealloc_guess = s_bytes.len() / 6;
    let result = create_table_with_capacity(luau, good_prealloc_guess, 0)?;
    let ac =  match AhoCorasick::new(separators) {
        Ok(ac) => ac,
        Err(err) => {
            return wrap_err!("{}: can't initialize AhoCorasick matcher (with separators) due to err: {}", function_name, err);
        }
    };

    let ac_iterator =  match ac.try_find_iter(s_bytes) {
        Ok(iterator) => iterator,
        Err(err) => {
            return wrap_err!("{}: can't generate AhoCorasick iterator due to err: {}", function_name, err);
        }
    };

    let mut prev_index = 0;
    let mut last_match_end = None;
    for mat in ac_iterator {
        let mat_start = mat.start();
        let mat_end = mat.end();

        match mode {
            SplitMode::Default => {
                if mat_start >= prev_index {
                    let slice = &s_bytes[prev_index..mat_start];
                    if !slice.is_empty() {
                        result.raw_push(luau.create_string(slice)?)?;
                    }
                }
                prev_index = mat_end;
            }
            SplitMode::Around => {
                if mat_start >= prev_index {
                    let slice = &s_bytes[prev_index..mat_start];
                    if !slice.is_empty() {
                        result.raw_push(luau.create_string(slice)?)?;
                    }
                }
                let sep = &s_bytes[mat_start..mat_end];
                result.raw_push(luau.create_string(sep)?)?;
                prev_index = mat_end;
            }
            SplitMode::Before => {
                if mat_start > prev_index {
                    // push the chunk before start of the separator
                    let chunk = &s_bytes[prev_index..mat_start];
                    result.raw_push(luau.create_string(chunk)?)?;
                }

                // start subsequent chunk at the separator
                prev_index = mat_start;
            }
            SplitMode::After => {
                if mat_start >= prev_index {
                    let slice = &s_bytes[prev_index..mat_end]; // include separator
                    if !slice.is_empty() {
                        result.raw_push(luau.create_string(slice)?)?;
                    }
                }
                prev_index = mat_end;
            }
        }

        last_match_end = Some((mat.start(), mat.end()));
    }

    let s_len = s_bytes.len();
    if prev_index < s_len {
        // final element excluded (between last match and end of string) so let's push it manually
        result.raw_push(luau.create_string(&s_bytes[prev_index..s_len])?)?;
    }

    // if mode is SplitMode::Before and the string ends with a matched separator,
    // push the separator as the final element of the returned list (instead of omitting it)
    if matches!(mode, SplitMode::Before)
        && let Some((start, end)) = last_match_end
        && end == s_len
    {
        let sep = &s_bytes[start..end];
        result.raw_push(luau.create_string(sep)?)?;
    }

    Ok(result)
}


/// str.split is an improvement on luau's string.split in that you can split on multiple different choices of characters/strings
/// (not just a single string) and that the splitting is fully unicode grapheme aware
/// by default, str.split splits the string by unicode characters (graphemes)
fn str_split(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "str.split(s: string, chars/graphemes: ...string)";
    let s_bytes = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => {
            s.as_bytes().to_owned()
        },
        Some(other) => {
            return wrap_err!("{} expected s to be a string, got: {:?}", function_name, other);
        }
        None => {
            return wrap_err!("{} expected a string s, but was incorrectly called with zero arguments", function_name);
        }
    };

    let result = if multivalue.is_empty() {
        split_graphemes(luau, &s_bytes)?
    } else {
        split_separators(luau, &s_bytes, multivalue, function_name, SplitMode::Default)?
    };
    Ok(LuaValue::Table(result))
}

/// str.splitaround has the same semantics as str.split except it splits around the separator strings, keeping them in the final result
fn str_splitaround(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "str.splitaround(s: string, separators: ...string)";
    let s_bytes = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => {
            s.as_bytes().to_owned()
        },
        Some(other) => {
            return wrap_err!("{} expected s to be a string, got: {:?}", function_name, other);
        }
        None => {
            return wrap_err!("{} expected a string s, but was incorrectly called with zero arguments", function_name);
        }
    };

    let result = if multivalue.is_empty() {
        split_graphemes(luau, &s_bytes)?
    } else {
        split_separators(luau, &s_bytes, multivalue, function_name, SplitMode::Around)?
    };
    Ok(LuaValue::Table(result))
}

/// str.splitbefore
fn str_splitbefore(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "str.splitbefore(s: string, separators: ...string)";
    let s_bytes = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => {
            s.as_bytes().to_owned()
        },
        Some(other) => {
            return wrap_err!("{} expected s to be a string, got: {:?}", function_name, other);
        }
        None => {
            return wrap_err!("{} expected a string s, but was incorrectly called with zero arguments", function_name);
        }
    };

    let result = if multivalue.is_empty() {
        split_graphemes(luau, &s_bytes)?
    } else {
        split_separators(luau, &s_bytes, multivalue, function_name, SplitMode::Before)?
    };
    Ok(LuaValue::Table(result))
}

/// str.splitafter
fn str_splitafter(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "str.splitafter(s: string, separators: ...string)";
    let s_bytes = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => {
            s.as_bytes().to_owned()
        },
        Some(other) => {
            return wrap_err!("{} expected s to be a string, got: {:?}", function_name, other);
        }
        None => {
            return wrap_err!("{} expected a string s, but was incorrectly called with zero arguments", function_name);
        }
    };

    let result = if multivalue.is_empty() {
        split_graphemes(luau, &s_bytes)?
    } else {
        split_separators(luau, &s_bytes, multivalue, function_name, SplitMode::After)?
    };
    Ok(LuaValue::Table(result))
}

type GraphemePair = Option<(usize, String)>;
/// returns an iterator function you can call multiple times to get the next grapheme of String `s`
/// without loading the whole String into memory
fn graphemes(s: String, function_name: &'static str) -> Box<dyn FnMut() -> LuaResult<GraphemePair>> {
    let mut current_byte: usize = 0;
    let s_cursor = Cursor::new(s);
    let mut graphemes = Graphemes::from(s_cursor);

    let stateful_iter_fn = move || -> LuaResult<GraphemePair> {
        let Some(result) = graphemes.next() else {
            return Ok(None);
        };
        match result {
            Ok(grapheme) => {
                let r = (current_byte, grapheme);
                current_byte += 1;
                Ok(Some(r))
            },
            Err(err) => {
                wrap_err!(
                    "{}: bytes around and after {} could not be converted to utf-8 graphemes: {}",
                    function_name, current_byte, err
                )
            }
        }
    };

    Box::new(stateful_iter_fn)
}

fn str_graphemes(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "str.graphemes(s: string)";
    let s = match value {
        LuaValue::String(s) => match s.to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => {
                return wrap_err!("{} expected s to be a valid utf-8 encoded string", function_name);
            }
        },
        other => {
            return wrap_err!("{} expected s to be a string, got: {:?}", function_name, other);
        }
    };

    let mut next_grapheme = graphemes(s, function_name);

    let iter_fn =
        luau.create_function_mut(move | luau: &Lua, _value: LuaValue| -> LuaMultiResult
    {
        let Some((current_byte, grapheme)) = next_grapheme()? else {
            return Ok(LuaMultiValue::from_vec(vec![LuaNil]));
        };

        let current_byte_for_luau = match i64::try_from(current_byte) {
            Ok(i) => i + 1, // we want to align start with string.sub start in luau
            Err(_) => {
                return wrap_err!("{}: usize too big to fit in i64 :(", function_name)
            }
        };

        Ok(LuaMultiValue::from_vec(vec![
            LuaValue::Integer(current_byte_for_luau),
            LuaValue::String(luau.create_string(&grapheme)?)
        ]))
    })?;

    Ok(LuaValue::Function(iter_fn))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Encoding {
    Utf8,
    Utf8Bom,
    Utf16LE,
    Utf16LEBom,
    Utf16BE,
    Utf16BEBom,
    Binary,
}

impl Encoding {
    const VALID_ENCODINGS: &'static str = "\"Utf8\" | \"Utf8Bom\" | \"Utf16LE\" | \"Utf16LEBom\" | \"Utf16BE\" | \"Utf16BEBom\" | \"Binary\"";

    fn as_str(self) -> &'static str {
        match self {
            Self::Utf8 => "Utf8",
            Self::Utf8Bom => "Utf8Bom",
            Self::Utf16LE => "Utf16LE",
            Self::Utf16LEBom => "Utf16LEBom",
            Self::Utf16BE => "Utf16BE",
            Self::Utf16BEBom => "Utf16BEBom",
            Self::Binary => "Binary",
        }
    }

    fn from_luau_string(s: LuaString, function_name: &'static str) -> LuaResult<Self> {
        Ok(match s.as_bytes().as_ref() {
            b"Utf8" => Self::Utf8,
            b"Utf8Bom" => Self::Utf8Bom,
            b"Utf16LE" => Self::Utf16LE,
            b"Utf16LEBom" => Self::Utf16LEBom,
            b"Utf16BE" => Self::Utf16BE,
            b"Utf16BEBom" => Self::Utf16BEBom,
            b"Binary" => Self::Binary,
            _ => {
                return wrap_err!("{}: expected encoding to be one of {}, got: {:?}", function_name, Self::VALID_ENCODINGS, s);
            }
        })
    }

    fn from_value(value: LuaValue, function_name: &'static str) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => Self::from_luau_string(s, function_name),
            other => wrap_err!("{}: expected encoding to be a string ({}), got: {:?}", function_name, Self::VALID_ENCODINGS, other),
        }
    }
}

/// bare (bom-less) utf-16 has no reliable signature to detect it by, so we guess from the density of null bytes:
/// ascii-range utf-16 text has a null byte in either the high or low byte of every code unit
fn looks_like_bare_utf16(bytes: &[u8]) -> Option<Encoding> {
    if bytes.len() < 4 || !bytes.len().is_multiple_of(2) {
        return None;
    }

    const NULL_BYTE_DENSITY_THRESHOLD: f64 = 0.4;
    let code_units = bytes.len() / 2;
    let zeros_at_even = bytes.iter().step_by(2).filter(|byte| **byte == 0).count();
    let zeros_at_odd = bytes.iter().skip(1).step_by(2).filter(|byte| **byte == 0).count();

    if zeros_at_even as f64 / code_units as f64 > NULL_BYTE_DENSITY_THRESHOLD {
        Some(Encoding::Utf16BE)
    } else if zeros_at_odd as f64 / code_units as f64 > NULL_BYTE_DENSITY_THRESHOLD {
        Some(Encoding::Utf16LE)
    } else {
        None
    }
}

fn detect_encoding(bytes: &[u8]) -> Encoding {
    if let Some((encoding, _bom_len)) = encoding_rs::Encoding::for_bom(bytes) {
        return match encoding.name() {
            "UTF-8" => Encoding::Utf8Bom,
            "UTF-16LE" => Encoding::Utf16LEBom,
            "UTF-16BE" => Encoding::Utf16BEBom,
            _ => Encoding::Binary, // unreachable: for_bom only ever detects the encodings matched above
        };
    }

    if std::str::from_utf8(bytes).is_ok() {
        return Encoding::Utf8;
    }

    looks_like_bare_utf16(bytes).unwrap_or(Encoding::Binary)
}

fn str_encoding(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "str.encoding(s: string | buffer)";
    let encoding = match &value {
        LuaValue::String(s) => detect_encoding(s.as_bytes().as_ref()),
        LuaValue::Buffer(buffy) => detect_encoding(&buffy.to_vec()),
        other => {
            return wrap_err!("{} expected s to be a string or buffer, got: {:?}", function_name, other);
        }
    };
    ok_string(encoding.as_str(), luau)
}

/// decodes `bytes` (known to be `from`-encoded) into a Rust `String`, stripping any BOM that `from` implies
fn decode_bytes(bytes: &[u8], from: Encoding, function_name: &'static str) -> LuaResult<String> {
    let (encoding, bom_len) = match from {
        Encoding::Utf8 => (encoding_rs::UTF_8, 0),
        Encoding::Utf8Bom => (encoding_rs::UTF_8, 3),
        Encoding::Utf16LE => (encoding_rs::UTF_16LE, 0),
        Encoding::Utf16LEBom => (encoding_rs::UTF_16LE, 2),
        Encoding::Utf16BE => (encoding_rs::UTF_16BE, 0),
        Encoding::Utf16BEBom => (encoding_rs::UTF_16BE, 2),
        Encoding::Binary => {
            return wrap_err!("{}: can't convert from Binary data since it isn't decodable text; use str.encoding to detect its actual encoding first", function_name);
        }
    };
    let (decoded, _had_errors) = encoding.decode_without_bom_handling(&bytes[bom_len.min(bytes.len())..]);
    Ok(decoded.into_owned())
}

/// encoding_rs intentionally can't encode to UTF-16 (per the WHATWG spec, UTF-16 is decode-only),
/// so utf-16 output is written by hand via `str::encode_utf16`
fn utf16_bytes(s: &str, big_endian: bool) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(s.len() * 2);
    for unit in s.encode_utf16() {
        bytes.extend_from_slice(&if big_endian { unit.to_be_bytes() } else { unit.to_le_bytes() });
    }
    bytes
}

/// `to == Binary` is handled by `str_convert` as a passthrough before this is ever called,
/// since Binary just means "give me the original bytes back untouched", not a real text encoding
fn encode_bytes(s: &str, to: Encoding) -> Vec<u8> {
    match to {
        Encoding::Utf8 | Encoding::Binary => s.as_bytes().to_vec(),
        Encoding::Utf8Bom => [&[0xEF, 0xBB, 0xBF], s.as_bytes()].concat(),
        Encoding::Utf16LE => utf16_bytes(s, false),
        Encoding::Utf16LEBom => [&[0xFF, 0xFE], utf16_bytes(s, false).as_slice()].concat(),
        Encoding::Utf16BE => utf16_bytes(s, true),
        Encoding::Utf16BEBom => [&[0xFE, 0xFF], utf16_bytes(s, true).as_slice()].concat(),
    }
}

fn str_convert(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "str.convert(s: string | buffer, to: Encoding, from: Encoding?)";
    let bytes: Vec<u8> = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.as_bytes().to_owned(),
        Some(LuaValue::Buffer(buffy)) => buffy.to_vec(),
        Some(other) => {
            return wrap_err!("{} expected s to be a string or buffer, got: {:?}", function_name, other);
        },
        None => {
            return wrap_err!("{} expected to be called with s, but received zero arguments", function_name);
        }
    };

    let to = match multivalue.pop_front() {
        Some(value) => Encoding::from_value(value, function_name)?,
        None => {
            return wrap_err!("{} expected to be called with a target encoding `to`", function_name);
        }
    };

    let from = match multivalue.pop_front() {
        Some(LuaNil) | None => detect_encoding(&bytes),
        Some(value) => Encoding::from_value(value, function_name)?,
    };

    if from == to || to == Encoding::Binary {
        return ok_string(bytes, luau);
    }

    let decoded = decode_bytes(&bytes, from, function_name)?;
    let converted = encode_bytes(&decoded, to);
    ok_string(converted, luau)
}

/// rust-implemented functions that str.luau pulls in via `require("@internal/str")`;
/// kept behind the @internal alias (like @internal/setup) instead of luau module top-level `...`,
/// since the latter is reserved by luau for the frozen cross-module-inlining export table
pub fn create_internal(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function_and_signature("split", str_split, signatures::STD_STR_SPLIT)?
        .with_function_and_signature("splitaround", str_splitaround, signatures::STD_STR_SPLITAROUND)?
        .with_function_and_signature("splitbefore", str_splitbefore, signatures::STD_STR_SPLITBEFORE)?
        .with_function_and_signature("splitafter", str_splitafter, signatures::STD_STR_SPLITAFTER)?
        .with_function_and_signature("graphemes", str_graphemes, signatures::STD_STR_GRAPHEMES)?
        .with_function_and_signature("encoding", str_encoding, signatures::STD_STR_ENCODING)?
        .with_function_and_signature("convert", str_convert, signatures::STD_STR_CONVERT)?
        .build_readonly()
}

const STR_DOT_LUAU_SRC: &str = include_str!("./str.luau");

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    let chunk = Chunk::Src(STR_DOT_LUAU_SRC.to_owned());
    match luau.load(chunk).set_name("std/str").eval::<LuaTable>() {
        Ok(str_table) => Ok(str_table),
        Err(err) => {
            panic!("std/str's str.luau did a bad: {}", err);
        }
    }
}