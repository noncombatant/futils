// Copyright 2023 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils mapx` command.

use crate::{
    shell::{Options, ShellResult, parse_options},
    util::{exit_with_result, help, run_command},
};
use itertools::{Itertools, chain};
use regex_splitter::RegexSplitter;
use std::io::stdin;

pub const MAPX_HELP: &str = include_str!("mapx.md");
pub const MAPX_HELP_VERBOSE: &str = include_str!("mapx_verbose.md");

/// Iterates over `RegexSplitter` and runs each of the `commands` on each
/// record.
fn mapx(splitter: RegexSplitter, options: &Options, command: &[String]) -> i32 {
    let mut status = 0;
    let chunk_size = options
        .limit
        .map_or(1, |limit| if limit > 0 { limit as usize } else { 1 });
    for chunk in &splitter.map_while(Result::ok).chunks(chunk_size) {
        // TODO: This is ugly and allocates.
        let records: Vec<Vec<u8>> = chunk.collect();
        let arguments = chain(
            command.iter().skip(1).map(std::string::String::as_bytes),
            records.iter().map(std::vec::Vec::as_slice),
        );
        match run_command(&command[0], &arguments.collect::<Vec<&[u8]>>(), true) {
            Ok(run_status) => {
                if run_status != 0 {
                    status += 1;
                }
            }
            Err(error) => {
                eprintln!("{command:#?} ... : {error}");
                status += 1;
            }
        }
    }
    status
}

/// Runs the `mapx` command on `arguments`.
pub fn mapx_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        exit_with_result(help(
            0,
            MAPX_HELP,
            true,
            if options.verbose {
                Some(MAPX_HELP_VERBOSE)
            } else {
                None
            },
        ));
    }
    if options.json_input || options.json_output {
        unimplemented!()
    }
    Ok(mapx(
        RegexSplitter::new(&mut stdin(), &options.input_record_delimiter),
        &options,
        arguments,
    ))
}
