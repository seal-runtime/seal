#![allow(dead_code)]

use mluau::prelude::*;

use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

use crate::{table_helpers::TableBuilder, signatures};

// needed to let wrap_err! macro work in here
use self as colors;

struct NoColorCache {
    value: bool,
    last_checked: Instant,
    runtime_override: Option<bool>,
}

static NO_COLOR_CACHE: LazyLock<Mutex<NoColorCache>> = LazyLock::new(|| {
    Mutex::new(NoColorCache {
        value: compute_no_color(),
        last_checked: Instant::now(),
        runtime_override: None,
    })
});

fn compute_no_color() -> bool {
    if let Ok(val) = std::env::var("SEAL_COLORS") {
        let v = val.trim().to_ascii_lowercase();
        if v == "true" || v == "1" || v == "yes" || v == "on" || v == "y" || v == "t" {
            return false;
        }
        if v == "false" || v == "0" || v == "no" || v == "off" || v == "n" || v == "f" {
            return true;
        }
    }
    std::env::var("NO_COLOR").is_ok()
}

pub fn are_disabled() -> bool {
    let mut cache = NO_COLOR_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(forced) = cache.runtime_override {
        return !forced;
    }
    if cache.last_checked.elapsed() > Duration::from_millis(150) {
        cache.value = compute_no_color();
        cache.last_checked = Instant::now();
    }
    cache.value
}

fn colors_override(_luau: &Lua, enabled: bool) -> LuaResult<()> {
    let mut cache = NO_COLOR_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    cache.runtime_override = Some(enabled);
    Ok(())
}

fn colors_enabled(_luau: &Lua, _: ()) -> LuaResult<bool> {
    Ok(!are_disabled())
}

pub const RESET: &str = "\x1b[0m";
pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";

pub const BOLD_BLACK: &str = "\x1b[1;30m";
pub const BOLD_RED: &str = "\x1b[1;31m";
pub const BOLD_GREEN: &str = "\x1b[1;32m";
pub const BOLD_YELLOW: &str = "\x1b[1;33m";
pub const BOLD_BLUE: &str = "\x1b[1;34m";
pub const BOLD_MAGENTA: &str = "\x1b[1;35m";
pub const BOLD_CYAN: &str = "\x1b[1;36m";
pub const BOLD_WHITE: &str = "\x1b[1;37m";

pub const BRIGHT_BLACK: &str = "\x1b[90m";
pub const BRIGHT_RED: &str = "\x1b[91m";
pub const BRIGHT_GREEN: &str = "\x1b[92m";
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
pub const BRIGHT_BLUE: &str = "\x1b[94m";
pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
pub const BRIGHT_CYAN: &str = "\x1b[96m";
pub const BRIGHT_WHITE: &str = "\x1b[97m";

pub const BLACK_BG: &str = "\x1b[40m";
pub const RED_BG: &str = "\x1b[41m";
pub const GREEN_BG: &str = "\x1b[42m";
pub const YELLOW_BG: &str = "\x1b[43m";
pub const BLUE_BG: &str = "\x1b[44m";
pub const MAGENTA_BG: &str = "\x1b[45m";
pub const CYAN_BG: &str = "\x1b[46m";
pub const WHITE_BG: &str = "\x1b[47m";

pub const BRIGHT_BLACK_BG: &str = "\x1b[100m";
pub const BRIGHT_RED_BG: &str = "\x1b[101m";
pub const BRIGHT_GREEN_BG: &str = "\x1b[102m";
pub const BRIGHT_YELLOW_BG: &str = "\x1b[103m";
pub const BRIGHT_BLUE_BG: &str = "\x1b[104m";
pub const BRIGHT_MAGENTA_BG: &str = "\x1b[105m";
pub const BRIGHT_CYAN_BG: &str = "\x1b[106m";
pub const BRIGHT_WHITE_BG: &str = "\x1b[107m";

pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const UNDERLINE: &str = "\x1b[4m";

type LuaValueResult = LuaResult<LuaValue>;

