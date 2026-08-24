use mluau::prelude::*;
use crate::{prelude::*, userdata::{SealUserData, SealUserDataFields, SealUserDataMethods}};
use std::borrow::Cow;

use archive::{ArchiveFormat, CompressionLevel, ZipCompression};

/// Wraps [`archive::CompressionLevel`] for `@std/archive/*` and `@std/serde/*` libs backed by archive crate
#[derive(Debug, Clone, Copy)]
pub struct ArchiveCompressionLevel(CompressionLevel);

impl ArchiveCompressionLevel {
    pub fn inner(&self) -> CompressionLevel {
        self.0
    }

    pub fn constructor_for_format(luau: &Lua, format: ArchiveFormat) -> LuaResult<Option<LuaFunction>> {
        let (make, sig): (fn(&Lua, LuaMultiValue) -> LuaValueResult, &'static std::ffi::CStr) = match format {
            ArchiveFormat::Zip => (zip_compression, signatures::STD_ARCHIVE_ZIP_COMPRESSION),
            ArchiveFormat::TarGz => (gzip_compression, signatures::STD_ARCHIVE_TAR_GZ_COMPRESSION),
            ArchiveFormat::TarBz2 => (bzip2_compression, signatures::STD_ARCHIVE_TAR_BZ2_COMPRESSION),
            ArchiveFormat::TarXz => (xz_compression, signatures::STD_ARCHIVE_TAR_XZ_COMPRESSION),
            ArchiveFormat::TarZst => (zstd_compression, signatures::STD_ARCHIVE_TAR_ZST_COMPRESSION),
            ArchiveFormat::TarLz4 => (lz4_compression, signatures::STD_ARCHIVE_TAR_LZ4_COMPRESSION),
            ArchiveFormat::Tar
            | ArchiveFormat::Ar
            | ArchiveFormat::Deb
            | ArchiveFormat::SevenZ
            | ArchiveFormat::Gz
            | ArchiveFormat::Bz2
            | ArchiveFormat::Xz
            | ArchiveFormat::Zst
            | ArchiveFormat::Lz4 => return Ok(None),
        };

        Ok(Some(luau.create_function_with_debug(make, Some(sig))?))
    }

    fn tostring(luau: &Lua, this: &ArchiveCompressionLevel, _: LuaValue) -> LuaValueResult {
        ok_string(format!("{}", this.0), luau)
    }

    fn name(luau: &Lua, this: &ArchiveCompressionLevel, _: LuaValue) -> LuaValueResult {
        this.0.name().into_lua(luau)
    }

    fn level(luau: &Lua, this: &ArchiveCompressionLevel, _: LuaValue) -> LuaValueResult {
        this.0.level().into_lua(luau)
    }
}

impl BorrowableMut for ArchiveCompressionLevel {}

impl SealUserData for ArchiveCompressionLevel {
    fn type_name<'a>() -> Cow<'a, str> {
        Cow::Borrowed("ArchiveCompressionLevel")
    }

    fn add_fields<F: SealUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "CompressionLevel");
    }
    fn add_methods<M: SealUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |luau, acl, val: LuaValue| {
            Self::tostring(luau, &acl, val)
        });
        methods.add_method("name", |luau, acl, val: LuaValue| {
            Self::name(luau, &acl, val)
        });
        methods.add_method("level", |luau, acl, val: LuaValue| {
            Self::level(luau, &acl, val)
        });
    }
}

fn zip_compression(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "zip.compression(mode: \"Store\" | \"Deflate\", level: number?)";

    let mode = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{}: called without required argument 'mode' (expected \"Store\" or \"Deflate\")", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected mode to be \"Store\" or \"Deflate\", got: {:?}", function_name, other);
        }
    };

    let level = match multivalue.pop_front() {
        Some(LuaValue::Integer(i)) => Some(int_to_u32(i, function_name, "level")?),
        Some(LuaValue::Number(f)) => Some(float_to_u32(f, function_name, "level")?),
        Some(LuaNil) | None => None,
        Some(other) => {
            return wrap_err!("{}: expected level to be a number or nil, got: {:?}", function_name, other);
        }
    };

    let zip_compression = if mode.eq_ignore_ascii_case("store") {
        if let Some(level) = level {
            return wrap_err!("{}: level is only applicable when mode is \"Deflate\", got mode \"Store\" with level {}", function_name, level);
        }
        ZipCompression::Stored
    } else if mode.eq_ignore_ascii_case("deflate") {
        ZipCompression::Deflated(level.unwrap_or(6))
    } else {
        return wrap_err!("{}: expected mode to be \"Store\" or \"Deflate\", got: \"{}\"", function_name, mode);
    };

    ArchiveCompressionLevel(CompressionLevel::Zip(zip_compression)).into_userdata(luau)
}

macro_rules! numeric_compression_fn {
    ($fn_name:ident, $lib_name:literal, $variant:ident, $ty:ty, $convert_int:path, $convert_float:path) => {
        fn $fn_name(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
            let function_name = concat!($lib_name, ".compression(level: number)");
            let level: $ty = match multivalue.pop_front() {
                Some(LuaValue::Integer(i)) => $convert_int(i, function_name, "level")?,
                Some(LuaValue::Number(f)) => $convert_float(f, function_name, "level")?,
                Some(LuaNil) | None => {
                    return wrap_err!("{}: called without required argument 'level'", function_name);
                },
                Some(other) => {
                    return wrap_err!("{}: expected level to be a number, got: {:?}", function_name, other);
                }
            };
            ArchiveCompressionLevel(CompressionLevel::$variant(level)).into_userdata(luau)
        }
    };
}

numeric_compression_fn!(gzip_compression, "tar.gz", Gzip, u32, int_to_u32, float_to_u32);
numeric_compression_fn!(bzip2_compression, "tar.bz2", Bzip2, u32, int_to_u32, float_to_u32);
numeric_compression_fn!(xz_compression, "tar.xz", Xz, u32, int_to_u32, float_to_u32);
numeric_compression_fn!(lz4_compression, "tar.lz4", Lz4, u32, int_to_u32, float_to_u32);
numeric_compression_fn!(zstd_compression, "tar.zst", Zstd, i32, int_to_i32, float_to_i32);
