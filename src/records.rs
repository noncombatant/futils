use std::io::{stdout, Write};

use crate::shell::{parse_options, FileOpener, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::{is_not_delimiter, StreamSplitter};
use crate::util::{help, unescape_backslashes};

/// Command line usage help.
pub(crate) const RECORDS_HELP_MESSAGE: &str = include_str!("records_help.md");

fn print_record(n: usize, record: &[u8], enumerate: bool, output_delimiter: &[u8]) -> ShellResult {
    let mut stdout = stdout();
    if !record.is_empty() {
        if enumerate {
            write!(stdout, "{:05}: ", n)?;
        }
        stdout.write_all(record)?;
        stdout.write_all(output_delimiter)?;
    }
    Ok(0)
}

/// Runs the `records` command on `arguments`.
pub(crate) fn records_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, RECORDS_HELP_MESSAGE);
    }

    let output_delimiter = unescape_backslashes(&options.output_record_delimiter)?;
    let output_delimiter = output_delimiter.as_bytes();

    let mut status = 0;
    for file in FileOpener::new(arguments) {
        match file.read {
            Ok(mut read) => {
                for (n, r) in StreamSplitter::new(&mut read, &options.input_record_delimiter)
                    .filter(is_not_delimiter)
                    .enumerate()
                {
                    print_record(n + 1, &r.bytes, options.enumerate, output_delimiter)?;
                }
            }
            Err(e) => {
                let p = file.pathname.unwrap_or(&STDIN_PATHNAME);
                eprintln!("{}: {}", p, e);
                status += 1;
            }
        }
    }
    Ok(status)
}
