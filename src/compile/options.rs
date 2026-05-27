use std::collections::VecDeque;
use std::path::PathBuf;

use mluau::prelude::*;
use crate::prelude::*;

use crate::std_env;
use crate::Args;

pub struct CompileOptions {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub should_transform: bool,
}
impl CompileOptions {
    fn is_reserved_flag(arg: &str) -> bool {
        matches!(arg, "-i" | "--input" | "-o" | "--output" | "-n" | "--no-transform")
    }
    fn ensure_no_duplicate_flags(args: &VecDeque<String>, function_name: &'static str) -> LuaResult<()> {
        let input_flag_count = args
            .iter()
            .filter(|arg| matches!(arg.as_str(), "-i" | "--input"))
            .count();
        let output_flag_count = args
            .iter()
            .filter(|arg| matches!(arg.as_str(), "-o" | "--output"))
            .count();
        let no_transform_count = args
            .iter()
            .filter(|arg| matches!(arg.as_str(), "-n" | "--no-transform"))
            .count();

        if input_flag_count > 1 {
            return wrap_err!("{}: you accidentally specified the --input/-i flag more than once; this is likely a mistake", function_name);
        }
        if output_flag_count > 1 {
            return wrap_err!("{}: you accidentally specified the --output/-o flag more than once; this is likely a mistake", function_name);
        }
        if no_transform_count > 1 {
            return wrap_err!("{}: you accidentally specified the --no-transform/-n flag more than once; this is likely a mistake", function_name);
        }

        if let Some(first) = args.front()
            && !Self::is_reserved_flag(first.as_str())
            && input_flag_count >= 1
        {
            return wrap_err!("{}: you implicitly specified the --input/-i flag more than once (like {} input.luau -i input.luau); this is likely a mistake", function_name, function_name);
        }

        Ok(())
    }
    fn get_input_path(args: &VecDeque<String>, function_name: &'static str) -> LuaResult<Option<PathBuf>> {
        if args.is_empty() {
            return Ok(None);
        }

        let mut index = 0;
        while let Some(arg) = args.get(index) {
            match arg.as_str() {
                "-i" | "--input" => {
                    if let Some(next_arg) = args.get(index + 1)
                        && !Self::is_reserved_flag(next_arg.as_str())
                    {
                        return Ok(Some(PathBuf::from(next_arg)));
                    } else {
                        return wrap_err!("{} got an --input flag but no input path was specified", function_name);
                    }
                },
                "-o" | "--output" => {
                    // skip 2 spots to jump over output path
                    index += 2;
                },
                other if index == 0 && !Self::is_reserved_flag(other) => {
                    // for backwards compat we must support seal compile input_path
                    return Ok(Some(PathBuf::from(other)));
                },
                _ => {
                    index += 1;
                }
            }
        }

        Ok(None)
    }
    fn get_output_path(args: &VecDeque<String>, function_name: &'static str) -> LuaResult<Option<PathBuf>> {
        if args.is_empty() {
            return Ok(None);
        }

        let mut index = 0;
        while let Some(arg) = args.get(index) {
            match arg.as_str() {
                "-o" | "--output" => {
                    if let Some(next_arg) = args.get(index + 1)
                        && !Self::is_reserved_flag(next_arg.as_str())
                    {
                        return Ok(Some(PathBuf::from(next_arg)));
                    } else {
                        return wrap_err!("{} got --output flag but no output path was specified", function_name);
                    }
                },
                "-i" | "--input" => {
                    // jump 2 spots over input/entry path
                    index += 2;
                },
                _ => {
                    index += 1;
                }
            }
        }

        Ok(None)
    }
    fn should_transform(args: &VecDeque<String>) -> bool {
        for arg in args {
            match arg.as_str() {
                "--no-transform" | "-n" => return false,
                _ => continue,
            }
        }
        true
    }
    pub fn from_args(mut args: Args, function_name: &'static str) -> LuaResult<Self> {
        let new_args = {
            let mut res = VecDeque::<String>::with_capacity(args.len());
            let mut index = 0;
            while let Some(arg) = args.pop_front() {
                match arg.into_string() {
                    Ok(s) => res.push_back(s),
                    Err(_) => {
                        return wrap_err!("{}: argument at index {} contains invalid utf-8", function_name, index);
                    }
                }
                index += 1;
            }
            res
        };

        Self::ensure_no_duplicate_flags(&new_args, function_name)?;
        
        let default_entry_path = std_env::get_cwd(function_name)?;
        let default_output_path = match default_entry_path.file_name() {
            Some(basename) => match basename.to_str() {
                Some(basename) => PathBuf::from(basename.to_owned()),
                None => {
                    return wrap_err!("{}: your output path (which defaults to cwd's name) contains invalid utf-8", function_name);
                }
            },
            None => {
                return wrap_err!("{} - why can't we figure out the basename of your cwd???", function_name);
            }
        };

        let input_path = Self::get_input_path(&new_args, function_name)?
            .unwrap_or(default_entry_path);
        let output_path = Self::get_output_path(&new_args, function_name)?
            .unwrap_or(default_output_path);
        let should_transform = Self::should_transform(&new_args);

        if !should_transform && !input_path.is_file() {
            return wrap_err!("{}: seal cannot bundle an entire codebase when transformations are disabled, did you forget to pass an input file (seal compile --input ./some_file.luau)?", function_name);
        }

        Ok(Self {
            input_path,
            output_path,
            should_transform,
        })
    }
}
