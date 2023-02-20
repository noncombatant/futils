use getopt::Opt;
use regex::bytes::Regex;
use std::fs::File;
use std::io::{stdin, stdout, Write};

use crate::shell::ShellResult;
use crate::stream_splitter::{is_not_delimiter, StreamSplitter};
use crate::util::{help, unescape_backslashes};
use crate::{DEFAULT_INPUT_RECORD_DELIMITER, DEFAULT_OUTPUT_RECORD_DELIMITER};

pub const RECORDS_HELP_MESSAGE: &str = "# `records` - splits a file into records

## Usage

```
records -h
records [-n] [-d delimiter] [-o delimiter] [pathname [...]]
```

## Description

Reads the given `pathname`s (or `stdin` if none are given), splits them into
records using the input delimiter, and prints them, delimiting them with the
output delimiter.

## Options

* `-d`: Use the given input record `delimiter`, a regular expression. The
  default delimiter is `r\"(\\r|\\n)+\"`.
* `-n`: Prefix each record with a record number.
* `-o`: Use the given output record `delimiter`. The default delimiter is `\\n`.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

## Additional Options

    -h  Prints this help message.";

fn print_record(n: usize, record: &[u8], enumerate: bool, output_delimiter: &[u8]) -> ShellResult {
    if !record.is_empty() {
        if enumerate {
            let s = format!("{:05}: ", n);
            stdout().write_all(s.as_bytes())?;
        }
        stdout().write_all(record)?;
        stdout().write_all(output_delimiter)?;
    }
    Ok(0)
}

pub fn records_main(arguments: &[String]) -> ShellResult {
    let mut options = getopt::Parser::new(arguments, "d:hno:");
    let mut input_delimiter = Regex::new(DEFAULT_INPUT_RECORD_DELIMITER)?;
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_RECORD_DELIMITER);
    let mut enumerate = false;
    loop {
        match options.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('d', Some(string)) => input_delimiter = Regex::new(&string)?,
                Opt('h', None) => help(0, RECORDS_HELP_MESSAGE),
                Opt('n', None) => enumerate = true,
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                _ => help(-1, RECORDS_HELP_MESSAGE),
            },
        }
    }

    let output_delimiter = unescape_backslashes(&output_delimiter)?;
    let output_delimiter_bytes = output_delimiter.as_bytes();

    let (_, arguments) = arguments.split_at(options.index());

    let mut status = 0;
    if arguments.is_empty() {
        let mut stdin = stdin();
        for (n, r) in StreamSplitter::new(&mut stdin, &input_delimiter)
            .filter(is_not_delimiter)
            .enumerate()
        {
            print_record(n + 1, &r.bytes, enumerate, output_delimiter_bytes)?;
        }
    } else {
        for pathname in arguments {
            match File::open(pathname) {
                Ok(mut file) => {
                    for (n, r) in StreamSplitter::new(&mut file, &input_delimiter)
                        .filter(is_not_delimiter)
                        .enumerate()
                    {
                        print_record(n + 1, &r.bytes, enumerate, output_delimiter_bytes)?;
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
