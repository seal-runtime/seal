use mluau::prelude::*;
use crate::prelude::*;

use archive::ArchiveFormat;
use pastey::paste;

// me: seal doesn't use advanced rust features
// also me:

macro_rules! marco {
    ($lib:ident, $format:expr) => {
        paste! {
            pub struct [<$lib:upper_camel>];
            impl [<$lib:upper_camel>] {
                fn extract(_luau: &Lua, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
                    let function_name = concat!(stringify!([<$lib:lower>]), ".extract(path: Pathlike, destination: Pathlike, options: ArchiveOptions?)");
                    super::archive_extract(&mut multivalue, $format, function_name)
                }
                fn readfile(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
                    let function_name = concat!(stringify!([<$lib:lower>]), ".readfile(path: Pathlike, options: ArchiveOptions?)");
                    super::archive_readfile(luau, &mut multivalue, $format, function_name)
                }
                fn writefile(_luau: &Lua, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
                    let function_name = concat!(stringify!([<$lib:lower>]), ".writefile(path: Pathlike, archive: Archive, options: ArchiveOptions?)");
                    super::archive_writefile(&mut multivalue, $format, function_name)
                }
                fn load(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
                    let function_name = concat!(stringify!([<$lib:lower>]), ".load(bytes: buffer, options: ArchiveOptions?)");
                    super::archive_load(luau, &mut multivalue, $format, function_name)
                }
                pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
                    TableBuilder::create(luau)?
                        .with_function_and_signature("extract", Self::extract, signatures::[<STD_ARCHIVE_ $lib:upper _EXTRACT>])?
                        .with_function_and_signature("readfile", Self::readfile, signatures::[<STD_ARCHIVE_ $lib:upper _READFILE>])?
                        .with_function_and_signature("writefile", Self::writefile, signatures::[<STD_ARCHIVE_ $lib:upper _WRITEFILE>])?
                        .with_function_and_signature("load", Self::load, signatures::[<STD_ARCHIVE_ $lib:upper _LOAD>])?
                        .with_function_and_signature("create", super::archive_create, signatures::[<STD_ARCHIVE_ $lib:upper _CREATE>])?
                        .build_readonly()
                }
            }
        }
    };
}

marco!(zip, ArchiveFormat::Zip);
marco!(ar, ArchiveFormat::Ar);
marco!(deb, ArchiveFormat::Deb);
marco!(sevenz, ArchiveFormat::SevenZ);

macro_rules! tars {
    ($lib:ident, $format:expr) => {
        paste! {
            struct [<$lib:upper_camel>];
            impl [<$lib:upper_camel>] {
                fn extract(_luau: &Lua, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
                    let function_name = concat!("tar.", stringify!([<$lib:lower>]), ".extract(path: Pathlike, destination: Pathlike, options: ArchiveOptions?)");
                    super::archive_extract(&mut multivalue, $format, function_name)
                }
                fn readfile(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
                    let function_name = concat!("tar.", stringify!([<$lib:lower>]), ".readfile(path: Pathlike, options: ArchiveOptions?)");
                    super::archive_readfile(luau, &mut multivalue, $format, function_name)
                }
                fn writefile(_luau: &Lua, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
                    let function_name = concat!("tar.", stringify!([<$lib:lower>]), ".writefile(path: Pathlike, archive: Archive, options: ArchiveOptions?)");
                    super::archive_writefile(&mut multivalue, $format, function_name)
                }
                fn load(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
                    let function_name = concat!("tar.", stringify!([<$lib:lower>]), ".load(bytes: buffer, options: ArchiveOptions?)");
                    super::archive_load(luau, &mut multivalue, $format, function_name)
                }
                pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
                    TableBuilder::create(luau)?
                        .with_function_and_signature("extract", Self::extract, signatures::[<STD_ARCHIVE_TAR_ $lib:upper _EXTRACT>])?
                        .with_function_and_signature("readfile", Self::readfile, signatures::[<STD_ARCHIVE_TAR_ $lib:upper _READFILE>])?
                        .with_function_and_signature("writefile", Self::writefile, signatures::[<STD_ARCHIVE_TAR_ $lib:upper _WRITEFILE>])?
                        .with_function_and_signature("load", Self::load, signatures::[<STD_ARCHIVE_TAR_ $lib:upper _LOAD>])?
                        .with_function_and_signature("create", super::archive_create, signatures::[<STD_ARCHIVE_TAR_ $lib:upper _CREATE>])?
                        .build_readonly()
                }
            }
        }
    };
}

tars!(gz, ArchiveFormat::TarGz);
tars!(uncompressed, ArchiveFormat::Tar);
tars!(xz, ArchiveFormat::TarXz);
tars!(lz4, ArchiveFormat::TarLz4);
tars!(bz2, ArchiveFormat::TarBz2);
tars!(zst, ArchiveFormat::TarZst);

pub struct Tar;
impl Tar {
    pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
        TableBuilder::create(luau)?
            .with_value("gz", Gz::create(luau)?)?
            .with_value("uncompressed", Uncompressed::create(luau)?)?
            .with_value("xz", Xz::create(luau)?)?
            .with_value("lz4", Lz4::create(luau)?)?
            .with_value("bz2", Bz2::create(luau)?)?
            .with_value("zst", Zst::create(luau)?)?
            .build_readonly()
    }
}


