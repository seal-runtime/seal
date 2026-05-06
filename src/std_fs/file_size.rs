use crate::prelude::*;
use mluau::prelude::*;

const KILOBYTE: u64 = 1_024;
const MEGABYTE: u64 = 1_024 * KILOBYTE;
const GIGABYTE: u64 = 1_024 * MEGABYTE;
const TERABYTE: u64 = 1_024 * GIGABYTE;

pub struct FileSize {
    inner_bytes: u64,
}
impl FileSize {
    pub fn from_bytes(count: u64) -> Self {
        Self { inner_bytes: count }
    }
    pub fn display(&self) -> String {
        let b = self.inner_bytes;
        if b < KILOBYTE {
            format!("{} bytes", b)
        } else if b < MEGABYTE {
            format!("{:.2} KB", b as f64 / KILOBYTE as f64)
        } else if b < GIGABYTE {
            format!("{:.2} MB", b as f64 / MEGABYTE as f64)
        } else if b < TERABYTE {
            format!("{:.2} GB", b as f64 / GIGABYTE as f64)
        } else {
            format!("{:.2} TB", b as f64 / TERABYTE as f64)
        }
    }
    pub fn into_userdata(self, luau: &Lua) -> LuaValueResult {
        ok_userdata(self, luau)
    }
}

impl LuaUserData for FileSize {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "FileSize");
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Add, |luau, this, other: LuaValue| {
            let function_name = "FileSize.__add";
            match other {
                LuaValue::UserData(ud) if let Ok(other) = ud.borrow::<FileSize>() => {
                    FileSize::from_bytes(this.inner_bytes + other.inner_bytes).into_userdata(luau)
                }
                LuaValue::UserData(ud) => {
                    wrap_err!("{}: expected another FileSize to add, got an unexpected userdata type {:?}", function_name, ud)
                }
                other => {
                    wrap_err!("{}: expected another FileSize to add, got {:?}", function_name, other)
                }
            }
        });
        methods.add_meta_method(LuaMetaMethod::Sub, |luau, this, other: LuaValue| {
            let function_name = "FileSize.__sub";
            match other {
                LuaValue::UserData(ud) if let Ok(other) = ud.borrow::<FileSize>() => {
                    if other.inner_bytes > this.inner_bytes {
                        return wrap_err!("{}: subtraction would underflow (result would be negative)", function_name);
                    }
                    FileSize::from_bytes(this.inner_bytes - other.inner_bytes).into_userdata(luau)
                }
                LuaValue::UserData(ud) => {
                    wrap_err!("{}: expected another FileSize to subtract, got an unexpected userdata type {:?}", function_name, ud)
                }
                other => {
                    wrap_err!("{}: expected another FileSize to subtract, got {:?}", function_name, other)
                }
            }
        });
        methods.add_meta_method(LuaMetaMethod::Lt, |_luau, this, other: LuaValue| {
            let function_name = "FileSize.__lt";
            match other {
                LuaValue::UserData(ud) if let Ok(other) = ud.borrow::<FileSize>() => {
                    Ok(LuaValue::Boolean(this.inner_bytes < other.inner_bytes))
                }
                LuaValue::UserData(ud) => {
                    wrap_err!("{}: expected another FileSize to compare, got an unexpected userdata type {:?}", function_name, ud)
                }
                other => {
                    wrap_err!("{}: expected another FileSize to compare, got {:?}", function_name, other)
                }
            }
        });
        methods.add_meta_method(LuaMetaMethod::Le, |_luau, this, other: LuaValue| {
            let function_name = "FileSize.__le";
            match other {
                LuaValue::UserData(ud) if let Ok(other) = ud.borrow::<FileSize>() => {
                    Ok(LuaValue::Boolean(this.inner_bytes <= other.inner_bytes))
                }
                LuaValue::UserData(ud) => {
                    wrap_err!("{}: expected another FileSize to compare, got an unexpected userdata type {:?}", function_name, ud)
                }
                other => {
                    wrap_err!("{}: expected another FileSize to compare, got {:?}", function_name, other)
                }
            }
        });
        methods.add_meta_method(LuaMetaMethod::Eq, |_luau, this, other: LuaValue| {
            let function_name = "FileSize.__eq";
            match other {
                LuaValue::UserData(ud) if let Ok(other) = ud.borrow::<FileSize>() => {
                    Ok(LuaValue::Boolean(this.inner_bytes == other.inner_bytes))
                }
                LuaValue::UserData(_) => {
                    Ok(LuaValue::Boolean(false))
                }
                other => {
                    wrap_err!("{}: expected another FileSize to compare, got {:?}", function_name, other)
                }
            }
        });
        methods.add_meta_method(LuaMetaMethod::ToString, |luau, this, _: ()| {
            ok_string(format!("FileSize<{}>", this.display()), luau)
        });
        methods.add_method("display", |luau, this, _: ()| {
            ok_string(this.display(), luau)
        });
        methods.add_method("format", |luau, this, unit: LuaValue| {
            let function_name = "FileSize:format(unit: FileSizeUnit)";
            match unit {
                LuaValue::String(s) => {
                    let formatted = match s.as_bytes().as_ref() {
                        b"Bytes" => format!("{} B", this.inner_bytes),
                        b"KB" => format!("{:.2} KB", this.inner_bytes as f64 / KILOBYTE as f64),
                        b"MB" => format!("{:.2} MB", this.inner_bytes as f64 / MEGABYTE as f64),
                        b"GB" => format!("{:.2} GB", this.inner_bytes as f64 / GIGABYTE as f64),
                        b"TB" => format!("{:.2} TB", this.inner_bytes as f64 / TERABYTE as f64),
                        other => {
                            return wrap_err!("{}: unknown unit {:?}, expected \"Bytes\", \"KB\", \"MB\", \"GB\", or \"TB\"", function_name, other);
                        }
                    };
                    ok_string(formatted, luau)
                }
                other => {
                    wrap_err!("{}: expected a unit string, got {:?}", function_name, other)
                }
            }
        });
        methods.add_method("as_bytes", |_luau, this, _: ()| {
            Ok(LuaValue::Integer(this.inner_bytes as i64))
        });
        methods.add_method("as_kilobytes", |_luau, this, _: ()| {
            Ok(LuaValue::Number(this.inner_bytes as f64 / KILOBYTE as f64))
        });
        methods.add_method("as_megabytes", |_luau, this, _: ()| {
            Ok(LuaValue::Number(this.inner_bytes as f64 / MEGABYTE as f64))
        });
        methods.add_method("as_gigabytes", |_luau, this, _: ()| {
            Ok(LuaValue::Number(this.inner_bytes as f64 / GIGABYTE as f64))
        });
        methods.add_method("as_terabytes", |_luau, this, _: ()| {
            Ok(LuaValue::Number(this.inner_bytes as f64 / TERABYTE as f64))
        });
    }
}

