//! The `futils reduce` command.

use std::io::{stdout, Write};
use std::str::from_utf8;

use crate::shell::{
    parse_options, FileOpener, Options, ShellError, ShellResult, UsageError, STDIN_PATHNAME,
};
use crate::stream_splitter::StreamSplitter;
use crate::util::{help, parse_number};

/// Command line usage help.
pub(crate) const REDUCE_HELP: &str = include_str!("reduce_help.md");

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
    let mut splitter = splitter.map_while(|r| r.ok());
    let mut result = match splitter.next() {
        Some(r) => r.bytes,
        None => return Err(ShellError::Usage(UsageError::new("No input"))),
    };

    for r in splitter {
        for command in &options.match_commands {
            match apply_command(&result, command, &r.bytes) {
                Ok(r) => {
                    result = r;
                }
                Err(e) => {
                    eprintln!("{}: {}", from_utf8(&r.bytes).unwrap(), e);
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
        help(0, REDUCE_HELP);
    }
    if options.json {
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
