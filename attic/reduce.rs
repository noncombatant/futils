// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils reduce` command.

use crate::{
    shell::{FileOpener, Options, STDIN_PATHNAME, ShellResult, UsageError, parse_options},
    util::{exit_with_result, help, parse_number},
};
use regex_splitter::RegexSplitter;
use std::{
    error::Error,
    io::{Write, stdout},
    str::from_utf8,
};

/// Parses `value` and returns a `BigDecimal`.
pub fn parse_number(value: &[u8]) -> Result<BigDecimal, Box<dyn Error>> {
    let separator = match Numeric::load_user_locale() {
        Ok(numeric) => numeric.thousands_sep,
        Err(_) => String::new(),
    };
    let value = from_utf8(value)?;
    let value = value.replace(&separator, "");
    Ok(BigDecimal::from_str(&value)?)
}

pub const REDUCE_HELP: &str = include_str!("reduce.md");
pub const REDUCE_HELP_VERBOSE: &str = include_str!("reduce_verbose.md");

// TODO: Change this program to work on each field in each record, instead of
// each record. Or, make that an option. That way, you could sum each column in
// a row, for example. That may mean ignoring number parse errors, or making
// reporting them be optional.

/// Returns the result of applying `command` to `accumulator` and `record`.
fn apply_command(
    accumulator: &[u8],
    command: &str,
    record: &[u8],
) -> Result<Vec<u8>, Box<dyn Error>> {
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
            Ok(Vec::from(format!("{r}").as_bytes()))
        }
        _ => {
            // Run `command` with `accumulator` and `record` as its stdin.
            unimplemented!();
        }
    }
}

/// Iterates over `RegexSplitter` and runs each of the `commands` on each
/// record. `verbose` enables printing `stdout` from the `commands`. Each
/// record’s output is delimited by `output_delimiter`.
fn reduce(splitter: RegexSplitter, options: &Options) -> ShellResult {
    let mut stdout = stdout();
    let mut status = 0;
    let mut splitter = splitter.map_while(Result::ok);
    let Some(mut result) = splitter.next() else {
        return Err(UsageError::new("No input").into());
    };

    for record in splitter {
        for command in &options.match_commands {
            match apply_command(&result, command, &record) {
                Ok(r) => {
                    result = r;
                }
                Err(error) => {
                    eprintln!("{}: {error}", from_utf8(&record).unwrap());
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
pub fn reduce_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        exit_with_result(help(
            0,
            REDUCE_HELP,
            true,
            if options.verbose {
                Some(REDUCE_HELP_VERBOSE)
            } else {
                None
            },
        ));
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
                    RegexSplitter::new(&mut read, &options.input_record_delimiter),
                    &options,
                ) {
                    Ok(s) => status += s,
                    Err(error) => {
                        eprintln!("{pathname}: {error}");
                        status += 1;
                    }
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
