use aho_corasick::AhoCorasick;
use unicode_segmentation::UnicodeSegmentation;

use std::io::Cursor;
use unicode_reader::Graphemes;

use mluau::prelude::*;
use crate::prelude::*;

/// bytes-per-split divisor grows with sqrt(len) past `SPLIT_SCALE_THRESHOLD`, so the guess grows sublinearly
/// rather than linearly with input size. this is for separator-style splits (comma, newline, etc): real-world
/// large inputs (multi-gb files, etc) almost always have much longer average records than small snippets do,
/// so assuming a constant split density at any scale wildly overestimates for huge inputs.
///
/// this only sizes an intermediate rust `Vec` now (see `build_table_from_slices`), not the luau table
/// directly, so a bad guess just costs a `Vec` reallocation or two, never an abort - but a decent guess
/// still avoids needless reallocation churn for the common case
const SPLIT_SCALE_THRESHOLD: usize = 1 << 16; // 64 KiB
const SPLIT_PREALLOC_CAP: usize = 1 << 20;

fn separator_split_prealloc_guess(len: usize, base_divisor: f64) -> usize {
    let divisor = if len <= SPLIT_SCALE_THRESHOLD {
        base_divisor
    } else {
        base_divisor * ((len / SPLIT_SCALE_THRESHOLD) as f64).sqrt()
    };
    let guess = (len as f64 / divisor) as usize;
    guess.min(SPLIT_PREALLOC_CAP)
}

/// converts a list of byte slices into a luau array table via `create_sequence_from` (one lock,
/// one stack guard for the whole batch) instead of pushing one-by-one; `&str` gets the fastest path,
/// `BString` (non-utf-8 fallback) still beats the old per-item raw_push+create_string loop
fn build_table_from_slices(luau: &Lua, slices: Vec<&[u8]>) -> LuaResult<LuaTable> {
    let mut strs = Vec::with_capacity(slices.len());
    for slice in &slices {
        match std::str::from_utf8(slice) {
            Ok(s) => strs.push(s),
            Err(_) => {
                let bstrings: Vec<mluau::BString> = slices.into_iter().map(mluau::BString::from).collect();
                return luau.create_sequence_from(bstrings);
            }
        }
    }
    luau.create_sequence_from(strs)
}

fn split_graphemes(luau: &Lua, s_bytes: &[u8]) -> LuaResult<LuaTable> {
    // check if whole string is valid utf-8
    if let Ok(s_str) = std::str::from_utf8(s_bytes) {
        // fast path: every grapheme is already a valid &str, so hand the whole vec straight to
        // create_sequence_from instead of build_table_from_slices (which would just redundantly
        // re-validate utf-8 we already know holds for every single grapheme)
        let graphemes: Vec<&str> = s_str.graphemes(true).collect();
        return luau.create_sequence_from(graphemes);
    }

    // uh oh we have a mix of graphemes and invalid utf8; this is rare/edge-case data, so we don't
    // bother distinguishing valid/invalid chunks here - build_table_from_slices figures out that
    // at least one chunk isn't valid utf-8 and falls back to the slow path on its own
    let mut slices: Vec<&[u8]> = Vec::new();
    for chunk in s_bytes.utf8_chunks() {
        for grapheme in chunk.valid().graphemes(true) {
            slices.push(grapheme.as_bytes());
        }
        if !chunk.invalid().is_empty() {
            slices.push(chunk.invalid());
        }
    }
    build_table_from_slices(luau, slices)
}

enum SplitMode {
    Default,
    Around,
    Before,
    After,
    /// like Default, but never drops an empty segment between two matches (only used by
    /// str.splitlines' non-utf-8 fallback; str.split's own Default mode must keep dropping
    /// empty splits, so this can't just be folded into Default)
    Lines,
}

fn split_separators(
    luau: &Lua,
    s_bytes: &[u8],
    multivalue: LuaMultiValue,
    function_name: &'static str,
    mode: SplitMode
) -> LuaResult<LuaTable> {
    let slices = split_separators_slices(s_bytes, multivalue, function_name, mode)?;
    build_table_from_slices(luau, slices)
}

