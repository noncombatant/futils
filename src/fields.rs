use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::num::ParseIntError;

use crate::shell::{parse_options, ShellResult};
use crate::stream_splitter::{is_not_delimiter, Record, StreamSplitter};
use crate::util::{help, unescape_backslashes};

/// Command line usage help.
pub(crate) const FIELDS_HELP_MESSAGE: &str = include_str!("fields_help.md");

// TODO: Implement support for named fields.

lazy_static! {
    static ref SPACE_CADET: Regex = Regex::new(r"\S").unwrap();
}

// Returns the index of the first byte that is not a space character.
fn skip_leading_spaces(record: &[u8]) -> Option<usize> {
    SPACE_CADET.find(record).map(|m| m.start())
}

fn print_record(
    r: Record,
    number: Option<usize>,
    skip_leading: bool,
    requested_fields: &[usize],
    input_field_delimiter: &Regex,
    output_field_delimiter: &[u8],
    output_record_delimiter: &[u8],
) -> ShellResult {
    let start = if skip_leading {
        match skip_leading_spaces(&r.bytes) {
            Some(start) => start,
            None => return Ok(0),
        }
    } else {
        0
    };
    let mut fields = input_field_delimiter
        .split(&r.bytes[start..])
        .collect::<Vec<&[u8]>>();
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
    if let Some(n) = number {
        write!(stdout(), "{}", n + 1)?;
        stdout().write_all(output_field_delimiter)?;
    }
    stdout().write_all(&record)?;
    stdout().write_all(output_record_delimiter)?;
    Ok(0)
}

pub(crate) fn fields_main(arguments: &[String]) -> ShellResult {
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

    let fields = options
        .fields
        .iter()
        .map(|f| str::parse::<usize>(f))
        .collect::<Result<Vec<usize>, ParseIntError>>()?;
    let fields = fields.iter().map(|f| f - 1).collect::<Vec<usize>>();

    let mut status = 0;
    if arguments.is_empty() {
        let mut stdin = stdin();
        for (n, r) in StreamSplitter::new(&mut stdin, &input_record_delimiter)
            .filter(is_not_delimiter)
            .enumerate()
        {
            print_record(
                r,
                if options.enumerate { Some(n) } else { None },
                options.skip,
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
                    for (n, r) in StreamSplitter::new(&mut file, &input_record_delimiter)
                        .filter(is_not_delimiter)
                        .enumerate()
                    {
                        print_record(
                            r,
                            if options.enumerate { Some(n) } else { None },
                            options.skip,
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
