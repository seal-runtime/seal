use mluau::prelude::*;
use crate::prelude::*;

use crossterm::cursor::SetCursorStyle;

#[derive(Clone)]
pub enum CursorStyle {
    Default,
    BlinkingBlock,
    SteadyBlock,
    BlinkingUnderScore,
    SteadyUnderScore,
    BlinkingBar,
    SteadyBar
}
impl CursorStyle {
    pub fn from_luau(value: LuaValue, function_name: &'static str) -> LuaResult<Self> {
        let options = [
            "Default", 
            "BlinkingBlock", 
            "SteadyBlock",
            "BlinkingUnderScore",
            "SteadyUnderScore",
            "BlinkingBar",
            "SteadyBar"
        ];
        let valid_modes_for_display = options
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(", ");

        let style = match value {
            LuaValue::String(ref mode) if let Ok(mode) = mode.to_str() => {
                if mode.eq_ignore_ascii_case("Default") {
                    Self::Default
                } else if mode.eq_ignore_ascii_case("BlinkingBlock") {
                    Self::BlinkingBlock
                } else if mode.eq_ignore_ascii_case("SteadyBlock") {
                    Self::SteadyBlock
                } else if mode.eq_ignore_ascii_case("BlinkingUnderScore") {
                    Self::BlinkingUnderScore
                } else if mode.eq_ignore_ascii_case("SteadyUnderScore") {
                    Self::SteadyUnderScore
                } else if mode.eq_ignore_ascii_case("BlinkingBar") {
                    Self::BlinkingBar
                } else if mode.eq_ignore_ascii_case("SteadyBar") {
                    Self::SteadyBar
                } else {
                    return wrap_err!("{}: expected mode to be one of these valid modes: {}, got: \"{}\"", function_name, &valid_modes_for_display, &mode);
                }
            }
            LuaValue::String(_) => {
                return wrap_err!("{} expected mode to be valid utf-8; valid modes: {}", function_name, &valid_modes_for_display);
            },
            other => {
                return wrap_err!("{}: expected mode to be one of these valid modes: {}, got something else: {:?}", function_name, &valid_modes_for_display, &other);
            }
        };

        Ok(style)
    }

    pub fn into_crossterm(self) -> SetCursorStyle {
        match self {
            Self::Default => SetCursorStyle::DefaultUserShape,
            Self::BlinkingBlock => SetCursorStyle::BlinkingBlock,
            Self::SteadyBlock => SetCursorStyle::SteadyBlock,
            Self::BlinkingUnderScore => SetCursorStyle::BlinkingUnderScore,
            Self::SteadyUnderScore => SetCursorStyle::SteadyUnderScore,
            Self::BlinkingBar => SetCursorStyle::BlinkingBar,
            Self::SteadyBar => SetCursorStyle::SteadyBar,
        }
    }
}