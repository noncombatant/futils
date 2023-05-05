//! The `futils fields` command.

use std::io::{stdout, Error, Read, Write};
use std::num::ParseIntError;

use atty::Stream;
use once_cell::sync::Lazy;
use regex::bytes::Regex;
use serde::Serialize;

use crate::shell::{parse_options, FileOpener, Options, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::StreamSplitter;
use crate::util::help;

/// Command line usage help.
pub(crate) const FIELDS_HELP: &str = include_str!("fields_help.md");

pub(crate) const FIELDS_HELP_VERBOSE: &str = include_str!("fields_help_verbose.md");

/// Returns the index of the first byte that is not a space character.
fn first_non_space(record: &[u8]) -> Option<usize> {
    static SPACE_CADET: Lazy<Regex> = Lazy::new(|| Regex::new(r"\S").unwrap());
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

#[derive(Serialize)]
struct EnumeratedRecord<'a> {
    n: Option<usize>,
    fields: Vec<&'a [u8]>,
}

impl<'a> EnumeratedRecord<'a> {
    fn new(
        n: Option<usize>,
        record: &'a [u8],
        requested_fields: &[isize],
        options: &Options,
    ) -> Self {
        let mut start = 0;
        if options.skip {
            if let Some(s) = first_non_space(record) {
                start = s
            }
        };
        let mut fields = options
            .input_field_delimiter
            .split(&record[start..])
            .collect::<Vec<&[u8]>>();
        if !requested_fields.is_empty() {
            fields = select_fields(&fields, requested_fields, options.invert_fields);
        }
        EnumeratedRecord { n, fields }
    }

    fn write_columns(&self, output: &mut dyn Write, options: &Options) -> Result<(), Error> {
        if !self.fields.is_empty() {
            if let Some(n) = self.n {
                write!(output, "{}", n + 1)?;
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
    options: &Options,
    requested_fields: &[isize],
) -> ShellResult {
    for (n, r) in StreamSplitter::new(reader, &options.input_record_delimiter)
        .map_while(Result::ok)
        .enumerate()
    {
        let fields = EnumeratedRecord::new(
            if options.enumerate { Some(n) } else { None },
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
pub(crate) fn fields_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(
            0,
            FIELDS_HELP,
            if options.verbose {
                Some(FIELDS_HELP_VERBOSE)
            } else {
                None
            },
        );
    }
    if options.invert_fields && options.fields.is_empty() {
        help(-1, FIELDS_HELP, None);
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
        match file.read {
            Ok(mut read) => match print_fields(&mut read, &options, &requested_fields) {
                Ok(_) => {}
                Err(e) => {
                    let p = file.pathname.unwrap_or(&STDIN_PATHNAME);
                    eprintln!("{}: {}", p, e);
                    status += 1;
                }
            },
            Err(e) => {
                let p = file.pathname.unwrap_or(&STDIN_PATHNAME);
                eprintln!("{}: {}", p, e);
                status += 1;
            }
        }
    }
    Ok(status)
}