fn rgb(luau: &Lua, (rgb_vec, text): (LuaVector, Option<String>)) -> LuaValueResult {
    let function_name = "colors.rgb(rgb: vector, text: string?)";
    if are_disabled() {
        return Ok(LuaValue::String(luau.create_string(text.as_deref().unwrap_or(""))?));
    }
    let (r, g, b) = (rgb_vec.x(), rgb_vec.y(), rgb_vec.z());
    if !(0.0..=255.0).contains(&r) || !(0.0..=255.0).contains(&g) || !(0.0..=255.0).contains(&b) {
        return wrap_err!("{}: r, g, b must each be in the range 0-255, got: {}, {}, {}", function_name, r, g, b);
    }
    let code = format!("\x1b[38;2;{};{};{}m", r as u8, g as u8, b as u8);
    match text.as_deref() {
        Some(t) if !t.is_empty() => Ok(LuaValue::String(luau.create_string(&(code + t + RESET))?)),
        _ => Ok(LuaValue::String(luau.create_string(&code)?)),
    }
}

fn colorize(luau: &Lua, text: String, color_code: &str) -> LuaValueResult {
    if are_disabled() {
        return Ok(LuaValue::String(luau.create_string(&text)?));
    }
    let colored_text = color_code.to_string() + &text + RESET;
    Ok(LuaValue::String(luau.create_string(&colored_text)?))
}

fn colorize_black(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, BLACK)
}

fn colorize_red(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, RED)
}

fn colorize_green(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, GREEN)
}

fn colorize_yellow(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, YELLOW)
}

fn colorize_blue(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, BLUE)
}

fn colorize_magenta(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, MAGENTA)
}

fn colorize_cyan(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, CYAN)
}

fn colorize_white(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, WHITE)
}

fn colorize_bold_black(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, BOLD_BLACK)
}

fn colorize_bold_red(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, BOLD_RED)
}

fn colorize_bold_green(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, BOLD_GREEN)
}

fn colorize_bold_yellow(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, BOLD_YELLOW)
}

fn colorize_bold_blue(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, BOLD_BLUE)
}

fn colorize_bold_magenta(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, BOLD_MAGENTA)
}

fn colorize_bold_cyan(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, BOLD_CYAN)
}

fn colorize_bold_white(luau: &Lua, text: String) -> LuaValueResult {
    colorize(luau, text, BOLD_WHITE)
}

fn stylize(luau: &Lua, text: String, style_code: &str) -> LuaValueResult {
    let styled = style_code.to_string() + &text + RESET;
    Ok(LuaValue::String(luau.create_string(&styled)?))
}

fn style_bold(luau: &Lua, text: String) -> LuaValueResult {
    stylize(luau, text, BOLD)
}

fn style_dim(luau: &Lua, text: String) -> LuaValueResult {
    stylize(luau, text, DIM)
}

