use getopt::Opt;
use std::io::{stdout, Write};

use crate::sub_slicer::SubSlicer;
use crate::util::{help, map_file, unescape_backslashes, ShellResult};
use crate::{DEFAULT_INPUT_DELIMITER, DEFAULT_OUTPUT_DELIMITER};

const HELP_MESSAGE: &str = "records - splits a file into records

Usage:

    records -h
    records [-d input_delimiter] [-o output_delimiter] file [...]

Reads the given *file*s, splits them into records using the *input_delimiter*
and prints them, delimiting them with the *output_delimiter*. By default, both
delimiters are \"\\n\".

Additional options:

    -h  Prints this help message.";

pub fn records_main(arguments: &[String]) -> ShellResult {
    let mut options = getopt::Parser::new(arguments, "d:ho:");

    let mut input_delimiter = String::from(DEFAULT_INPUT_DELIMITER);
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);
    loop {
        match options.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('d', Some(string)) => input_delimiter = string.clone(),
                Opt('h', None) => help(0, HELP_MESSAGE),
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                _ => help(-1, HELP_MESSAGE),
            },
        }
    }

    let input_delimiter = unescape_backslashes(&input_delimiter)?;
    let input_delimiter_bytes = input_delimiter.as_bytes();
    let output_delimiter = unescape_backslashes(&output_delimiter)?;
    let output_delimiter_bytes = output_delimiter.as_bytes();

    let (_, arguments) = arguments.split_at(options.index());

    if arguments.is_empty() {
        eprintln!("TODO: Reading from stdin not implemented yet. Sorry!");
        help(-1, HELP_MESSAGE);
    } else {
        for pathname in arguments {
            if let Some(mapped) = map_file(pathname) {
                let slicer = SubSlicer {
                    slice: &mapped,
                    input_delimiter: input_delimiter_bytes,
                    start: 0,
                };
                for s in slicer {
                    stdout().write_all(s)?;
                    stdout().write_all(output_delimiter_bytes)?;
                }
            }
        }
    }
    Ok(0)
}
