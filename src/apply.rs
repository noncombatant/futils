//! The `apply` command.

use std::io::{stdout, Write};

use crate::shell::{parse_options, FileOpener, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::{is_not_delimiter, StreamSplitter};
use crate::util::{help, run_command, unescape_backslashes};

/// Command line usage help.
pub(crate) const APPLY_HELP_MESSAGE: &str = include_str!("apply_help.md");

/// Iterates over `StreamSplitter` and runs each of the `commands` on each
/// record. `verbose` enables printing `stdout` from the `commands`. Each
/// record’s output is delimited by `output_delimiter`.
fn apply(
    splitter: StreamSplitter,
    commands: &[String],
    verbose: bool,
    output_delimiter: &[u8],
) -> ShellResult {
    let mut stdout = stdout();
    let mut status = 0;
    for r in splitter.filter(is_not_delimiter) {
        for command in commands {
            match run_command(command, &r.bytes, verbose) {
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
            if verbose {
                stdout.write_all(output_delimiter)?;
            }
        }
    }
    Ok(status)
}

/// Runs the `apply` command on `arguments`.
pub(crate) fn apply_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, APPLY_HELP_MESSAGE);
    }

    let input_delimiter = options.input_record_delimiter;
    let output_delimiter = unescape_backslashes(&options.output_record_delimiter)?;
    let output_delimiter = output_delimiter.as_bytes();

    let mut status = 0;
    for file in FileOpener::new(arguments) {
        let pathname = file.pathname.unwrap_or(&STDIN_PATHNAME);
        match file.read {
            Ok(mut read) => {
                match apply(
                    StreamSplitter::new(&mut read, &input_delimiter),
                    &options.match_commands,
                    options.verbose,
                    output_delimiter,
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
