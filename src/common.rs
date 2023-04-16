//! The `futils common` command.

use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::cmp::Ordering::*;

use crate::shell::{parse_options, Options, ShellResult};
use crate::stream_splitter::{Record, StreamSplitter};
use crate::util::help;

/// Command line usage help.
pub(crate) const COMMON_HELP: &str = include_str!("common_help.md");

fn print(column: i8, field: &Record, options: &Options) -> ShellResult {
    let mut out = stdout();
    match column {
        1 => (),
        2 => out.write_all(&options.output_field_delimiter)?,
        3 => {
            out.write_all(&options.output_field_delimiter)?;
            out.write_all(&options.output_field_delimiter)?;
        }
        _ => unreachable!(),
    }
    out.write_all(&field.data)?;
    out.write_all(&options.output_record_delimiter)?;
    Ok(0)
}

/// Runs the `common` command on `arguments`.
pub(crate) fn common_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, COMMON_HELP);
    }
    if arguments.is_empty() || arguments.len() > 2 {
        help(1, COMMON_HELP);
    }

    let mut stdin = stdin();
    let mut file1 = File::open(&arguments[0])?;
    let mut file2: File;
    let records1: StreamSplitter;
    let records2: StreamSplitter;
    let delimiter = &options.input_record_delimiter;
    if arguments.len() == 2 {
        file2 = File::open(&arguments[1])?;
        records1 = StreamSplitter::new(&mut file1, delimiter);
        records2 = StreamSplitter::new(&mut file2, delimiter);
    } else {
        records1 = StreamSplitter::new(&mut stdin, delimiter);
        records2 = StreamSplitter::new(&mut file1, delimiter);
    }

    let mut records1 = records1.map_while(Result::ok);
    let mut records2 = records2.map_while(Result::ok);

    // Adapted from *Command-Line Rust* by Ken Youens-Clark, pp. 242 – 243.
    let mut record1 = records1.next();
    let mut record2 = records2.next();
    while record1.is_some() || record2.is_some() {
        match (&record1, &record2) {
            (Some(r1), Some(r2)) => match r1.cmp(r2) {
                Equal => {
                    print(3, r1, &options)?;
                    record1 = records1.next();
                    record2 = records2.next();
                }
                Less => {
                    print(1, r1, &options)?;
                    record1 = records1.next();
                }
                Greater => {
                    print(2, r2, &options)?;
                    record2 = records2.next();
                }
            },
            (Some(r1), None) => {
                print(1, r1, &options)?;
                record1 = records1.next();
            }
            (None, Some(r2)) => {
                print(2, r2, &options)?;
                record2 = records2.next();
            }
            _ => (),
        }
    }
    Ok(0)
}
