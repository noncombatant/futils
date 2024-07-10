// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils files` command.

use std::cmp::Ordering;
use std::io::{stdout, Write};
use std::time::SystemTime;

use chrono::NaiveDateTime;
use walkdir::{DirEntry, WalkDir};

use crate::shell::{parse_options, Options, ShellResult};
use crate::time::Time;
use crate::util::{exit_with_result, help, run_command};

pub const FILES_HELP: &str = include_str!("files.md");
pub const FILES_HELP_VERBOSE: &str = include_str!("files_verbose.md");

fn is_hidden(e: &DirEntry) -> bool {
    e.path().to_str().map_or(false, |s| s.contains("./"))
}

fn compare_times(e: &DirEntry, t: &Time) -> Result<bool, std::io::Error> {
    let metadata = e.metadata()?;
    let modified = metadata.modified()?;
    let modified = modified
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let modified = NaiveDateTime::from_timestamp_opt(modified.try_into().unwrap(), 0).unwrap();
    let given = t.date_time;
    Ok(match t.ordering {
        Ordering::Greater => given <= modified,
        Ordering::Less => given >= modified,
        Ordering::Equal => given == modified,
    })
}

fn print_matches(pathname: &str, options: &Options) -> ShellResult {
    let mut stdout = stdout();
    let mut it = if options.depth > 0 {
        WalkDir::new(pathname).max_depth(options.depth)
    } else {
        WalkDir::new(pathname)
    }
    .into_iter();
    let mut status = 0;

    'outer: loop {
        let entry = match it.next() {
            None => break Ok(status),
            Some(entry) => entry,
        };
        let entry = match entry {
            Err(error) => {
                eprintln!("{error}");
                status += 1;
                continue;
            }
            Ok(e) => e,
        };

        let file_type = entry.file_type();
        let is_dir = file_type.is_dir();
        let is_file = file_type.is_file();
        let is_symlink = file_type.is_symlink();
        if (is_dir && !options.file_types.contains('d'))
            || (is_file && !options.file_types.contains('f'))
            || (is_symlink && !options.file_types.contains('s'))
        {
            continue;
        }

        if !options.show_all && is_hidden(&entry) {
            if is_dir {
                it.skip_current_dir();
            }
            continue;
        }

        let p = entry.path();
        let pathname = if let Some(s) = p.to_str() {
            s
        } else {
            eprintln!("pathname not valid Unicode: '{}'", p.display());
            status += 1;
            continue;
        };

        for re in &options.prune_expressions {
            if re.is_match(pathname.as_bytes()) {
                if entry.file_type().is_dir() {
                    it.skip_current_dir();
                }
                continue 'outer;
            }
        }

        for re in &options.match_expressions {
            if !re.is_match(pathname.as_bytes()) {
                continue 'outer;
            }
        }

        for mtime in &options.mtime_expressions {
            match compare_times(&entry, mtime) {
                Ok(true) => continue,
                Ok(false) => continue 'outer,
                Err(error) => {
                    eprintln!("{error}");
                    status += 1;
                    continue 'outer;
                }
            }
        }

        for command in &options.match_commands {
            match run_command(command, &[pathname.as_bytes()], options.verbose) {
                Ok(status) => {
                    if status != 0 {
                        continue 'outer;
                    }
                }
                Err(error) => {
                    eprintln!("{command} \"{pathname}\": {error}");
                    status += 1;
                    continue 'outer;
                }
            }
        }

        stdout.write_all(pathname.as_bytes())?;
        stdout.write_all(&options.output_record_delimiter)?;
    }
}

/// Runs the `files` command on `arguments`.
pub fn files_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        exit_with_result(help(
            0,
            FILES_HELP,
            true,
            if options.verbose {
                Some(FILES_HELP_VERBOSE)
            } else {
                None
            },
        ));
    }
    if options.json_input || options.json_output {
        unimplemented!()
    }

    let mut pathnames = vec![".".to_string()];
    if !arguments.is_empty() {
        pathnames = arguments.into();
    }
    let mut status = 0;
    for pathname in pathnames {
        match print_matches(&pathname, &options) {
            Ok(print_status) => status += print_status,
            Err(error) => {
                eprintln!("{pathname}: {error}");
                status += 1;
            }
        }
    }
    Ok(status)
}
