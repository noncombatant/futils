use std::io::{stdout, Write};

use crate::shell::{parse_options, FileOpener, Options, ShellResult, STDIN_PATHNAME};
use crate::stream_splitter::{is_not_delimiter, StreamSplitter};
use crate::util::{help, run_command};

/// Command line usage help.
pub(crate) const FILTER_HELP: &str = include_str!("filter_help.md");

fn print_matches(pathname: &str, splitter: StreamSplitter, options: &Options) -> ShellResult {
    let mut stdout = stdout();
    let mut matched = false;
    'outer: for r in splitter.map_while(|r| r.ok()).filter(is_not_delimiter) {
        for re in &options.prune_expressions {
            if re.is_match(&r.bytes) {
                continue 'outer;
            }
            matched = true;
        }
        for re in &options.match_expressions {
            if !re.is_match(&r.bytes) {
                continue 'outer;
            }
            matched = true;
        }
        for command in &options.match_commands {
            match run_command(command, &r.bytes, options.verbose) {
                Ok(status) => {
                    if status != 0 {
                        continue 'outer;
                    }
                    matched = true;
                }
                Err(e) => {
                    eprintln!(
                        "{} \"{}\": {}",
                        command,
                        String::from_utf8_lossy(&r.bytes),
                        e
                    );
                    continue 'outer;
                }
            }
        }
        stdout.write_all(pathname.as_bytes())?;
        stdout.write_all(&options.output_field_delimiter)?;
        stdout.write_all(&r.bytes)?;
        stdout.write_all(&options.output_record_delimiter)?;
    }
    Ok(if matched { 0 } else { 1 })
}

/// Runs the `filter` command on `arguments`.
pub(crate) fn filter_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, FILTER_HELP);
    }
    if options.json {
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
