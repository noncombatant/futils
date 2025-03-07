// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils map` command.

use crate::{
    shell::{FileOpener, Options, STDIN_PATHNAME, ShellResult, parse_options},
    util::{exit_with_result, help, run_command},
};
use itertools::Itertools;
use regex_splitter::RegexSplitter;

pub const MAP_HELP: &str = include_str!("map.md");
pub const MAP_HELP_VERBOSE: &str = include_str!("map_verbose.md");

/// Iterates over `RegexSplitter` and runs each of the `commands` on each
/// record.
fn map(splitter: RegexSplitter, options: &Options) -> i32 {
    let mut status = 0;
    let chunk_size = options
        .limit
        .map_or(1, |limit| if limit > 0 { limit as usize } else { 1 });
    for chunk in &splitter.map_while(Result::ok).chunks(chunk_size) {
        // TODO: This is ugly and allocates.
        let records: Vec<Vec<u8>> = chunk.collect();
        let records: Vec<&[u8]> = records.iter().map(std::vec::Vec::as_slice).collect();
        for command in &options.match_commands {
            match run_command(command, &records, true) {
                Ok(run_status) => {
                    if run_status != 0 {
                        status += 1;
                    }
                }
                Err(error) => {
                    eprintln!("{command} ... : {error}");
                    status += 1;
                }
            }
        }
    }
    status
}

/// Runs the `map` command on `arguments`.
pub fn map_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        exit_with_result(help(
            0,
            MAP_HELP,
            true,
            if options.verbose {
                Some(MAP_HELP_VERBOSE)
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
                status += map(
                    RegexSplitter::new(&mut read, &options.input_record_delimiter),
                    &options,
                );
            }
            Err(error) => {
                eprintln!("{pathname}: {error}");
                status += 1;
            }
        }
    }
    Ok(status)
}
