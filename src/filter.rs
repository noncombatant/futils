//! The `futils filter` command.

use std::io::{stdout, Write};

use crate::shell::{parse_options, FileOpener, Options, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::StreamSplitter;
use crate::util::{help, run_command};

/// Command line usage help.
pub(crate) const FILTER_HELP: &str = include_str!("filter_help.md");

pub(crate) const FILTER_HELP_VERBOSE: &str = include_str!("filter_help_verbose.md");

/// TODO: Define an `EnumeratedMatch`, like `EnumeratedRecord`, and give it
/// `write_{columns,json}`.

fn print_matches(pathname: &str, splitter: StreamSplitter, options: &Options) -> ShellResult {
    let mut stdout = stdout();
    let mut matched = false;
    'outer: for (n, r) in splitter
        .map_while(Result::ok)
        .enumerate()
        .filter(|pair| !pair.1.data.is_empty())
    {
        for re in &options.prune_expressions {
            if re.is_match(&r.data) {
                continue 'outer;
            }
            matched = true;
            if options.limit == Some(0) {
                break 'outer;
            }
        }
        for re in &options.match_expressions {
            if !re.is_match(&r.data) {
                continue 'outer;
            }
            matched = true;
            if options.limit == Some(0) {
                break 'outer;
            }
        }
        for command in &options.match_commands {
            match run_command(command, &[&r.data], options.verbose) {
                Ok(status) => {
                    if status != 0 {
                        continue 'outer;
                    }
                    matched = true;
                    if options.limit == Some(0) {
                        break 'outer;
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{} \"{}\": {}",
                        command,
                        String::from_utf8_lossy(&r.data),
                        e
                    );
                    continue 'outer;
                }
            }
        }

        stdout.write_all(pathname.as_bytes())?;
        stdout.write_all(&options.output_field_delimiter)?;
        if options.enumerate {
            write!(stdout, "{}", n + 1)?;
            stdout.write_all(&options.output_field_delimiter)?;
        }
        stdout.write_all(&r.data)?;
        stdout.write_all(&options.output_record_delimiter)?;
    }
    Ok(if matched { 0 } else { 1 })
}

/// Runs the `filter` command on `arguments`.
pub(crate) fn filter_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(
            0,
            FILTER_HELP,
            if options.verbose {
                Some(FILTER_HELP_VERBOSE)
            } else {
                None
            },
        );
    }
    if options.json_input || options.json_output {
        unimplemented!()
    }

    let mut status = 0;
    for file in FileOpener::new(arguments) {
        let pathname = file.pathname.unwrap_or(&STDIN_PATHNAME);
        match file.read {
            Ok(mut read) => {
                let s = print_matches(
                    pathname,
                    StreamSplitter::new(&mut read, &options.input_record_delimiter),
                    &options,
                )?;
                if s != 0 {
                    status += 1;
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
