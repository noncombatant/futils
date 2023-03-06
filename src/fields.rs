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
  and fields will be output in the order given on the command line. Field
  numbering starts from 1. If no `-f` options are given, `fields` will print all
  fields.
* `-n`: Prefix each record with a record number.
* `-O`: Use the given output field `delimiter`. The default delimiter is `\\t`.
* `-o`: Use the given output record `delimiter`. The default delimiter is `\\n`.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

## Additional Options

    -h  Prints this help message.";

fn print_record(
    r: Record,
    requested_fields: &[usize],
    input_field_delimiter: &Regex,
    output_field_delimiter: &[u8],
    output_record_delimiter: &[u8],
) -> ShellResult {
    let mut fields = input_field_delimiter.split(&r.bytes).collect::<Vec<&[u8]>>();
    if !requested_fields.is_empty() {
        let mut selected_fields: Vec<&[u8]> = Vec::new();
        for i in requested_fields {
            // We use `get` instead of indexing with `[]` to avoid a `panic!` in
            // case a record does not have the requested field. One could argue
            // that we should panic, or print an error. For now I'm going with
            // yielding an empty field. This is a semipredicate error: field not
            // present vs. present and empty looks the same with this
            // implementation. TODO: Consider that.
            if let Some(f) = fields.get(*i) {
                selected_fields.push(f);
            } else {
                selected_fields.push(b"");
            }
        }
        fields = selected_fields;
    };
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

    let input_record_delimiter = options.input_record_delimiter;
    let input_field_delimiter = options.input_field_delimiter;
    let output_record_delimiter = unescape_backslashes(&options.output_record_delimiter)?;
    let output_record_delimiter = output_record_delimiter.as_bytes();
    let output_field_delimiter = unescape_backslashes(&options.output_field_delimiter)?;
    let output_field_delimiter = output_field_delimiter.as_bytes();

    // TODO: `unwrap` is inappropriate here. Instead, if any parse failed,
    // return an error from `fields_main`.
    let fields: Vec<usize> = options
        .fields
        .iter()
        .map(|f| str::parse::<usize>(f).unwrap() - 1)
        .collect();

    let mut status = 0;
    if arguments.is_empty() {
        let mut stdin = stdin();
        for r in StreamSplitter::new(&mut stdin, &input_record_delimiter).filter(is_not_delimiter) {
            print_record(
                r,
                &fields,
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
                            &fields,
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