/// the byte-slice-producing half of `split_separators`, split out so callers that need to
/// post-process the pieces (like str.splitlines' non-utf-8 fallback trimming each line) can do so
/// before ever touching the luau vm, instead of patching table entries one-by-one afterwards
fn split_separators_slices<'a>(
    s_bytes: &'a [u8],
    multivalue: LuaMultiValue,
    function_name: &'static str,
    mode: SplitMode,
) -> LuaResult<Vec<&'a [u8]>> {
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
    let good_prealloc_guess = separator_split_prealloc_guess(s_bytes.len(), 6.0);

    // fast path: a single single-byte separator (by far the most common case: "\n", ",", etc)
    // needs no automaton at all - memchr's vectorized byte scan is dramatically faster than even
    // aho-corasick's DFA for this (measured ~7x on a 175mb/5m-line input)
    if let [sep] = separators.as_slice()
        && let [sep_byte] = sep.as_slice()
    {
        let matches = memchr::memchr_iter(*sep_byte, s_bytes).map(|i| (i, i + 1));
        return Ok(collect_slices_from_matches(s_bytes, matches, &mode, good_prealloc_guess));
    }

    let ac = match AhoCorasick::new(separators) {
        Ok(ac) => ac,
        Err(err) => {
            return wrap_err!("{}: can't initialize AhoCorasick matcher (with separators) due to err: {}", function_name, err);
        }
    };
    let ac_iterator = match ac.try_find_iter(s_bytes) {
        Ok(iterator) => iterator,
        Err(err) => {
            return wrap_err!("{}: can't generate AhoCorasick iterator due to err: {}", function_name, err);
        }
    };
    let matches = ac_iterator.map(|mat| (mat.start(), mat.end()));
    Ok(collect_slices_from_matches(s_bytes, matches, &mode, good_prealloc_guess))
}

/// shared by both the memchr fast path and the general aho-corasick path: turns a stream of
/// (match_start, match_end) byte offsets into the actual slices `str.split`/etc should return,
/// according to `mode`
fn collect_slices_from_matches<'a>(
    s_bytes: &'a [u8],
    matches: impl Iterator<Item = (usize, usize)>,
    mode: &SplitMode,
    good_prealloc_guess: usize,
) -> Vec<&'a [u8]> {
    let mut slices: Vec<&[u8]> = Vec::with_capacity(good_prealloc_guess);

    let mut prev_index = 0;
    let mut last_match_end = None;
    for (mat_start, mat_end) in matches {
        match mode {
            SplitMode::Default => {
                if mat_start >= prev_index {
                    let slice = &s_bytes[prev_index..mat_start];
                    if !slice.is_empty() {
                        slices.push(slice);
                    }
                }
                prev_index = mat_end;
            }
            SplitMode::Around => {
                if mat_start >= prev_index {
                    let slice = &s_bytes[prev_index..mat_start];
                    if !slice.is_empty() {
                        slices.push(slice);
                    }
                }
                slices.push(&s_bytes[mat_start..mat_end]);
                prev_index = mat_end;
            }
            SplitMode::Before => {
                if mat_start > prev_index {
                    // push the chunk before start of the separator
                    slices.push(&s_bytes[prev_index..mat_start]);
                }

                // start subsequent chunk at the separator
                prev_index = mat_start;
            }
            SplitMode::After => {
                if mat_start >= prev_index {
                    let slice = &s_bytes[prev_index..mat_end]; // include separator
                    if !slice.is_empty() {
                        slices.push(slice);
                    }
                }
                prev_index = mat_end;
            }
            SplitMode::Lines => {
                if mat_start >= prev_index {
                    slices.push(&s_bytes[prev_index..mat_start]); // pushed even if empty
                }
                prev_index = mat_end;
            }
        }

        last_match_end = Some((mat_start, mat_end));
    }

    let s_len = s_bytes.len();
    if prev_index < s_len {
        // final element excluded (between last match and end of string) so let's push it manually
        slices.push(&s_bytes[prev_index..s_len]);
    }

    // if mode is SplitMode::Before and the string ends with a matched separator,
    // push the separator as the final element of the returned list (instead of omitting it)
    if matches!(mode, SplitMode::Before)
        && let Some((start, end)) = last_match_end
        && end == s_len
    {
        slices.push(&s_bytes[start..end]);
    }

    slices
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

