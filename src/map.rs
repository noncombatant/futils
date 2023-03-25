//! The `futils map` command.

use std::io::{stdout, Write};

use crate::shell::{parse_options, FileOpener, Options, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::StreamSplitter;
use crate::util::{help, run_command};

/// Command line usage help.
pub(crate) const MAP_HELP: &str = include_str!("map_help.md");

/// Iterates over `StreamSplitter` and runs each of the `commands` on each
/// record, with each field of the record as a distinct argument to the command.
/// Each record’s output is delimited by `output_delimiter`.
fn map(splitter: StreamSplitter, options: &Options) -> ShellResult {
    let mut stdout = stdout();
    let mut status = 0;
    for r in splitter.map_while(|r| r.ok()) {
        for command in &options.match_commands {
            // TODO: split `&r.bytes` on input_field_separator, pass to
            // `run_command`. This requires updating `run_command`.
            match run_command(command, &r.bytes, true) {
                Ok(s) => {
                    if s != 0 {
                        status += 1;
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", std::str::from_utf8(&r.bytes).unwrap(), e);
                    status += 1;
                }
            }
            stdout.write_all(&options.output_record_delimiter)?;
        }
    }
    Ok(status)
}

/// Runs the `map` command on `arguments`.
pub(crate) fn map_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, MAP_HELP);
    }
    if options.json {
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
