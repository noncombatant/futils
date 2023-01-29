use getopt::Opt;
//use std::io::{stdout, Write};
use std::process::exit;

use crate::sub_slicer::SubSlicer;
use crate::util::{map_file, run_command, unescape_backslashes, ShellResult};
use crate::{DEFAULT_INPUT_DELIMITER, DEFAULT_OUTPUT_DELIMITER};

pub fn apply_help() {
    eprintln!("TODO: apply_help");
    exit(1);
}

pub fn apply_main(arguments: &[String]) -> ShellResult {
    let mut options = getopt::Parser::new(arguments, "d:ho:x:v");

    let mut input_delimiter = String::from(DEFAULT_INPUT_DELIMITER);
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);
    let mut command = String::new();
    let mut verbose = false;

    loop {
        match options.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('d', Some(string)) => input_delimiter = string.clone(),
                Opt('h', None) => apply_help(),
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                Opt('x', Some(string)) => command = string.clone(),
                Opt('v', None) => verbose = true,
                _ => apply_help(),
            },
        }
    }

    let input_delimiter = unescape_backslashes(&input_delimiter)?;
    let input_delimiter_bytes = input_delimiter.as_bytes();
    let output_delimiter = unescape_backslashes(&output_delimiter)?;
    let _output_delimiter_bytes = output_delimiter.as_bytes();

    let (_, arguments) = arguments.split_at(options.index());

    let mut status = 0;
    if arguments.is_empty() {
        eprintln!("TODO: Reading from stdin not implemented yet. Sorry!");
        apply_help();
    } else {
        for pathname in arguments {
            match map_file(pathname) {
                Ok(mapped) => {
                    let slicer = SubSlicer {
                        slice: &mapped,
                        input_delimiter: input_delimiter_bytes,
                        start: 0,
                    };
                    for s in slicer {
                        match run_command(&command, s, verbose) {
                            Ok(s) => {
                                if s != 0 && status == 0 {
                                    status = s
                                }
                            }
                            _ => panic!("We're gonna die"),
                        };
                        // TODO: First, remove the trailing \n, if
                        // output_delimiter_bytes is not \n.
                        //stdout().write_all(output_delimiter_bytes)?;
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
