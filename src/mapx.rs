// Copyright 2023 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils mapx` command.

use std::io::stdin;

use itertools::{chain, Itertools};

use crate::shell::{parse_options, Options, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::StreamSplitter;
use crate::util::{help, run_command};

pub(crate) const MAPX_HELP: &str = include_str!("mapx.md");
pub(crate) const MAPX_HELP_VERBOSE: &str = include_str!("mapx_verbose.md");

/// Iterates over `StreamSplitter` and runs each of the `commands` on each
/// record.
fn mapx(splitter: StreamSplitter, options: &Options, command: &[String]) -> ShellResult {
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
        let arguments = chain(
            command.iter().skip(1).map(|r| r.as_bytes()),
            records.iter().map(|r| r.as_slice()),
        );
        match run_command(&command[0], &arguments.collect::<Vec<&[u8]>>(), true) {
            Ok(s) => {
                if s != 0 {
                    status += 1;
                }
            }
            Err(e) => {
                eprintln!("{:#?} ... : {}", command, e);
                status += 1;
            }
        }
    }
    Ok(status)
}

/// Runs the `mapx` command on `arguments`.
pub(crate) fn mapx_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(
            0,
            MAPX_HELP,
            true,
            if options.verbose {
                Some(MAPX_HELP_VERBOSE)
            } else {
                None
            },
        );
    }
    if options.json_input || options.json_output {
        unimplemented!()
    }

    let mut status = 0;
    match mapx(
        StreamSplitter::new(&mut stdin(), &options.input_record_delimiter),
        &options,
        arguments,
    ) {
        Ok(s) => status += s,
        Err(e) => {
            eprintln!("{:?}: {}", STDIN_PATHNAME, e);
            status += 1;
        }
    }
    Ok(status)
}
