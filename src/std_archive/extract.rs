use std::fs;
use std::path::{Path, PathBuf};

use mluau::prelude::*;
use rustyline::completion::Candidate;
use crate::prelude::*;

use super::options::ArchiveOptions;

use archive::{ArchiveFormat, ArchiveEntry, ArchiveError};

use crate::std_fs::{
    file_size::FileSize,
    entry::wrap_io_read_errors_empty,
    validate_path,
};

pub fn from_file(
    multivalue: &mut LuaMultiValue,
    format: ArchiveFormat,
    function_name: &'static str
) -> LuaEmptyResult {
    let path = match multivalue.pop_front() {
        Some(LuaValue::String(path)) => validate_path(&path, function_name)?,
        Some(LuaNil) | None => {
            return wrap_err!("{}: expected path to be a string or Pathlike, got nothing or nil", function_name);
        }
        Some(other) => {
            return wrap_err!("{}: expected path to be a string or Pathlike, got {:?}", function_name, other);
        }
    };

    let destination = match multivalue.pop_front() {
        Some(LuaValue::String(path)) => validate_path(&path, function_name)?,
        Some(LuaNil) | None => {
            return wrap_err!("{}: expected destination to be a string or Pathlike, got nothing or nil", function_name);
        }
        Some(other) => {
            return wrap_err!("{}: expected destination to be a string or Pathlike, got {:?}", function_name, other);
        }
    };

    let options = multivalue.pop_front().unwrap_or(LuaNil);
    let options = ArchiveOptions::from_value(options, function_name)?;

    let contents = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(err) => {
            return wrap_io_read_errors_empty(err, function_name, &path);
        }
    };

    let entries = super::extract::contents(
        contents,
        &path,
        &options,
        format,
        function_name
    )?;

    super::extract::write_to_disk(
        &entries,
        destination,
        options,
        format,
        function_name
    )?;

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
    path: &str,
    options: &ArchiveOptions,
    format: ArchiveFormat,
    function_name: &'static str
) -> LuaResult<Vec<ArchiveEntry>> {
    let extractor = options.extractor();
    let entries = match extractor.extract(&contents, format) {
        Ok(files) => files,
        Err(ArchiveError::FileTooLarge { path, size, limit }) => {
            let size = FileSize::from_bytes(size as u64);
            let limit = FileSize::from_bytes(limit as u64);
            if let Some(path) = path {
                return wrap_err!("{}: file in archive at path {} (size {}) exceeds max_file_size ({}); see options to change defaults", function_name, path, size, limit);
            } else {
                return wrap_err!("{}: a file in the archive (size {}) exceeds max_file_size ({}); see options to change defaults", function_name, size, limit);
            }
        },
        Err(ArchiveError::TotalSizeTooLarge { size, limit }) => {
            let size = FileSize::from_bytes(size as u64);
            let limit = FileSize::from_bytes(limit as u64);
            return wrap_err!("{}: archive (size {}) exceeds max_total_size ({}); see options to change defaults", function_name, size, limit);
        },
        Err(ArchiveError::AllocationFailed { size, source }) => {
            let size = FileSize::from_bytes(size as u64);
            return wrap_err!("{}: you don't have enough memory to extract {}\n   (failed to reserve {} due to err: {})", function_name, path, size, source);
        },
        Err(ArchiveError::InvalidArchive(reason)) => {
            return wrap_err!("{}: archive at '{}' is invalid or not of format {}: {}", function_name, &path, format.name(), reason);
        },
        Err(ArchiveError::UnsafePath(bad_path)) => {
            return wrap_err!("{}: Path/Symlink Traversal: '{}'\n \nArchive contains a path that, once extracted, will traverse outside the extraction directory:\nTraversing path: '{}'\n \n{}", function_name, &path, bad_path, UNSAFE_PATH_BULLETPOINTS);
        },
        Err(err) => {
            return wrap_err!("{}: unable to extract archive at '{}' due to err: {}", function_name, &path, err);
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
                return wrap_err!("{}: Path/Symlink Traversal:\n \nArchive contains a path that, once extracted, will traverse outside the extraction directory:\nTraversing path: '{}'\n \n{}", function_name, path.display(), UNSAFE_PATH_BULLETPOINTS);
            }
        }
        Ok(())
    }

    for entry in entries {
        let path = Path::new(entry.path());
        let path = destination.join(path);

        validate_path_safety(&path, &options, function_name)?;

        match entry {
            ArchiveEntry::Directory { .. } => {
                if let Err(err) = fs::create_dir_all(&path) {
                    return wrap_io_read_errors_empty(err, function_name, &path);
                }
            },
            ArchiveEntry::File { data, .. } => {
                // fs::write can fail on some platforms if parent path not exist
                create_parent_if_needed(&path, function_name)?;

                if let Err(err) = fs::write(&path, data) {
                    return wrap_io_read_errors_empty(err, function_name, &path);
                }
            },
            ArchiveEntry::Symlink { path, target, .. } => {
                if !options.allow_symlinks {
                    return wrap_err!("{}: archive has symlink from {} -> {}; this is unusual and could be malicious...\n  pass options.symlinks_allowed = true to extract symlinks", function_name, path, target);
                }
            }
        }
    }

    Ok(())
}
