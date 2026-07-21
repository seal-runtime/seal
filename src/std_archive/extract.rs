use std::borrow::Cow;
use std::fs;
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

/// Applies an entry's Unix permission bits to the file/directory just written to `path`,
/// if the source archive format recorded a `mode`. A no-op on non-Unix platforms, since
/// there's no equivalent permission bit layout to apply there.
fn apply_mode(path: &Path, mode: Option<u32>, function_name: &'static str) -> LuaEmptyResult {
    #[cfg(unix)]
    if let Some(mode) = mode {
        use std::os::unix::fs::PermissionsExt;
        if let Err(err) = fs::set_permissions(path, fs::Permissions::from_mode(mode)) {
            return wrap_io_read_errors_empty(err, function_name, path);
        }
    }
    #[cfg(not(unix))]
    let _ = mode;

    Ok(())
}

/// Applies an entry's `mtime` to the file/directory just written to `path`, if the source
/// archive format recorded one. Opening a directory via `fs::File::open` (rather than
/// `fs::OpenOptions::new().write(true)`, which directories reject) works on Unix; on Windows
/// it doesn't, so directory mtimes are best-effort there and silently skipped on failure.
fn apply_mtime(path: &Path, mtime: Option<SystemTime>, function_name: &'static str) -> LuaEmptyResult {
    let Some(mtime) = mtime else {
        return Ok(());
    };

    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) if path.is_dir() => return Ok(()),
        Err(err) => return wrap_io_read_errors_empty(err, function_name, path),
    };

    if let Err(err) = file.set_modified(mtime) {
        if path.is_dir() {
            return Ok(());
        }
        return wrap_io_read_errors_empty(err, function_name, path);
    }

    Ok(())
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
        Err(err) => {
            return wrap_err!("{}: unable to extract archive at '{}' due to err: {}", function_name, path_for_display(), err);
        }
    };

    Ok(entries)
}

pub fn write_to_disk<P: AsRef<Path>>(
    entries: &[ArchiveEntry],
    destination: P,
    options: ArchiveOptions,
    format: ArchiveFormat,
    function_name: &'static str
) -> LuaEmptyResult {
    let destination = destination.as_ref().to_path_buf();
    if format.is_single_file() {
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
