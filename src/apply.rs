use getopt::Opt;
use std::io::{stdout, Write};
use std::process::exit;

use crate::sub_slicer::SubSlicer;
use crate::util::{map_file, run_command, unescape_backslashes};
use crate::{DEFAULT_INPUT_DELIMITER, DEFAULT_OUTPUT_DELIMITER};

pub fn apply_help() {
    eprintln!("TODO: apply_help");
    exit(1);
}

pub fn apply_main(arguments: &[String]) {
    let mut options = getopt::Parser::new(&arguments, "d:ho:x:");

    let mut input_delimiter = String::from(DEFAULT_INPUT_DELIMITER);
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);
    let mut command = String::new();

    loop {
        match options.next().transpose().unwrap() {
            None => break,
            Some(opt) => match opt {
                Opt('d', Some(string)) => input_delimiter = string.clone(),
                Opt('h', None) => apply_help(),
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                Opt('x', Some(string)) => command = string.clone(),
                _ => apply_help(),
            },
        }
    }

    // TODO: Support this someday.
    //let input_delimiter = Regex::new(&input_delimiter).unwrap();
    let input_delimiter = unescape_backslashes(&input_delimiter).unwrap();
    let input_delimiter_bytes = input_delimiter.as_bytes();
    let output_delimiter = unescape_backslashes(&output_delimiter).unwrap();
    let output_delimiter_bytes = output_delimiter.as_bytes();

    let (_, arguments) = arguments.split_at(options.index());

    if arguments.is_empty() {
        eprintln!("TODO: Reading from stdin not implemented yet. Sorry!");
        apply_help();
    } else {
        for pathname in arguments {
            if let Some(mapped) = map_file(&pathname) {
                let slicer = SubSlicer {
                    slice: &mapped,
                    input_delimiter: &input_delimiter_bytes,
                    start: 0,
                };
                for s in slicer {
                    run_command(&command, s);
                    stdout().write_all(output_delimiter_bytes).unwrap();
                }
            }
        }
    }
}
