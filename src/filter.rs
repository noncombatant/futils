use std::io::{stdout, Write};

use crate::shell::{parse_options, FileOpener, Options, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::{is_not_delimiter, StreamSplitter};
use crate::util::{help, run_command, unescape_backslashes};

/// Command line usage help.
pub(crate) const FILTER_HELP_MESSAGE: &str = include_str!("filter_help.md");

fn print_matches(
    pathname: &str,
    splitter: StreamSplitter,
    options: &Options,
    output_field_delimiter: &[u8],
    output_record_delimiter: &[u8],
) -> ShellResult {
    let mut stdout = stdout();
    'outer: for r in splitter.filter(is_not_delimiter) {
        for re in &options.prune_expressions {
            if re.is_match(&r.bytes) {
                continue 'outer;
            }
        }
        for re in &options.match_expressions {
            if !re.is_match(&r.bytes) {
                continue 'outer;
            }
        }
        for command in &options.match_commands {
            match run_command(command, &r.bytes, options.verbose) {
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
        stdout.write_all(pathname.as_bytes())?;
        stdout.write_all(output_field_delimiter)?;
        stdout.write_all(&r.bytes)?;
        stdout.write_all(output_record_delimiter)?;
    }
    Ok(0)
}

/// Runs the `filter` command on `arguments`.
pub(crate) fn filter_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, FILTER_HELP_MESSAGE);
    }

    let output_field_delimiter = unescape_backslashes(&options.output_field_delimiter)?;
    let output_field_delimiter = output_field_delimiter.as_bytes();
    let output_record_delimiter = unescape_backslashes(&options.output_record_delimiter)?;
    let output_record_delimiter = output_record_delimiter.as_bytes();

    let mut status = 0;
    for file in FileOpener::new(arguments) {
        let pathname = file.pathname.unwrap_or(&STDIN_PATHNAME);
        match file.read {
            Ok(mut read) => {
                print_matches(
                    pathname,
                    StreamSplitter::new(&mut read, &options.input_record_delimiter),
                    &options,
                    output_field_delimiter,
                    output_record_delimiter,
                )?;
            }
            Err(e) => {
                eprintln!("{}: {}", pathname, e);
                status += 1;
            }
        }
    }
    Ok(status)
}