fn style_underline(luau: &Lua, text: String) -> LuaValueResult {
    stylize(luau, text, UNDERLINE)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    let bold_colors = TableBuilder::create(luau)?
        .with_function_and_signature("black", colorize_bold_black, signatures::STD_IO_COLORS_BOLD_BLACK)?
        .with_function_and_signature("red", colorize_bold_red, signatures::STD_IO_COLORS_BOLD_RED)?
        .with_function_and_signature("green", colorize_bold_green, signatures::STD_IO_COLORS_BOLD_GREEN)?
        .with_function_and_signature("yellow", colorize_bold_yellow, signatures::STD_IO_COLORS_BOLD_YELLOW)?
        .with_function_and_signature("blue", colorize_bold_blue, signatures::STD_IO_COLORS_BOLD_BLUE)?
        .with_function_and_signature("magenta", colorize_bold_magenta, signatures::STD_IO_COLORS_BOLD_MAGENTA)?
        .with_function_and_signature("cyan", colorize_bold_cyan, signatures::STD_IO_COLORS_BOLD_CYAN)?
        .with_function_and_signature("white", colorize_bold_white, signatures::STD_IO_COLORS_BOLD_WHITE)?
        .build_readonly()?;

    let styles = TableBuilder::create(luau)?
        .with_function_and_signature("bold", style_bold, signatures::STD_IO_COLORS_STYLE_BOLD)?
        .with_function_and_signature("dim", style_dim, signatures::STD_IO_COLORS_STYLE_DIM)?
        .with_function_and_signature("underline", style_underline, signatures::STD_IO_COLORS_STYLE_UNDERLINE)?
        .build_readonly()?;

    let codes = TableBuilder::create(luau)?
        .with_value("RESET", RESET)?
        .with_value("BLACK", BLACK)?
        .with_value("RED", RED)?
        .with_value("GREEN", GREEN)?
        .with_value("YELLOW", YELLOW)?
        .with_value("BLUE", BLUE)?
        .with_value("MAGENTA", MAGENTA)?
        .with_value("CYAN", CYAN)?
        .with_value("WHITE", WHITE)?
        .with_value("BOLD_BLACK", BOLD_BLACK)?
        .with_value("BOLD_RED", BOLD_RED)?
        .with_value("BOLD_GREEN", BOLD_GREEN)?
        .with_value("BOLD_YELLOW", BOLD_YELLOW)?
        .with_value("BOLD_BLUE", BOLD_BLUE)?
        .with_value("BOLD_MAGENTA", BOLD_MAGENTA)?
        .with_value("BOLD_CYAN", BOLD_CYAN)?
        .with_value("BOLD_WHITE", BOLD_WHITE)?
        .with_value("BRIGHT_BLACK", BRIGHT_BLACK)?
        .with_value("BRIGHT_RED", BRIGHT_RED)?
        .with_value("BRIGHT_GREEN", BRIGHT_GREEN)?
        .with_value("BRIGHT_YELLOW", BRIGHT_YELLOW)?
        .with_value("BRIGHT_BLUE", BRIGHT_BLUE)?
        .with_value("BRIGHT_MAGENTA", BRIGHT_MAGENTA)?
        .with_value("BRIGHT_CYAN", BRIGHT_CYAN)?
        .with_value("BRIGHT_WHITE", BRIGHT_WHITE)?
        .with_value("BLACK_BG", BLACK_BG)?
        .with_value("RED_BG", RED_BG)?
        .with_value("GREEN_BG", GREEN_BG)?
        .with_value("YELLOW_BG", YELLOW_BG)?
        .with_value("BLUE_BG", BLUE_BG)?
        .with_value("MAGENTA_BG", MAGENTA_BG)?
        .with_value("CYAN_BG", CYAN_BG)?
        .with_value("WHITE_BG", WHITE_BG)?
        .with_value("BRIGHT_BLACK_BG", BRIGHT_BLACK_BG)?
        .with_value("BRIGHT_RED_BG", BRIGHT_RED_BG)?
        .with_value("BRIGHT_GREEN_BG", BRIGHT_GREEN_BG)?
        .with_value("BRIGHT_YELLOW_BG", BRIGHT_YELLOW_BG)?
        .with_value("BRIGHT_BLUE_BG", BRIGHT_BLUE_BG)?
        .with_value("BRIGHT_MAGENTA_BG", BRIGHT_MAGENTA_BG)?
        .with_value("BRIGHT_CYAN_BG", BRIGHT_CYAN_BG)?
        .with_value("BRIGHT_WHITE_BG", BRIGHT_WHITE_BG)?
        .with_value("BOLD", BOLD)?
        .with_value("DIM", DIM)?
        .with_value("UNDERLINE", UNDERLINE)?
        .build_readonly()?;

    TableBuilder::create(luau)?
        .with_function_and_signature("black", colorize_black, signatures::STD_IO_COLORS_BLACK)?
        .with_function_and_signature("red", colorize_red, signatures::STD_IO_COLORS_RED)?
        .with_function_and_signature("green", colorize_green, signatures::STD_IO_COLORS_GREEN)?
        .with_function_and_signature("yellow", colorize_yellow, signatures::STD_IO_COLORS_YELLOW)?
        .with_function_and_signature("blue", colorize_blue, signatures::STD_IO_COLORS_BLUE)?
        .with_function_and_signature("magenta", colorize_magenta, signatures::STD_IO_COLORS_MAGENTA)?
        .with_function_and_signature("cyan", colorize_cyan, signatures::STD_IO_COLORS_CYAN)?
        .with_function_and_signature("white", colorize_white, signatures::STD_IO_COLORS_WHITE)?
        .with_function_and_signature("rgb", rgb, signatures::STD_IO_COLORS_RGB)?
        .with_function_and_signature("override", colors_override, signatures::STD_IO_COLORS_OVERRIDE)?
        .with_function_and_signature("enabled", colors_enabled, signatures::STD_IO_COLORS_ENABLED)?
        .with_value("bold", bold_colors)?
        .with_value("style", styles)?
        .with_value("codes", codes)?
        .build_readonly()
}