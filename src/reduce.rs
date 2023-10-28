// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils reduce` command.

use std::io::{stdout, Write};
use std::str::from_utf8;

use crate::shell::{
    parse_options, FileOpener, Options, ShellError, ShellResult, UsageError, STDIN_PATHNAME,
};
use crate::stream_splitter::StreamSplitter;
use crate::util::{help, parse_number};

pub(crate) const REDUCE_HELP: &str = include_str!("reduce.md");
pub(crate) const REDUCE_HELP_VERBOSE: &str = include_str!("reduce_verbose.md");

// TODO: Change this program to work on each field in each record, instead of
// each record. Or, make that an option. That way, you could sum each column in
// a row, for example. That may mean ignoring number parse errors, or making
// reporting them be optional.

/// Returns the result of applying `command` to `accumulator` and `record`.
fn apply_command(accumulator: &[u8], command: &str, record: &[u8]) -> Result<Vec<u8>, ShellError> {
    match command {
        "+" | "-" | "*" | "/" => {
            let a = parse_number(accumulator)?;
            let b = parse_number(record)?;
            let r = match command {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => unreachable!(),
            };
            Ok(Vec::from(format!("{}", r).as_bytes()))
        }
        _ => {
            // Run `command` with `accumulator` and `record` as its stdin.
            unimplemented!();
        }
    }
}

/// Iterates over `StreamSplitter` and runs each of the `commands` on each
/// record. `verbose` enables printing `stdout` from the `commands`. Each
/// record’s output is delimited by `output_delimiter`.
fn reduce(splitter: StreamSplitter, options: &Options) -> ShellResult {
    let mut stdout = stdout();
    let mut status = 0;
    let mut splitter = splitter.map_while(Result::ok);
    let mut result = match splitter.next() {
        Some(r) => r,
        None => return Err(ShellError::Usage(UsageError::new("No input"))),
    };

    for r in splitter {
        for command in &options.match_commands {
            match apply_command(&result, command, &r) {
                Ok(r) => {
                    result = r;
                }
                Err(e) => {
                    eprintln!("{}: {}", from_utf8(&r).unwrap(), e);
                    status += 1;
                }
            }
        }
    }
    stdout.write_all(&result)?;
    stdout.write_all(&options.output_record_delimiter)?;
    Ok(status)
}

/// Runs the `reduce` command on `arguments`.
pub(crate) fn reduce_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(
            0,
            REDUCE_HELP,
            true,
            if options.verbose {
                Some(REDUCE_HELP_VERBOSE)
            } else {
                None
            },
        );
    }
    if options.json_input || options.json_output {
        unimplemented!()
    }

    let mut status = 0;
    for file in FileOpener::new(arguments) {
        let pathname = file.pathname.unwrap_or(&STDIN_PATHNAME);
        match file.read {
            Ok(mut read) => {
                match reduce(
                    StreamSplitter::new(&mut read, &options.input_record_delimiter),
                    &options,
                ) {
                    Ok(s) => status += s,
                    Err(e) => {
                        eprintln!("{}: {}", pathname, e);
                        status += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("{}: {}", pathname, e);
                status += 1;
            }
        }
    }
    Ok(status)
}