/// Multiply a float unit count by a byte scale factor before truncating to u64.
/// This lets fractional unit counts like 1.2 gigabytes work correctly by doing
/// the multiplication at f64 precision rather than truncating first.
fn float_scaled_to_bytes(f: f64, unit: u64, function_name: &'static str, parameter_name: &'static str) -> LuaResult<u64> {
    if f.is_nan() || f.is_infinite() {
        return wrap_err!("{}: {} cannot be NaN nor infinite", function_name, parameter_name);
    } else if f.is_sign_negative() {
        return wrap_err!("{}: {} cannot be negative (got: {})", function_name, parameter_name, f);
    }
    let scaled = f * unit as f64;
    if scaled > u64::MAX as f64 {
        return wrap_err!("{}: {} is too large to fit in a FileSize (got: {})", function_name, parameter_name, f);
    }
    // SAFETY: checked nan/infinite/negative/size above
    Ok(unsafe { scaled.trunc().to_int_unchecked() })
}

fn filesize_bytes(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "filesize.bytes(count: number)";
    let bytes = match value {
        LuaValue::Number(f) => float_to_u64(f, function_name, "count")?,
        LuaValue::Integer(i) => int_to_u64(i, function_name, "count")?,
        other => {
            return wrap_err!("{}: expected 'count' to be a whole number, got {:?}", function_name, other);
        }
    };
    FileSize::from_bytes(bytes).into_userdata(luau)
}

fn filesize_kilobytes(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "filesize.kilobytes(count: number)";
    let bytes = match value {
        LuaValue::Number(f) => float_scaled_to_bytes(f, KILOBYTE, function_name, "count")?,
        LuaValue::Integer(i) => int_to_u64(i, function_name, "count")? * KILOBYTE,
        other => {
            return wrap_err!("{}: expected 'count' to be a number, got {:?}", function_name, other);
        }
    };
    FileSize::from_bytes(bytes).into_userdata(luau)
}

fn filesize_megabytes(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "filesize.megabytes(count: number)";
    let bytes = match value {
        LuaValue::Number(f) => float_scaled_to_bytes(f, MEGABYTE, function_name, "count")?,
        LuaValue::Integer(i) => int_to_u64(i, function_name, "count")? * MEGABYTE,
        other => {
            return wrap_err!("{}: expected 'count' to be a number, got {:?}", function_name, other);
        }
    };
    FileSize::from_bytes(bytes).into_userdata(luau)
}

fn filesize_gigabytes(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "filesize.gigabytes(count: number)";
    let bytes = match value {
        LuaValue::Number(f) => float_scaled_to_bytes(f, GIGABYTE, function_name, "count")?,
        LuaValue::Integer(i) => int_to_u64(i, function_name, "count")? * GIGABYTE,
        other => {
            return wrap_err!("{}: expected 'count' to be a number, got {:?}", function_name, other);
        }
    };
    FileSize::from_bytes(bytes).into_userdata(luau)
}

fn filesize_terabytes(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "filesize.terabytes(count: number)";
    let bytes = match value {
        LuaValue::Number(f) => float_scaled_to_bytes(f, TERABYTE, function_name, "count")?,
        LuaValue::Integer(i) => int_to_u64(i, function_name, "count")? * TERABYTE,
        other => {
            return wrap_err!("{}: expected 'count' to be a number, got {:?}", function_name, other);
        }
    };
    FileSize::from_bytes(bytes).into_userdata(luau)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("bytes", filesize_bytes)?
        .with_function("kilobytes", filesize_kilobytes)?
        .with_function("megabytes", filesize_megabytes)?
        .with_function("gigabytes", filesize_gigabytes)?
        .with_function("terabytes", filesize_terabytes)?
        .build_readonly()
}
