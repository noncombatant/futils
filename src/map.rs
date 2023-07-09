// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils map` command.

use itertools::Itertools;

use crate::shell::{parse_options, FileOpener, Options, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::StreamSplitter;
use crate::util::{help, run_command};

/// Command line usage help.
pub(crate) const MAP_HELP: &str = include_str!("map_help.md");

pub(crate) const MAP_HELP_VERBOSE: &str = include_str!("map_help_verbose.md");

/// Iterates over `StreamSplitter` and runs each of the `commands` on each
/// record.
fn map(splitter: StreamSplitter, options: &Options) -> ShellResult {
    let mut status = 0;
    let chunk_size = match options.limit {
        Some(limit) => {
            if limit > 0 {
                limit as usize
            } else {
                1
            }
        }
        None => 1,
    };
    for chunk in splitter
        .map_while(Result::ok)
        .chunks(chunk_size)
        .into_iter()
    {
        // TODO: This is ugly and allocates.
        let records: Vec<Vec<u8>> = chunk.collect();
        let records: Vec<&[u8]> = records.iter().map(|r| r.as_slice()).collect();
        for command in &options.match_commands {
            match run_command(command, &records, true) {
                Ok(s) => {
                    if s != 0 {
                        status += 1;
                    }
                }
                Err(e) => {
                    eprintln!("{} ... : {}", command, e);
                    status += 1;
                }
            }
        }
    }
    Ok(status)
}

/// Runs the `map` command on `arguments`.
pub(crate) fn map_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(
            0,
            MAP_HELP,
            if options.verbose {
                Some(MAP_HELP_VERBOSE)
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
                match map(
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
