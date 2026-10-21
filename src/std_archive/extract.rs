use std::borrow::Cow;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use mluau::prelude::*;
use crate::prelude::*;

use super::options::ArchiveOptions;

use archive::{ArchiveFormat, ArchiveEntry, ArchiveError};

use crate::std_fs::{
    file_size::FileSize,
    entry::wrap_io_read_errors_empty
};

/// Fixes RUSTSEC-2026-0367; strips setuid/setgid so an untrusted archive can't plant a setuid executable.
#[cfg(unix)]
const EXTRACTED_MODE_MASK: u32 = 0o1777;

// permission bits on unix system, noop on windows
fn set_mode_raw(path: &Path, mode: Option<u32>) -> io::Result<()> {
    #[cfg(unix)]
    if let Some(mode) = mode {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(mode & EXTRACTED_MODE_MASK))?;
    }
    #[cfg(not(unix))]
    let _ = mode;

    Ok(())
}

/// needs to be open for writes on windows or setting mtime fails with permission denied
/// directory mtimes are just skipped for rn, not worth fixing atm
fn set_mtime_raw(path: &Path, mtime: Option<SystemTime>) -> io::Result<()> {
    let Some(mtime) = mtime else {
        return Ok(());
    };

    let file = match fs::OpenOptions::new().write(true).open(path) {
        Ok(file) => file,
        Err(_) => match fs::File::open(path) {
            Ok(file) => file,
            Err(_) if path.is_dir() => return Ok(()),
            Err(err) => return Err(err),
        }
    };

    if let Err(err) = file.set_modified(mtime) {
        if path.is_dir() {
            return Ok(());
        }
        return Err(err);
    }

    Ok(())
}

pub(super) fn apply_mode(path: &Path, mode: Option<u32>, function_name: &'static str) -> LuaEmptyResult {
    match set_mode_raw(path, mode) {
        Ok(()) => Ok(()),
        Err(err) => wrap_io_read_errors_empty(err, function_name, path),
    }
}

pub(super) fn apply_mtime(path: &Path, mtime: Option<SystemTime>, function_name: &'static str) -> LuaEmptyResult {
    match set_mtime_raw(path, mtime) {
        Ok(()) => Ok(()),
        Err(err) => wrap_io_read_errors_empty(err, function_name, path),
    }
}

const UNSAFE_PATH_BULLETPOINTS: &str = "
This could mean the archive:
  1. Is malicious (path/symlink traversal attack)
  2. Was accidentally generated from the wrong directory
  3. Is not meant to be extracted to disk
 
If you trust this archive, pass ArchiveOptions.allow_unsafe_path_traversals = true
to read, write, or extract it.
";

pub fn contents(
    contents: Vec<u8>,
    path: Option<&str>,
    options: &ArchiveOptions,
    format: ArchiveFormat,
    function_name: &'static str
) -> LuaResult<Vec<ArchiveEntry>> {
    let extractor = options.extractor();
    let path_for_display = move || {
        match path {
            Some(path) => Cow::Owned(format!("at '{}'", path)),
            None => Cow::Borrowed("loaded from memory")
        }
    };

    let entries = match extractor.extract(&contents, format) {
        Ok(files) => files,
        Err(ArchiveError::FileTooLarge { path, size, limit }) => {
            let size = FileSize::from_bytes(size as u64);
            let limit = FileSize::from_bytes(limit as u64);
            if let Some(archive_path) = path {
                return wrap_err!("{}: file in archive {} at archive path '{}' (size {}) exceeds max_file_size ({}); see options to change defaults", function_name, path_for_display(), archive_path, size, limit);
            } else {
                return wrap_err!("{}: a file in the archive {} (size {}) exceeds max_file_size ({}); see options to change defaults", function_name, path_for_display(), size, limit);
            }
        },
        Err(ArchiveError::TotalSizeTooLarge { size, limit }) => {
            let size = FileSize::from_bytes(size as u64);
            let limit = FileSize::from_bytes(limit as u64);
            return wrap_err!("{}: archive (size {}) exceeds max_total_size ({}); see options to change defaults", function_name, size, limit);
        },
        Err(ArchiveError::AllocationFailed { size, source }) => {
            let size = FileSize::from_bytes(size as u64);
            return wrap_err!("{}: you don't have enough memory to extract archive {}\n   (failed to reserve {} due to err: {})", function_name, path_for_display(), size, source);
        },
        Err(ArchiveError::InvalidArchive(reason)) => {
            return wrap_err!("{}: archive {} is invalid or not of format {}: {}", function_name, path_for_display(), format.name(), reason);
        },
        Err(ArchiveError::UnsafePath(bad_path)) => {
            return wrap_err!("{}: Path/Symlink Traversal: archive {}\n \nArchive contains a path that, once extracted, will traverse outside the extraction directory:\nTraversing path: '{}'\n \n{}", function_name, path_for_display(), bad_path, UNSAFE_PATH_BULLETPOINTS);
        },
        Err(ArchiveError::InvalidCompressionLevel(reason)) => {
            return wrap_err!("{}: invalid CompressionLevel for this kind of archive/single-file format; {}", function_name, reason);
        },
        Err(err) => {
            return wrap_err!("{}: unable to extract archive at '{}' due to err: {}", function_name, path_for_display(), err);
        }
    };

    Ok(entries)
}