/// pops (s, sep, max) off `multivalue` for str.splitfront/str.splitback, validating along the way.
/// `max` defaults to 1 if omitted (matching str.split with no cap would just duplicate str.split)
fn pop_splitfront_back_args(multivalue: &mut LuaMultiValue, function_name: &'static str) -> LuaResult<(Vec<u8>, Vec<u8>, usize)> {
    let s_bytes = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.as_bytes().to_owned(),
        Some(other) => {
            return wrap_err!("{} expected s to be a string, got: {:?}", function_name, other);
        },
        None => {
            return wrap_err!("{} expected a string s, but was incorrectly called with zero arguments", function_name);
        }
    };

    let sep_bytes = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.as_bytes().to_owned(),
        Some(other) => {
            return wrap_err!("{} expected sep to be a string, got: {:?}", function_name, other);
        },
        None => {
            return wrap_err!("{} expected a sep string to split on", function_name);
        }
    };
    if sep_bytes.is_empty() {
        return wrap_err!("{} expected sep to be a non-empty string", function_name);
    }

    let max = match multivalue.pop_front() {
        Some(LuaNil) | None => 1,
        Some(LuaValue::Integer(n)) => int_to_usize(n, function_name, "max")?,
        Some(LuaValue::Number(n)) => float_to_usize(n, function_name, "max")?,
        Some(other) => {
            return wrap_err!("{} expected max to be a number or nil, got: {:?}", function_name, other);
        }
    };

    Ok((s_bytes, sep_bytes, max))
}

/// splits `s` on `sep` starting from the front, stopping after `max` splits (default 1) - the
/// remainder (everything after the last split) becomes the final element as-is, even if `sep`
/// occurs again within it. like python's `str.split(sep, maxsplit)`
fn str_splitfront(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "str.splitfront(s: string, sep: string, max: number?)";
    let (s_bytes, sep_bytes, max) = pop_splitfront_back_args(&mut multivalue, function_name)?;

    let finder = memchr::memmem::Finder::new(&sep_bytes);
    let mut slices: Vec<&[u8]> = Vec::new();
    let mut prev = 0usize;
    for (splits_done, start) in finder.find_iter(&s_bytes).enumerate() {
        if splits_done >= max {
            break;
        }
        let slice = &s_bytes[prev..start];
        if !slice.is_empty() {
            slices.push(slice);
        }
        prev = start + sep_bytes.len();
    }
    let remainder = &s_bytes[prev..];
    if !remainder.is_empty() {
        slices.push(remainder);
    }

    ok_table(build_table_from_slices(luau, slices))
}

/// splits `s` on `sep` starting from the back, stopping after `max` splits (default 1) - the
/// remainder (everything before the first split) becomes the first element as-is, even if `sep`
/// occurs again within it. like python's `str.rsplit(sep, maxsplit)`
fn str_splitback(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "str.splitback(s: string, sep: string, max: number?)";
    let (s_bytes, sep_bytes, max) = pop_splitfront_back_args(&mut multivalue, function_name)?;

    let finder = memchr::memmem::Finder::new(&sep_bytes);
    let all_matches: Vec<usize> = finder.find_iter(&s_bytes).collect();
    let splits_to_take = max.min(all_matches.len());
    let taken = &all_matches[all_matches.len() - splits_to_take..];

    let mut slices: Vec<&[u8]> = Vec::new();
    let mut prev = 0usize;
    for (i, &start) in taken.iter().enumerate() {
        // the very first taken match absorbs everything before it, including any earlier
        // (non-taken) separator occurrences, since that's the "remainder" for this direction
        let piece_start = if i == 0 { 0 } else { prev };
        let slice = &s_bytes[piece_start..start];
        if !slice.is_empty() {
            slices.push(slice);
        }
        prev = start + sep_bytes.len();
    }
    let remainder = &s_bytes[prev..];
    if !remainder.is_empty() {
        slices.push(remainder);
    }

    ok_table(build_table_from_slices(luau, slices))
}

