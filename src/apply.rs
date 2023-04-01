//! The `futils apply` command.

use std::io::{stdout, Write};

use crate::shell::{parse_options, FileOpener, Options, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::StreamSplitter;
use crate::util::{help, run_command};

/// Command line usage help.
pub(crate) const APPLY_HELP: &str = include_str!("apply_help.md");

// TODO: Make `apply` differ from `map`: No `-x`; instead, the 1st field of each
// record is the command.

/// Iterates over `StreamSplitter` and runs each of the `commands` on each
/// record. `verbose` enables printing `stdout` from the `commands`. Each
/// record’s output is delimited by `output_delimiter`.
fn apply(splitter: StreamSplitter, options: &Options) -> ShellResult {
    let mut stdout = stdout();
    let mut status = 0;
    for r in splitter.map_while(|r| r.ok()) {
        for command in &options.match_commands {
            let fields = options
                .input_field_delimiter
                .split(&r.data)
                .collect::<Vec<&[u8]>>();
            match run_command(command, &fields, options.verbose) {
                Ok(s) => {
                    if s != 0 {
                        status += 1;
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", String::from_utf8_lossy(&r.data), e);
                    status += 1;
                }
            }
            if options.verbose {
                stdout.write_all(&options.output_record_delimiter)?;
            }
        }
    }
    Ok(status)
}

/// Runs the `apply` command on `arguments`.
pub(crate) fn apply_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, APPLY_HELP);
    }
    if options.json_input || options.json_output {
        unimplemented!()
    }

    let mut status = 0;
    for file in FileOpener::new(arguments) {
        let pathname = file.pathname.unwrap_or(&STDIN_PATHNAME);
        match file.read {
            Ok(mut read) => {
                match apply(
                    StreamSplitter::new(&mut read, &options.input_record_delimiter),
                    &options,
                ) {
                    Ok(s) => status += s,
                    Err(e) => {
                        eprintln!("{}: {}", pathname, e);
                        status += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("{}: {}", pathname, e);
                status += 1;
            }
        }
    }
    Ok(status)
}
