//! The `futils records` command.

use std::io::{stdout, Error, Write};

use atty::Stream;
use itertools::Either;
use serde::Serialize;

use crate::shell::{parse_options, FileOpener, Options, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::{Record, StreamSplitter};
use crate::util::help;

/// Command line usage help.
pub(crate) const RECORDS_HELP: &str = include_str!("records_help.md");

#[derive(Serialize)]
struct EnumeratedRecord {
    n: Option<usize>,
    r: Record,
}

impl EnumeratedRecord {
    fn write_columns(&self, output: &mut dyn Write, options: &Options) -> Result<(), Error> {
        if options.print_empty || !self.r.data.is_empty() {
            if let Some(n) = self.n {
                write!(output, "{}", n + 1)?;
                output.write_all(&options.output_field_delimiter)?;
            }
            output.write_all(&self.r.data)?;
            output.write_all(&options.output_record_delimiter)?;
        }
        Ok(())
    }

    fn write_json(
        &self,
        output: &mut dyn Write,
        pretty: bool,
        options: &Options,
    ) -> Result<(), Error> {
        if options.print_empty || !self.r.data.is_empty() {
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

/// Runs the `records` command on `arguments`.
pub(crate) fn records_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, RECORDS_HELP);
    }

    let mut status = 0;
    let mut stdout = stdout();
    for file in FileOpener::new(arguments) {
        match file.read {
            Ok(mut read) => {
                let records = StreamSplitter::new(&mut read, &options.input_record_delimiter)
                    .map_while(Result::ok);
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

                if options.json_output {
                    println!("[");
                }
                for er in records.enumerate().map(|pair| EnumeratedRecord {
                    n: if options.enumerate {
                        Some(pair.0)
                    } else {
                        None
                    },
                    r: pair.1,
                }) {
                    if options.json_output {
                        er.write_json(&mut stdout, atty::is(Stream::Stdout), &options)?;
                        stdout.write_all(b",\n")?;
                    } else {
                        er.write_columns(&mut stdout, &options)?;
                    }
                }
                if options.json_output {
                    println!("{{}}]");
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
