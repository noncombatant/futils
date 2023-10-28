// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils filter` command.

use std::io::{stdout, Write};

use atty::Stream;
use itertools::Either;

use crate::enumerated_record::EnumeratedRecord;
use crate::shell::{parse_options, FileOpener, Options, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::StreamSplitter;
use crate::util::{help, run_command};

pub(crate) const FILTER_HELP: &str = include_str!("filter.md");
pub(crate) const FILTER_HELP_VERBOSE: &str = include_str!("filter_verbose.md");

fn print_matches(pathname: &str, splitter: StreamSplitter, options: &Options) -> ShellResult {
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
            n: if options.enumerate {
                Some(pair.0)
            } else {
                None
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
                    Err(e) => {
                        eprintln!("{} \"{}\": {}", command, String::from_utf8_lossy(&er.r), e);
                        return false;
                    }
                }
            }
            true
        })
    {
        if options.json_output {
            er.write_json(&mut stdout, atty::is(Stream::Stdout), options)?;
            stdout.write_all(b",\n")?;
        } else {
            er.write_columns(&mut stdout, options)?;
        }
    }
    Ok(if matched { 0 } else { 1 })
}

/// Runs the `filter` command on `arguments`.
pub(crate) fn filter_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(
            0,
            FILTER_HELP,
            true,
            if options.verbose {
                Some(FILTER_HELP_VERBOSE)
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
                let s = print_matches(
                    pathname,
                    StreamSplitter::new(&mut read, &options.input_record_delimiter),
                    &options,
                )?;
                if s != 0 {
                    status += 1;
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
