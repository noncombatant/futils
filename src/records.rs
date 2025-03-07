// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils records` command.

use crate::{
    enumerated_record::EnumeratedRecord,
    shell::{FileOpener, STDIN_PATHNAME, ShellResult, parse_options},
    util::{exit_with_result, help},
};
use itertools::Either;
use regex_splitter::RegexSplitter;
use std::io::{IsTerminal, Write, stdout};

pub const RECORDS_HELP: &str = include_str!("records.md");
pub const RECORDS_HELP_VERBOSE: &str = include_str!("records_verbose.md");

/// Runs the `records` command on `arguments`.
pub fn records_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        exit_with_result(help(
            0,
            RECORDS_HELP,
            true,
            if options.verbose {
                Some(RECORDS_HELP_VERBOSE)
            } else {
                None
            },
        ));
    }

    let mut status = 0;
    let mut stdout = stdout();
    for file in FileOpener::new(arguments) {
        let pathname = file.pathname.unwrap_or(&STDIN_PATHNAME);
        match file.read {
            Ok(mut read) => {
                let records = RegexSplitter::new(&mut read, &options.input_record_delimiter)
                    .map_while(Result::ok);
                let records = match options.limit {
                    Some(limit) => {
                        Either::Right(if limit >= 0 {
                            Either::Right(records.take(limit as usize))
                        } else {
                            // TODO: For best efficiency — not reading the
                            // entire stream first when not necessary — we will
                            // probably want to implement `DoubleEndedIterator`
                            // for `RegexSplitter`.
                            Either::Left(
                                records
                                    .collect::<Vec<Vec<u8>>>()
                                    .into_iter()
                                    .rev()
                                    .take(limit.unsigned_abs())
                                    .rev(),
                            )
                        })
                    }
                    None => Either::Left(records),
                };

                if options.json_output {
                    println!("[");
                }
                for er in records.enumerate().map(|pair| EnumeratedRecord {
                    n: if options.no_enumerate {
                        None
                    } else {
                        Some(pair.0)
                    },
                    pathname,
                    r: pair.1,
                }) {
                    if options.json_output {
                        let t = stdout.is_terminal();
                        er.write_json(&mut stdout, t, &options)?;
                        stdout.write_all(b",\n")?;
                    } else {
                        er.write_columns(&mut stdout, &options)?;
                    }
                }
                if options.json_output {
                    println!("{{}}]");
                }
            }
            Err(error) => {
                eprintln!("{pathname}: {error}");
                status += 1;
            }
        }
    }
    Ok(status)
}
