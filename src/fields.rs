use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::io::{stdout, Write};
use std::num::ParseIntError;

use crate::shell::{parse_options, FileOpener, Options, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::{is_not_delimiter, Record, StreamSplitter};
use crate::util::help;

/// Command line usage help.
pub(crate) const FIELDS_HELP_PAGE: &str = include_str!("fields_help.md");

// TODO: Implement support for named fields.

lazy_static! {
    static ref SPACE_CADET: Regex = Regex::new(r"\S").unwrap();
}

/// Returns the index of the first byte that is not a space character.
fn skip_leading_spaces(record: &[u8]) -> Option<usize> {
    SPACE_CADET.find(record).map(|m| m.start())
}

fn compute_index(i: isize, length: usize) -> usize {
    if i < 0 {
        let length = length as isize;
        let i = i.abs();
        if i > length {
            // Let the caller handle the invalid index, rather than panicking
            // here on overflow.
            i as usize
        } else {
            (length - i) as usize
        }
    } else {
        i as usize
    }
}

fn select_fields<'a>(fields: &[&'a [u8]], requested: &[isize], invert: bool) -> Vec<&'a [u8]> {
    let requested: Vec<usize> = requested
        .iter()
        .map(|f| compute_index(*f, fields.len()))
        .collect();
    if invert {
        fields
            .iter()
            .enumerate()
            .filter(|pair| !requested.contains(&(pair.0)))
            .map(|pair| *pair.1)
            .collect()
    } else {
        requested
            .iter()
            // We use `get` instead of indexing with `[]` to avoid a `panic!` in
            // case a record does not have the requested field. One could argue
            // that we should panic, or print an error. For now I'm going with
            // yielding an empty field. This is a semipredicate error: field not
            // present vs. present and empty looks the same with this
            // implementation. TODO: Consider that.
            .map(|n| {
                if let Some(f) = fields.get(*n) {
                    *f
                } else {
                    b""
                }
            })
            .collect()
    }
}

fn print_record(
    r: Record,
    number: Option<usize>,
    options: &Options,
    requested_fields: &[isize],
) -> ShellResult {
    let mut stdout = stdout();
    let start = if options.skip {
        match skip_leading_spaces(&r.bytes) {
            Some(start) => start,
            None => return Ok(0),
        }
    } else {
        0
    };
    let mut fields = options
        .input_field_delimiter
        .split(&r.bytes[start..])
        .collect::<Vec<&[u8]>>();
    if !requested_fields.is_empty() {
        fields = select_fields(&fields, requested_fields, options.invert_fields);
    }
    let record = fields.join(options.output_field_delimiter.as_slice());
    if let Some(n) = number {
        write!(stdout, "{}", n + 1)?;
        stdout.write_all(&options.output_field_delimiter)?;
    }
    stdout.write_all(&record)?;
    stdout.write_all(&options.output_record_delimiter)?;
    Ok(0)
}

/// Runs the `fields` command on `arguments`.
pub(crate) fn fields_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, FIELDS_HELP_PAGE);
    }

    if options.invert_fields && options.fields.is_empty() {
        help(-1, FIELDS_HELP_PAGE);
    }

    // TODO: To support named fields, use an `enum Field` here with `isize` and
    // `String` variants.
    let fields = options
        .fields
        .iter()
        .map(|f| str::parse::<isize>(f))
        .collect::<Result<Vec<isize>, ParseIntError>>()?;

    let mut status = 0;
    for file in FileOpener::new(arguments) {
        match file.read {
            Ok(mut read) => {
                for (n, r) in StreamSplitter::new(&mut read, &options.input_record_delimiter)
                    .filter(is_not_delimiter)
                    .enumerate()
                {
                    print_record(
                        r,
                        if options.enumerate { Some(n) } else { None },
                        &options,
                        &fields,
                    )?;
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