fn is_trailing_trim_byte(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | 0x0B | 0x0C | b'\r')
}

/// Splits `s` into lines based on the following rules:
/// - every `\n` or `\r\n` is a line separator (technically, every '\r' in front of an '\n' counts as part of the separator)
/// - We split around line separators, trim trailing whitespace if requested,
///   and preserve any empty lines in between other lines.
/// - If `s`'s final line is nothing but a trailing line separator (so the last element of the
///   result would otherwise be an empty string), we exclude that element by default, unless
///   `include_terminator` is true, in which case we keep it - regardless of `trim_trailing_whitespace`.
fn splitlines_str(s: &str, trim_trailing_whitespace: bool, include_terminator: bool) -> Vec<&str> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut lines = Vec::with_capacity(len / 20 + 1);
    let mut start = 0usize;

    loop {
        let nl = memchr::memchr(b'\n', &bytes[start..]).map(|i| start + i);
        let mut raw_end = nl.unwrap_or(len);

        // a "\r" immediately before this "\n" is part of the line terminator, not the line itself,
        // regardless of `trim_trailing_whitespace`
        if nl.is_some() && raw_end > start && bytes[raw_end - 1] == b'\r' {
            raw_end -= 1;
        }

        // a delimiter always closes off a line (even an empty one). the final, delimiter-less
        // chunk after the last "\n" only counts as a line if it's got raw content, or if
        // `include_terminator` says to keep it anyway (reflecting that a terminator was there,
        // even if trimming would otherwise erase all evidence of it) - independent of whether
        // trimming is even requested at all
        let should_push = nl.is_some() || raw_end > start || include_terminator;

        if should_push {
            let mut end = raw_end;
            if trim_trailing_whitespace {
                while end > start && is_trailing_trim_byte(bytes[end - 1]) {
                    end -= 1;
                }
            }
            // s[start..end] is safe (not unsafe from_utf8_unchecked) because both start and end
            // always land on a byte that's either 0, len, or immediately after '\n'/a trimmed
            // single-byte ascii whitespace char - never mid-codepoint - and rust's str indexing
            // cheaply asserts that (a boundary check, not a full utf-8 content re-scan) rather
            // than silently doing the wrong thing if this reasoning is ever somehow wrong
            lines.push(&s[start..end]);
        }

        match nl {
            Some(i) => start = i + 1,
            None => break,
        }
    }

    lines
}

fn str_splitlines(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "str.splitlines(s: string, trim_trailing_whitespace: boolean?, include_terminator: boolean?)";
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

    let trim_trailing_whitespace = match multivalue.pop_front() {
        Some(LuaValue::Boolean(b)) => b,
        Some(LuaNil) | None => true,
        Some(other) => {
            return wrap_err!("{} expected trim_trailing_whitespace to be a boolean or nil, got: {:?}", function_name, other);
        }
    };

    let include_terminator = match multivalue.pop_front() {
        Some(LuaValue::Boolean(b)) => b,
        Some(LuaNil) | None => false,
        Some(other) => {
            return wrap_err!("{} expected include_terminator to be a boolean or nil, got: {:?}", function_name, other);
        }
    };

    let result = match std::str::from_utf8(&s_bytes) {
        Ok(s_str) => {
            let lines = splitlines_str(s_str, trim_trailing_whitespace, include_terminator);
            luau.create_sequence_from(lines)?
        },
        Err(_) => {
            // rare/edge-case: not valid utf-8. fall back to the general (slower) byte-oriented
            // path: split on "\n"/"\r\n" via aho-corasick, trim each piece, then build the table once
            let multivalue = LuaMultiValue::from_vec(vec![
                LuaValue::String(luau.create_string("\n")?),
                LuaValue::String(luau.create_string("\r\n")?),
            ]);
            let mut slices = split_separators_slices(&s_bytes, multivalue, function_name, SplitMode::Lines)?;
            // split_separators_slices/collect_slices_from_matches already only pushes an
            // unterminated tail when it has raw content, matching include_terminator=false.
            // when include_terminator is true and s_bytes ends exactly at a real terminator
            // (no raw tail was pushed at all), add the trailing "" back in - independent of
            // trim_trailing_whitespace, matching splitlines_str's utf-8 behavior
            if include_terminator && s_bytes.ends_with(b"\n") {
                slices.push(&[]);
            }
            if trim_trailing_whitespace {
                for slice in &mut slices {
                    *slice = trim_trailing_bytes(slice);
                }
            }
            build_table_from_slices(luau, slices)?
        }
    };
    Ok(LuaValue::Table(result))
}

