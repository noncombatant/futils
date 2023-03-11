use regex::bytes::Regex;
use std::io::{stdout, Write};

use crate::shell::{parse_options, FileOpener, ShellResult};
use crate::stream_splitter::{is_not_delimiter, StreamSplitter};
use crate::util::{help, run_command, unescape_backslashes};

/// Command line usage help.
pub(crate) const FILTER_HELP_MESSAGE: &str = include_str!("filter_help.md");

fn print_matches(
    splitter: StreamSplitter,
    prune_expressions: &[Regex],
    match_expressions: &[Regex],
    match_commands: &[String],
    verbose: bool,
    output_delimiter: &[u8],
) -> ShellResult {
    'outer: for r in splitter.filter(is_not_delimiter) {
        for re in prune_expressions {
            if re.is_match(&r.bytes) {
                continue 'outer;
            }
        }
        for re in match_expressions {
            if !re.is_match(&r.bytes) {
                continue 'outer;
            }
        }
        for command in match_commands {
            match run_command(command, &r.bytes, verbose) {
                Ok(status) => {
                    if status != 0 {
                        continue 'outer;
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{} \"{}\": {}",
                        command,
                        String::from_utf8_lossy(&r.bytes),
                        e
                    );
                    continue 'outer;
                }
            }
        }
        stdout().write_all(&r.bytes)?;
        stdout().write_all(output_delimiter)?;
    }
    Ok(0)
}

/// Runs the `filter` command on `arguments`.
pub(crate) fn filter_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, FILTER_HELP_MESSAGE);
    }

    let output_delimiter = unescape_backslashes(&options.output_record_delimiter)?;
    let output_delimiter = output_delimiter.as_bytes();

    let mut status = 0;
    for file in FileOpener::new(arguments) {
        match file {
            Ok(mut file) => {
                print_matches(
                    StreamSplitter::new(&mut file, &options.input_record_delimiter),
                    &options.prune_expressions,
                    &options.match_expressions,
                    &options.match_commands,
                    options.verbose,
                    output_delimiter,
                )?;
            }
            Err(e) => {
                eprintln!("{}", e);
                status += 1;
            }
        }
    }
    Ok(status)
}
