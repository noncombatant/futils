use getopt::Opt;
use regex::bytes::Regex;
use std::io::{stdin, stdout, Read, Write};

use crate::util::{help, map_file, unescape_backslashes, ShellResult};
use crate::DEFAULT_OUTPUT_DELIMITER;

pub const RECORDS_HELP_MESSAGE: &str = "records - splits a file into records

Usage:

```
records -h
records [-n] [-d delimiter] [-o delimiter] pathname [...]
```

Options:

* `-d`: Use the given input record `delimiter`, a regular expression. The
  default delimiter is `r\"(\\r\\n|\\n|\\r)\"`.
* `-n`: Prefixes each record with a record number.
* `-o`: Use the given output record `delimiter`. The default delimiter is `\\n`.

Reads the given `pathname`s, splits them into records using the input delimiter
and prints them, delimiting them with the output delimiter.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

Additional options:

    -h  Prints this help message.";

pub fn records_main(arguments: &[String]) -> ShellResult {
    let mut options = getopt::Parser::new(arguments, "d:hno:");
    let mut input_delimiter = Regex::new(r"(\r\n|\n|\r)")?;
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);
    let mut show_number = false;
    loop {
        match options.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('d', Some(string)) => {
                    input_delimiter = Regex::new(&unescape_backslashes(&string)?)?
                }
                Opt('h', None) => help(0, RECORDS_HELP_MESSAGE),
                Opt('n', None) => show_number = true,
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
        let mut bytes = Vec::new();
        // TODO: This is inefficient! We need stream processing, and to combine
        // this branch with the `pathname in arguments` branch below — we should
        // be iterating over chunks from `split`, regardless of the source.
        stdin().read_to_end(&mut bytes)?;
        let mut n = 0usize;
        for record in input_delimiter.split(&bytes) {
            if !record.is_empty() {
                if show_number {
                    n += 1;
                    let s = format!("{:05}: ", n);
                    stdout().write_all(s.as_bytes())?;
                }
                stdout().write_all(record)?;
                stdout().write_all(output_delimiter_bytes)?;
            }
        }
    } else {
        for pathname in arguments {
            match map_file(pathname) {
                Ok(mapped) => {
                    let mut n = 0usize;
                    for record in input_delimiter.split(&mapped) {
                        if !record.is_empty() {
                            if show_number {
                                n += 1;
                                let s = format!("{:05}: ", n);
                                stdout().write_all(s.as_bytes())?;
                            }
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