/// Reads `contents` (the compressed archive bytes) and writes its entries straight to disk
/// through `ArchiveExtractor::extract_streaming`, without ever collecting a `Vec<ArchiveEntry>`
pub fn stream_to_disk<P: AsRef<Path>>(
    contents: &[u8],
    path: Option<&str>,
    destination: P,
    options: &ArchiveOptions,
    format: ArchiveFormat,
    function_name: &'static str
) -> LuaEmptyResult {
    let destination = destination.as_ref().to_path_buf();
    let extractor = options.extractor();
    let path_for_display = move || {
        match path {
            Some(path) => Cow::Owned(format!("at '{}'", path)),
            None => Cow::Borrowed("loaded from memory")
        }
    };

    fn create_parent_if_needed(path: &Path) -> io::Result<()> {
        let Some(parent) = path.parent() else {
            return Ok(());
        };
        match fs::create_dir_all(parent) {
            Ok(()) => Ok(()),
            Err(err) if matches!(err.kind(), std::io::ErrorKind::AlreadyExists) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn unsafe_path_traversals_message(bad_path: String) -> String {
        format!("Path/Symlink Traversal:\n \nArchive contains a path that, once extracted, will traverse outside the extraction directory:\nTraversing path: '{}'\n \n{}", bad_path, UNSAFE_PATH_BULLETPOINTS)
    }

    // mirrors the validate_path_safety in write_to_disk, but raw (io::Error) so it can be
    // called from inside extract_streaming's callback, whose Result is archive::Result
    fn validate_path_safety(path: &Path, options: &ArchiveOptions) -> io::Result<()> {
        use std::io::Write;
        let validation = archive::path_safety::validate_path(path.to_string_lossy().as_ref(), false);
        if let Err(ArchiveError::UnsafePath(bad_path)) = validation {
            if options.allow_unsafe_path_traversals {
                let mut stderr = std::io::stderr().lock();
                if !colors::are_disabled() {
                    let _ = writeln!(stderr, "{}[WARN]{}{} writing to '{}' (ArchiveOptions.allow_unsafe_path_traversals enabled){}", colors::BOLD_YELLOW, colors::RESET, colors::YELLOW, bad_path, colors::RESET);
                } else {
                    let _ = writeln!(stderr, "[WARN] writing to '{}' (ArchiveOptions.allow_unsafe_path_traversals enabled)", bad_path);
                }
            } else {
                return Err(io::Error::other(unsafe_path_traversals_message(bad_path)));
            }
        }
        Ok(())
    }

    if format.is_single_file() {
        let result = extractor.extract_streaming(contents, format, |meta, reader| {
            let mut file = fs::File::create(&destination)?;
            io::copy(reader, &mut file)?;
            set_mode_raw(&destination, meta.mode)?;
            set_mtime_raw(&destination, meta.mtime)?;
            Ok(())
        });

        if let Err(err) = result {
            return translate_streaming_err(err, &path_for_display, format, function_name);
        }
        return Ok(());
    }

    // symlinks are created after every other entry is written; an attacker could otherwise smuggle a symlink 
    // in early and have a later "legitimate" entry write straight through it
    let mut symlinks: Vec<(PathBuf, String)> = Vec::new();

    // directory mtimes are applied after every entry is written, since writing a file inside
    // a directory bumps that directory's mtime right back
    let mut directory_mtimes: Vec<(PathBuf, SystemTime)> = Vec::new();

    let result = extractor.extract_streaming(contents, format, |meta, reader| {
        let entry_path = Path::new(&meta.path);
        validate_path_safety(entry_path, options)?;
        #[allow(clippy::disallowed_methods, reason = "\
            Case 1: With default options, the validate_path_safety call above ensures entry_path is not an absolute path\
            Case 2: if unsafe_path_traversals is ALLOWED, the user is purposely opting into path clobbering\
        ")]
        let full_path = destination.join(entry_path);

        if meta.is_dir {
            fs::create_dir_all(&full_path)?;
            set_mode_raw(&full_path, meta.mode)?;
            if let Some(mtime) = meta.mtime {
                directory_mtimes.push((full_path, mtime));
            }
        } else if meta.is_symlink {
            if !options.allow_symlinks {
                return Err(io::Error::other(format!(
                    "archive has internal symlink from {} -> {}; this is unusual...\n  pass options.allow_symlinks = true to extract symlinks",
                    meta.path, meta.symlink_target().unwrap_or_default()
                )).into());
            }
            symlinks.push((full_path, meta.symlink_target().unwrap_or_default().to_string()));
        } else {
            // fs::File::create can fail on some platforms if the parent path doesn't exist
            create_parent_if_needed(&full_path)?;
            let mut file = fs::File::create(&full_path)?;
            io::copy(reader, &mut file)?;
            set_mode_raw(&full_path, meta.mode)?;
            set_mtime_raw(&full_path, meta.mtime)?;
        }

        Ok(())
    });

    if let Err(err) = result {
        return translate_streaming_err(err, &path_for_display, format, function_name);
    }

    for (path, target) in symlinks {
        crate::std_fs::create_symlink(&path.to_string_lossy(), &target, function_name)?;
    }

    for (path, mtime) in directory_mtimes {
        apply_mtime(&path, Some(mtime), function_name)?;
    }

    Ok(())
}

fn translate_streaming_err<'a>(
    err: ArchiveError,
    path_for_display: &dyn Fn() -> Cow<'a, str>,
    format: ArchiveFormat,
    function_name: &'static str
) -> LuaEmptyResult {
    match err {
        ArchiveError::FileTooLarge { path, size, limit } => {
            let size = FileSize::from_bytes(size as u64);
            let limit = FileSize::from_bytes(limit as u64);
            if let Some(archive_path) = path {
                wrap_err!("{}: file in archive {} at archive path '{}' (size {}) exceeds max_file_size ({}); see options to change defaults", function_name, path_for_display(), archive_path, size, limit)
            } else {
                wrap_err!("{}: a file in the archive {} (size {}) exceeds max_file_size ({}); see options to change defaults", function_name, path_for_display(), size, limit)
            }
        },
        ArchiveError::TotalSizeTooLarge { size, limit } => {
            let size = FileSize::from_bytes(size as u64);
            let limit = FileSize::from_bytes(limit as u64);
            wrap_err!("{}: archive (size {}) exceeds max_total_size ({}); see options to change defaults", function_name, size, limit)
        },
        ArchiveError::AllocationFailed { size, source } => {
            let size = FileSize::from_bytes(size as u64);
            wrap_err!("{}: you don't have enough memory to extract archive {}\n   (failed to reserve {} due to err: {})", function_name, path_for_display(), size, source)
        },
        ArchiveError::InvalidArchive(reason) => {
            wrap_err!("{}: archive {} is invalid or not of format {}: {}", function_name, path_for_display(), format.name(), reason)
        },
        ArchiveError::UnsafePath(bad_path) => {
            wrap_err!("{}: Path/Symlink Traversal: archive {}\n \nArchive contains a path that, once extracted, will traverse outside the extraction directory:\nTraversing path: '{}'\n \n{}", function_name, path_for_display(), bad_path, UNSAFE_PATH_BULLETPOINTS)
        },
        ArchiveError::InvalidCompressionLevel(reason) => {
            wrap_err!("{}: invalid CompressionLevel for this kind of archive/single-file format; {}", function_name, reason)
        },
        err => {
            wrap_err!("{}: unable to extract archive at '{}' due to err: {}", function_name, path_for_display(), err)
        }
    }
}

pub fn write_to_disk<P: AsRef<Path>>(
    entries: &[ArchiveEntry],
    destination: P,
    options: ArchiveOptions,
    format: Option<ArchiveFormat>,
    function_name: &'static str
) -> LuaEmptyResult {
    let destination = destination.as_ref().to_path_buf();
    if let Some(format) = format && format.is_single_file() {
        let contents = if let Some(file) = entries.first()
            && let Some(contents) = file.data()
        {
            contents
        } else {
            return wrap_err!("{}: single file entry is empty", function_name);
        };

        if let Err(err) = fs::write(&destination, contents) {
            return wrap_io_read_errors_empty(err, function_name, &destination);
        }
        let file = &entries[0];
        apply_mode(&destination, file.mode(), function_name)?;
        apply_mtime(&destination, file.mtime(), function_name)?;
        return Ok(());
    }

    fn create_parent_if_needed(path: &PathBuf, function_name: &'static str) -> LuaEmptyResult {
        let Some(parent) = path.parent() else {
            return Ok(());
        };

        if let Err(err) = fs::create_dir_all(parent) {
            if matches!(err.kind(), std::io::ErrorKind::AlreadyExists) {
                return Ok(());
            }
            return wrap_io_read_errors_empty(err, function_name, path);
        }

        Ok(())
    }

    fn validate_path_safety(path: &Path, options: &ArchiveOptions, function_name: &'static str) -> LuaEmptyResult {
        let validation = archive::path_safety::validate_path(path.to_string_lossy().as_ref(), false);
        if let Err(ArchiveError::UnsafePath(path)) = validation {
            if options.allow_unsafe_path_traversals {
                if !colors::are_disabled() {
                    eputs!("{}[WARN]{}{} writing to '{}' (ArchiveOptions.allow_unsafe_path_traversals enabled){}", colors::BOLD_YELLOW, colors::RESET, colors::YELLOW, &path, colors::RESET)?;
                } else {
                    eputs!("[WARN] writing to '{}' (ArchiveOptions.allow_unsafe_path_traversals enabled)", &path)?;
                }
            } else {
                return wrap_err!("{}: Path/Symlink Traversal:\n \nArchive contains a path that, once extracted, will traverse outside the extraction directory:\nTraversing path: '{}'\n \n{}", function_name, path, UNSAFE_PATH_BULLETPOINTS);
            }
        }
        Ok(())
    }

    let mut symlinks: Option<Vec<&ArchiveEntry>> = if options.allow_symlinks {
        Some(Vec::new())
    } else {
        None
    };
    // directory mtimes are applied after every entry is written, since writing a file inside
    // a directory bumps that directory's mtime right back
    let mut directory_mtimes: Vec<(PathBuf, SystemTime)> = Vec::new();

    for entry in entries {
        let path = Path::new(entry.path());
        validate_path_safety(path, &options, function_name)?;
        #[allow(clippy::disallowed_methods, reason = "\
            Case 1: With default options, the validate_path_safety call above ensures entry_path is not an absolute path\
            Case 2: if unsafe_path_traversals is ALLOWED, the user is purposely opting into path clobbering\
        ")]
        let path = destination.join(path);

        match entry {
            ArchiveEntry::Directory { mode, mtime, .. } => {
                if let Err(err) = fs::create_dir_all(&path) {
                    return wrap_io_read_errors_empty(err, function_name, &path);
                }
                apply_mode(&path, *mode, function_name)?;
                if let Some(mtime) = mtime {
                    directory_mtimes.push((path, *mtime));
                }
            },
            ArchiveEntry::File { data, mode, mtime, .. } => {
                // fs::write can fail on some platforms if parent path not exist
                create_parent_if_needed(&path, function_name)?;

                if let Err(err) = fs::write(&path, data) {
                    return wrap_io_read_errors_empty(err, function_name, &path);
                }
                apply_mode(&path, *mode, function_name)?;
                apply_mtime(&path, *mtime, function_name)?;
            },
            symlink @ ArchiveEntry::Symlink { path, target, .. } => {
                if !options.allow_symlinks {
                    return wrap_err!("{}: archive has internal symlink from {} -> {}; this is unusual...\n  pass options.symlinks_allowed = true to extract symlinks", function_name, path, target);
                }
                // we handle symlinks after everything else is written
                let symlinks = symlinks.as_mut().expect("we know symlinks are allowed here");
                symlinks.push(symlink);
                continue;
            }
        }
    }

    if let Some(symlinks) = symlinks && !symlinks.is_empty() {
        for link in symlinks {
            let ArchiveEntry::Symlink { path, target, .. } = link else {
                unreachable!("all ArchiveEntries in symlinks vec should be symlinks");
            };
            crate::std_fs::create_symlink(path, target, function_name)?;
        }
    }

    for (path, mtime) in directory_mtimes {
        apply_mtime(&path, Some(mtime), function_name)?;
    }

    Ok(())
}