fn trim_trailing_bytes(bytes: &[u8]) -> &[u8] {
    let mut end = bytes.len();
    while end > 0 && is_trailing_trim_byte(bytes[end - 1]) {
        end -= 1;
    }
    &bytes[..end]
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

/// detects `bytes`' encoding and returns it as utf-8, only allocating/converting when `bytes` isn't
/// already plain utf-8; used by things like json.readfile so reading a utf-16 (bom'd or not) file just works
pub(crate) fn bytes_to_utf8<'a>(bytes: &'a [u8], function_name: &'static str) -> LuaResult<std::borrow::Cow<'a, str>> {
    match detect_encoding(bytes) {
        Encoding::Utf8 => match std::str::from_utf8(bytes) {
            Ok(s) => Ok(std::borrow::Cow::Borrowed(s)),
            Err(_) => unreachable!("detect_encoding already validated this is utf-8"),
        },
        Encoding::Binary => {
            wrap_err!("{}: content doesn't look like valid text in any supported encoding (utf-8, utf-16 with or without a bom)", function_name)
        },
        from => Ok(std::borrow::Cow::Owned(decode_bytes(bytes, from, function_name)?)),
    }
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

/// mirrors luau's own `string.sub` index semantics: positive counts from the start (1-indexed),
/// negative counts from the end (-1 = last byte), and everything is clamped into a valid range.
/// see lua's `str_sub`/`posrelat` in lstrlib.c, which this is a direct port of
fn posrelat(pos: i64, len: usize) -> i64 {
    if pos >= 0 {
        pos
    } else if (-pos) as usize > len {
        0
    } else {
        len as i64 - (-pos) + 1
    }
}

/// resolves a `string.sub`-style (start, end) pair into a clamped, 0-indexed, exclusive-end byte
/// range `[lo, hi)`. returns `(0, 0)` (empty range) if start ends up past end, same as `string.sub`
/// returning `""` in that case
fn resolve_sub_range(start: i64, end: i64, len: usize) -> (usize, usize) {
    let mut start = posrelat(start, len);
    let mut end = posrelat(end, len);
    if start < 1 {
        start = 1;
    }
    if end > len as i64 {
        end = len as i64;
    }
    if start > end { (0, 0) } else { ((start - 1) as usize, end as usize) }
}

fn pop_optional_index(multivalue: &mut LuaMultiValue, function_name: &'static str, parameter_name: &'static str, default: i64) -> LuaResult<i64> {
    match multivalue.pop_front() {
        Some(LuaNil) | None => Ok(default),
        Some(LuaValue::Integer(n)) => Ok(n),
        Some(LuaValue::Number(n)) => Ok(n as i64),
        Some(other) => {
            wrap_err!("{}: expected {} to be a number or nil, got: {:?}", function_name, parameter_name, other)
        }
    }
}

/// replaces non-overlapping, literal (not pattern) occurrences of `old` in `s` with `new`,
/// using memchr's SIMD-accelerated substring search - operates on raw bytes, so this works
/// regardless of whether `s` is valid utf-8. `start_index`/`end_index` (string.sub-style, 1-indexed,
/// negative-from-end) restrict which portion of `s` is searched; content outside that range is left
/// untouched even if `old` occurs there. `max_replacements` caps how many occurrences get replaced
fn str_replace(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "str.replace(s: string, old: string, new: string, max_replacements: number?, start_index: number?, end_index: number?)";
    let s_bytes = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.as_bytes().to_owned(),
        Some(other) => {
            return wrap_err!("{} expected s to be a string, got: {:?}", function_name, other);
        },
        None => {
            return wrap_err!("{} expected a string s, but was incorrectly called with zero arguments", function_name);
        }
    };

    let old_bytes = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.as_bytes().to_owned(),
        Some(other) => {
            return wrap_err!("{} expected old to be a string, got: {:?}", function_name, other);
        },
        None => {
            return wrap_err!("{} expected an old string to search for", function_name);
        }
    };
    if old_bytes.is_empty() {
        return wrap_err!("{} expected old to be a non-empty string", function_name);
    }

    let new_bytes = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.as_bytes().to_owned(),
        Some(other) => {
            return wrap_err!("{} expected new to be a string, got: {:?}", function_name, other);
        },
        None => {
            return wrap_err!("{} expected a new string to replace occurrences of old with", function_name);
        }
    };

    let max_replacements = match multivalue.pop_front() {
        Some(LuaNil) | None => usize::MAX,
        Some(LuaValue::Integer(n)) => int_to_usize(n, function_name, "max_replacements")?,
        Some(LuaValue::Number(n)) => float_to_usize(n, function_name, "max_replacements")?,
        Some(other) => {
            return wrap_err!("{} expected max_replacements to be a number or nil, got: {:?}", function_name, other);
        }
    };

    let start_index = pop_optional_index(&mut multivalue, function_name, "start_index", 1)?;
    let end_index = pop_optional_index(&mut multivalue, function_name, "end_index", -1)?;
    let (lo, hi) = resolve_sub_range(start_index, end_index, s_bytes.len());

    let finder = memchr::memmem::Finder::new(&old_bytes);
    let mut result = Vec::with_capacity(s_bytes.len());
    result.extend_from_slice(&s_bytes[..lo]);

    let mut last_end = lo;
    let mut replacements_made = 0usize;
    if lo < hi {
        for match_start_in_range in finder.find_iter(&s_bytes[lo..hi]) {
            if replacements_made >= max_replacements {
                break;
            }
            let match_start = lo + match_start_in_range;
            result.extend_from_slice(&s_bytes[last_end..match_start]);
            result.extend_from_slice(&new_bytes);
            last_end = match_start + old_bytes.len();
            replacements_made += 1;
        }
    }
    result.extend_from_slice(&s_bytes[last_end..]);

    ok_string(result, luau)
}

