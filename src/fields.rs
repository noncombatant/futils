use getopt::Opt;
use regex::bytes::Regex;
use std::fs::File;
use std::io::{stdin, stdout, Write};

use crate::shell::ShellResult;
use crate::stream_splitter::{is_not_delimiter, Record, StreamSplitter};
use crate::util::{help, unescape_backslashes};
use crate::{DEFAULT_OUTPUT_FIELD_DELIMITER, DEFAULT_OUTPUT_RECORD_DELIMITER};

pub const FIELDS_HELP_MESSAGE: &str =
    "# `fields` — selects and formats the fields from input records

## Usage

```
fields -h
fields [-D delimiter] [-d delimiter] [-O delimiter] [-o delimiter] [-f field] [pathname [...]]
```

## Description

Reads the given `pathname`s (or `stdin` if none are given), splits them into
records using the input delimiter, splits each record into fields using the
field delimiter, selects the requested `field`(s), and prints them, delimiting
them with the output field and record delimiters.

## Options

* `-D`: Use the given input field `delimiter`, a regular expression. The
  default delimiter is `r\"\\s+\"`.
* `-d`: Use the given input record `delimiter`, a regular expression. The
  default delimiter is `r\"(\\r|\\n)+\"`.
* `-f`: Select the given `field`(s). This option can be given multiple times,
  and fields will be output in the order given on the command line.
* `-n`: Prefix each record with a record number.
* `-O`: Use the given output field `delimiter`. The default delimiter is `\\t`.
* `-o`: Use the given output record `delimiter`. The default delimiter is `\\n`.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

## Additional Options

    -h  Prints this help message.";

fn print_record(
    r: Record,
    input_field_delimiter: &Regex,
    output_field_delimiter: &[u8],
    output_record_delimiter: &[u8],
) -> ShellResult {
    let fields = input_field_delimiter.split(&r.bytes);
    let fields = fields.collect::<Vec<&[u8]>>();
    let record = fields.join(output_field_delimiter);
    stdout().write_all(&record)?;
    stdout().write_all(output_record_delimiter)?;
    Ok(0)
}

pub fn fields_main(arguments: &[String]) -> ShellResult {
    let mut options = getopt::Parser::new(arguments, "D:d:f:hO:o:");
    let mut input_field_delimiter = Regex::new(r"\s+")?;
    let mut input_record_delimiter = Regex::new(r"(\r\n|\n|\r)")?;
    let mut output_record_delimiter = String::from(DEFAULT_OUTPUT_RECORD_DELIMITER);
    let mut output_field_delimiter = String::from(DEFAULT_OUTPUT_FIELD_DELIMITER);
    let mut fields = Vec::new();
    loop {
        match options.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('D', Some(string)) => input_field_delimiter = Regex::new(&string)?,
                Opt('d', Some(string)) => input_record_delimiter = Regex::new(&string)?,
                Opt('f', Some(string)) => fields.push(string.clone()),
                Opt('h', None) => help(0, FIELDS_HELP_MESSAGE),
                Opt('O', Some(string)) => output_field_delimiter = string.clone(),
                Opt('o', Some(string)) => output_record_delimiter = string.clone(),
                _ => help(-1, FIELDS_HELP_MESSAGE),
            },
        }
    }
    let (_, arguments) = arguments.split_at(options.index());

    // TODO: Since we use this pattern all over the place, maybe we should have
    // `unescape_backslashes` return bytes.
    let output_record_delimiter = unescape_backslashes(&output_record_delimiter)?;
    let output_record_delimiter = output_record_delimiter.as_bytes();
    let output_field_delimiter = unescape_backslashes(&output_field_delimiter)?;
    let output_field_delimiter = output_field_delimiter.as_bytes();

    let mut status = 0;
    if arguments.is_empty() {
        let mut stdin = stdin();
        for r in StreamSplitter::new(&mut stdin, &input_record_delimiter).filter(is_not_delimiter) {
            print_record(
                r,
                &input_field_delimiter,
                output_field_delimiter,
                output_record_delimiter,
            )?;
        }
    } else {
        for pathname in arguments {
            match File::open(pathname) {
                Ok(mut file) => {
                    for r in StreamSplitter::new(&mut file, &input_record_delimiter)
                        .filter(is_not_delimiter)
                    {
                        print_record(
                            r,
                            &input_field_delimiter,
                            output_field_delimiter,
                            output_record_delimiter,
                        )?;
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
