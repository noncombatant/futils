use getopt::Opt;
use regex::bytes::Regex;
use std::io::{stdin, stdout, Read, Write};

use crate::util::{help, map_file, unescape_backslashes, ShellResult};
use crate::DEFAULT_OUTPUT_DELIMITER;

const HELP_MESSAGE: &str = "records - splits a file into records

Usage:

    records -h
    records [-d input_delimiter] [-o output_delimiter] file [...]

Reads the given *file*s, splits them into records using the *input_delimiter* (a
regular expression) and prints them, delimiting them with the *output_delimiter*
(a string).

By default, the input delimiter is `r\"(\\r\\n|\\n|\\r)\"` and the output
delimiter is \"\\n\".

Regular expressions use the Rust regex library syntax
(https://docs.rs/regex/latest/regex/).

Additional options:

    -h  Prints this help message.";

pub fn records_main(arguments: &[String]) -> ShellResult {
    let mut options = getopt::Parser::new(arguments, "d:ho:");
    let mut input_delimiter = Regex::new(r"(\r\n|\n|\r)")?;
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);
    loop {
        match options.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('d', Some(string)) => input_delimiter = Regex::new(&unescape_backslashes(&string)?)?,
                Opt('h', None) => help(0, HELP_MESSAGE),
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                _ => help(-1, HELP_MESSAGE),
            },
        }
    }

    let output_delimiter = unescape_backslashes(&output_delimiter)?;
    let output_delimiter_bytes = output_delimiter.as_bytes();

    let (_, arguments) = arguments.split_at(options.index());

    let mut status = 0;
    if arguments.is_empty() {
        let mut bytes = Vec::new();
        // TODO: This is inefficient!
        stdin().read_to_end(&mut bytes)?;
        for record in input_delimiter.split(&bytes) {
            if !record.is_empty() {
                stdout().write_all(record)?;
                stdout().write_all(output_delimiter_bytes)?;
            }
        }
    } else {
        for pathname in arguments {
            match map_file(pathname) {
                Ok(mapped) => {
                    for record in input_delimiter.split(&mapped) {
                        if !record.is_empty() {
                            stdout().write_all(record)?;
                            stdout().write_all(output_delimiter_bytes)?;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    status += 1;
                }
            }
        }
    }
    Ok(status)
}
