use itertools::Either;
use std::io::{stdout, Write};

use crate::shell::{parse_options, FileOpener, Options, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::{is_not_delimiter, Record, StreamSplitter};
use crate::util::help;

/// Command line usage help.
pub(crate) const RECORDS_HELP: &str = include_str!("records_help.md");

fn print_record(output: &mut dyn Write, n: usize, record: &[u8], options: &Options) -> ShellResult {
    if !record.is_empty() {
        if options.enumerate {
            write!(output, "{}", n)?;
            output.write_all(&options.output_field_delimiter)?;
        }
        output.write_all(record)?;
        output.write_all(&options.output_record_delimiter)?;
    }
    Ok(0)
}

/// Runs the `records` command on `arguments`.
pub(crate) fn records_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, RECORDS_HELP);
    }
    if options.json {
        unimplemented!()
    }

    let mut status = 0;
    for file in FileOpener::new(arguments) {
        match file.read {
            Ok(mut read) => {
                let records = StreamSplitter::new(&mut read, &options.input_record_delimiter)
                    .map_while(|r| r.ok())
                    .filter(is_not_delimiter);
                let records = match options.limit {
                    Some(limit) => {
                        Either::Right(if limit >= 0 {
                            Either::Right(records.take(limit as usize))
                        } else {
                            // TODO: For best efficiency — not reading the
                            // entire stream first when not necessary — we will
                            // probably want to implement `DoubleEndedIterator`
                            // for `StreamSplitter`.
                            Either::Left(
                                records
                                    .collect::<Vec<Record>>()
                                    .into_iter()
                                    .rev()
                                    .take(limit.unsigned_abs())
                                    .rev(),
                            )
                        })
                    }
                    None => Either::Left(records),
                };
                for (n, r) in records.enumerate() {
                    print_record(&mut stdout(), n + 1, &r.bytes, &options)?;
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
