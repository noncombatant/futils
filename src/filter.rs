// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils filter` command.

use crate::{
    enumerated_record::EnumeratedRecord,
    shell::{FileOpener, Options, STDIN_PATHNAME, ShellResult, parse_options},
    util::{exit_with_result, help, run_command},
};
use itertools::Either;
use regex_splitter::RegexSplitter;
use std::io::{IsTerminal, Write, stdout};

pub const FILTER_HELP: &str = include_str!("filter.md");
pub const FILTER_HELP_VERBOSE: &str = include_str!("filter_verbose.md");

fn print_matches(pathname: &str, splitter: RegexSplitter, options: &Options) -> ShellResult {
    let mut stdout = stdout();
    let mut matched = false;
    let records = splitter.map_while(Result::ok);
    let records = match options.limit {
        Some(limit) => Either::Right(if limit > 0 {
            Either::Right(records.take(limit as usize))
        } else {
            Either::Left(records)
        }),
        None => Either::Left(records),
    };
    for er in records
        .enumerate()
        .map(|pair| EnumeratedRecord {
            n: if options.no_enumerate {
                None
            } else {
                Some(pair.0)
            },
            pathname,
            r: pair.1,
        })
        .filter(|er| {
            for re in &options.prune_expressions {
                if re.is_match(&er.r) {
                    return false;
                }
                matched = true;
                if options.limit == Some(0) {
                    return false;
                }
            }
            for re in &options.match_expressions {
                if !re.is_match(&er.r) {
                    return false;
                }
                matched = true;
                if options.limit == Some(0) {
                    return false;
                }
            }
            for command in &options.match_commands {
                match run_command(command, &[&er.r], options.verbose) {
                    Ok(status) => {
                        if status != 0 {
                            return false;
                        }
                        matched = true;
                        if options.limit == Some(0) {
                            return false;
                        }
                    }
                    Err(error) => {
                        eprintln!("{command} \"{}\": {error}", String::from_utf8_lossy(&er.r));
                        return false;
                    }
                }
            }
            true
        })
    {
        if options.json_output {
            let t = stdout.is_terminal();
            er.write_json(&mut stdout, t, options)?;
            stdout.write_all(b",\n")?;
        } else {
            er.write_columns(&mut stdout, options)?;
        }
    }
    Ok(i32::from(!matched))
}

/// Runs the `filter` command on `arguments`.
pub fn filter_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        exit_with_result(help(
            0,
            FILTER_HELP,
            true,
            if options.verbose {
                Some(FILTER_HELP_VERBOSE)
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
                let s = print_matches(
                    pathname,
                    RegexSplitter::new(&mut read, &options.input_record_delimiter),
                    &options,
                )?;
                if s != 0 {
                    status += 1;
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
