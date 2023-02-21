use regex::bytes::Regex;
use std::fs::File;
use std::io::{stdin, stdout, Write};

use crate::shell::{parse_options, ShellResult};
use crate::stream_splitter::{is_not_delimiter, Record, StreamSplitter};
use crate::util::{help, unescape_backslashes};

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
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, FIELDS_HELP_MESSAGE);
    }

    let input_record_delimiter = options.input_record_delimiter.expect("No input record delimiter specified");
    let input_field_delimiter = options.input_field_delimiter.expect("No input field delimiter specified");
    let output_record_delimiter = options
        .output_record_delimiter
        .expect("No output record delimiter specified");
    let output_record_delimiter = unescape_backslashes(&output_record_delimiter)?;
    let output_record_delimiter = output_record_delimiter.as_bytes();
    let output_field_delimiter = options
        .output_field_delimiter
        .expect("No output field delimiter specified");
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