/// reverses `s` by unicode grapheme (not byte, unlike luau's own `string.reverse`, which would
/// corrupt multi-byte utf-8 content); falls back to a raw byte reversal for non-utf-8 input,
/// matching the rest of str's "don't error on arbitrary bytes" convention
fn str_reverse(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "str.reverse(s: string)";
    let s_bytes = match value {
        LuaValue::String(s) => s.as_bytes().to_owned(),
        other => {
            return wrap_err!("{} expected s to be a string, got: {:?}", function_name, other);
        }
    };

    let reversed: Vec<u8> = match std::str::from_utf8(&s_bytes) {
        Ok(s) => s.graphemes(true).rev().flat_map(|grapheme| grapheme.bytes()).collect(),
        Err(_) => s_bytes.iter().rev().copied().collect(),
    };

    ok_string(reversed, luau)
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
        .with_function_and_signature("splitlines", str_splitlines, signatures::STD_STR_SPLITLINES)?
        .with_function_and_signature("graphemes", str_graphemes, signatures::STD_STR_GRAPHEMES)?
        .with_function_and_signature("encoding", str_encoding, signatures::STD_STR_ENCODING)?
        .with_function_and_signature("convert", str_convert, signatures::STD_STR_CONVERT)?
        .with_function_and_signature("replace", str_replace, signatures::STD_STR_REPLACE)?
        .with_function_and_signature("reverse", str_reverse, signatures::STD_STR_REVERSE)?
        .with_function_and_signature("splitfront", str_splitfront, signatures::STD_STR_SPLITFRONT)?
        .with_function_and_signature("splitback", str_splitback, signatures::STD_STR_SPLITBACK)?
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