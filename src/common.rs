// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils common` command.

use std::cmp::Ordering;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::process::exit;

use crate::shell::{parse_options, Options, ShellResult};
use crate::stream_splitter::StreamSplitter;
use crate::util::{exit_with_result, help, icmp};

pub const COMMON_HELP: &str = include_str!("common.md");
pub const COMMON_HELP_VERBOSE: &str = include_str!("common_verbose.md");

fn print(column: i8, field: &[u8], options: &Options) -> ShellResult {
    let mut out = stdout();
    match column {
        1 => (),
        2 => out.write_all(&options.output_field_delimiter)?,
        3 => {
            out.write_all(&options.output_field_delimiter)?;
            out.write_all(&options.output_field_delimiter)?;
        }
        _ => unreachable!(),
    }
    out.write_all(field)?;
    out.write_all(&options.output_record_delimiter)?;
    Ok(0)
}

/// Runs the `common` command on `arguments`.
pub fn common_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        exit_with_result(help(
            0,
            COMMON_HELP,
            true,
            if options.verbose {
                Some(COMMON_HELP_VERBOSE)
            } else {
                None
            },
        ));
    }
    if arguments.is_empty() || arguments.len() > 2 {
        match help(-1, COMMON_HELP, false, None) {
            Ok(status) => exit(status),
            Err(_) => exit(-1),
        }
    }

    let mut stdin = stdin();
    let mut file1 = File::open(&arguments[0])?;
    let mut file2: File;
    let records1: StreamSplitter;
    let records2: StreamSplitter;
    let delimiter = &options.input_record_delimiter;
    if arguments.len() == 2 {
        file2 = File::open(&arguments[1])?;
        records1 = StreamSplitter::new(&mut file1, delimiter);
        records2 = StreamSplitter::new(&mut file2, delimiter);
    } else {
        records1 = StreamSplitter::new(&mut stdin, delimiter);
        records2 = StreamSplitter::new(&mut file1, delimiter);
    }

    let mut records1 = records1.map_while(Result::ok);
    let mut records2 = records2.map_while(Result::ok);

    // Adapted from *Command-Line Rust* by Ken Youens-Clark, pp. 242 – 243.
    let mut record1 = records1.next();
    let mut record2 = records2.next();
    while record1.is_some() || record2.is_some() {
        match (&record1, &record2) {
            (Some(r1), Some(r2)) => match if options.insensitive {
                icmp(r1, r2)
            } else {
                r1.cmp(r2)
            } {
                Ordering::Equal => {
                    print(3, r1, &options)?;
                    record1 = records1.next();
                    record2 = records2.next();
                }
                Ordering::Less => {
                    print(1, r1, &options)?;
                    record1 = records1.next();
                }
                Ordering::Greater => {
                    print(2, r2, &options)?;
                    record2 = records2.next();
                }
            },
            (Some(r1), None) => {
                print(1, r1, &options)?;
                record1 = records1.next();
            }
            (None, Some(r2)) => {
                print(2, r2, &options)?;
                record2 = records2.next();
            }
            _ => (),
        }
    }
    Ok(0)
}
