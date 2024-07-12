// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils fields` command.

use std::io::{stdout, Error, Read, Write};
use std::num::ParseIntError;

use atty::Stream;
use once_cell::sync::Lazy;
use regex::bytes::Regex;
use serde::Serialize;

use crate::shell::{parse_options, FileOpener, Options, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::StreamSplitter;
use crate::util::{exit_with_result, help};

pub const FIELDS_HELP: &str = include_str!("fields.md");
pub const FIELDS_HELP_VERBOSE: &str = include_str!("fields_verbose.md");

/// Returns the index of the first byte that is not a space character.
fn first_non_space(record: &[u8]) -> Option<usize> {
    static SPACE_CADET: Lazy<Regex> = Lazy::new(|| Regex::new(r"\S").unwrap());
    SPACE_CADET.find(record).map(|m| m.start())
}

/// `select_fields` returns the `requested` indices from `fields` in 1 of 2
/// ways:
///
/// When `invert` is false, it iterates over `requested` and gathers the indexed
/// fields. When the requested index is negative, it counts from the end of
/// `fields`; e.g. -1 is the last element, -2 is the second-to-last element, and
/// so on.
///
/// When `invert` is true, it iterates over `fields` and gathers each element
/// *unless* `requested` contains that element's index (treating negative
/// indices as above). Note that this approach is O(n ✖️ m ✖️ m), where n =
/// `fields.len()` and m = `requested.len()`, due to the linear behavior of
/// `contains` on a slice and because for each index of `fields` we check both
/// whether its positive index and its negative index is in `requested`.
/// (There's probably a more efficient way to do it. However, both n and m are
/// likely to be small.)
#[allow(clippy::cast_possible_wrap)] // Checked below.
fn select_fields<'a>(fields: &[&'a [u8]], requested: &[isize], invert: bool) -> Vec<&'a [u8]> {
    let mut result: Vec<&'a [u8]> = vec![];
    assert!(isize::try_from(fields.len()).is_ok());
    let length = fields.len() as isize;
    if invert {
        for (n, f) in fields.iter().enumerate() {
            assert!(isize::try_from(n).is_ok());
            let n = n as isize;
            let m = -(length - n);
            if requested.contains(&n) || requested.contains(&m) {
                continue;
            }
            result.push(*f);
        }
    } else {
        for n in requested {
            let n = *n;
            // We say `length + n` because adding a negative is subtraction,
            // which is what we want.
            let contains = (n >= 0 && n < length) || (n < 0 && length + n >= 0);
            if contains {
                let n = if n < 0 { length + n } else { n };
                result.push(fields[n as usize]);
            }
        }
    }
    result
}

#[test]
fn test_select_fields() {
    let fields: Vec<&[u8]> = vec![b"hello", b"world", b"how's", b"it", b"goin'"];

    let expected: Vec<&[u8]> = vec![b"hello", b"it"];
    let result = select_fields(&fields, &[0, 3], false);
    assert_eq!(expected, result);

    let expected: Vec<&[u8]> = vec![b"world", b"goin'"];
    let result = select_fields(&fields, &[1, -1], false);
    assert_eq!(expected, result);

    let expected: Vec<&[u8]> = vec![b"hello", b"how's", b"it"];
    let result = select_fields(&fields, &[1, -1], true);
    assert_eq!(expected, result);

    let expected: Vec<&[u8]> = vec![b"world", b"how's", b"goin'"];
    let result = select_fields(&fields, &[0, 3], true);
    assert_eq!(expected, result);
}

// TODO: Consider folding this into enumerated_record.rs?
#[derive(Serialize)]
struct EnumeratedRecord<'a> {
    n: Option<usize>,
    pathname: &'a str,
    fields: Vec<&'a [u8]>,
}

impl<'a> EnumeratedRecord<'a> {
    fn new(
        n: Option<usize>,
        pathname: &'a str,
        record: &'a [u8],
        requested_fields: &[isize],
        options: &Options,
    ) -> Self {
        let mut start = 0;
        if options.skip {
            if let Some(s) = first_non_space(record) {
                start = s;
            }
        };
        let mut fields = options
            .input_field_delimiter
            .split(&record[start..])
            .collect::<Vec<&[u8]>>();
        if !requested_fields.is_empty() {
            fields = select_fields(&fields, requested_fields, options.invert_fields);
        }
        EnumeratedRecord {
            n,
            pathname,
            fields,
        }
    }

    fn write_columns(&self, output: &mut dyn Write, options: &Options) -> Result<(), Error> {
        if !self.fields.is_empty() {
            if let Some(n) = self.n {
                output.write_all(self.pathname.as_bytes())?;
                output.write_all(&options.output_field_delimiter)?;
                write!(output, "{:>5}", n + 1)?;
                output.write_all(&options.output_field_delimiter)?;
            }
            for (n, f) in self.fields.iter().enumerate() {
                output.write_all(f)?;
                if n != self.fields.len() - 1 {
                    output.write_all(&options.output_field_delimiter)?;
                }
            }
            output.write_all(&options.output_record_delimiter)?;
        }
        Ok(())
    }

    fn write_json(&self, output: &mut dyn Write, pretty: bool) -> Result<(), Error> {
        if !self.fields.is_empty() {
            let to_json = if pretty {
                serde_json::to_writer_pretty
            } else {
                serde_json::to_writer
            };
            to_json(output, &self)?;
        }
        Ok(())
    }
}

fn print_fields(
    reader: &mut dyn Read,
    pathname: &str,
    options: &Options,
    requested_fields: &[isize],
) -> ShellResult {
    for (n, r) in StreamSplitter::new(reader, &options.input_record_delimiter)
        .map_while(Result::ok)
        .enumerate()
    {
        let fields = EnumeratedRecord::new(
            if options.no_enumerate { None } else { Some(n) },
            pathname,
            &r,
            requested_fields,
            options,
        );
        if options.json_output {
            fields.write_json(&mut stdout(), atty::is(Stream::Stdout))?;
        } else {
            fields.write_columns(&mut stdout(), options)?;
        }
    }
    Ok(0)
}

/// Runs the `fields` command on `arguments`.
pub fn fields_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        exit_with_result(help(
            0,
            FIELDS_HELP,
            true,
            if options.verbose {
                Some(FIELDS_HELP_VERBOSE)
            } else {
                None
            },
        ));
    }
    if options.invert_fields && options.fields.is_empty() {
        exit_with_result(help(-1, FIELDS_HELP, false, None));
    }

    // TODO: To support named fields, use an `enum Field` here with `isize` and
    // `String` variants.
    let requested_fields = options
        .fields
        .iter()
        .map(|f| str::parse::<isize>(f))
        .collect::<Result<Vec<isize>, ParseIntError>>()?;

    let mut status = 0;
    for file in FileOpener::new(arguments) {
        let pathname = file.pathname.unwrap_or(&STDIN_PATHNAME);
        match file.read {
            Ok(mut read) => match print_fields(&mut read, pathname, &options, &requested_fields) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("{pathname}: {error}");
                    status += 1;
                }
            },
            Err(error) => {
                eprintln!("{pathname}: {error}");
                status += 1;
            }
        }
    }
    Ok(status)
}
