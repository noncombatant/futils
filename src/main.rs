use getopt::Opt;
use regex::Regex;
use std::env;
use std::io::{stdout, Write};
use std::process::exit;

mod sub_slicer;
mod util;

use sub_slicer::SubSlicer;
use util::map_file;

// TODO: Support regex someday.
//static DEFAULT_INPUT_DELIMITER: &str = r"(\r\n|\n|\r)";
static DEFAULT_INPUT_DELIMITER: &str = "\n";
static DEFAULT_OUTPUT_DELIMITER: &str = "\n";

// Main functions

fn filter_help() {
    eprintln!("TODO: filter_help");
    exit(1);
}

fn filter_main(arguments: &[String]) {
    let mut options = getopt::Parser::new(&arguments, "d:hm:o:p:x:");

    let mut input_delimiter = String::from(DEFAULT_INPUT_DELIMITER);
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);
    let mut match_expression = String::new();
    let mut prune_expression = String::new();
    let mut match_command = String::new();
    loop {
        match options.next().transpose().unwrap() {
            None => break,
            Some(opt) => match opt {
                Opt('d', Some(string)) => input_delimiter = string.clone(),
                Opt('h', None) => filter_help(),
                Opt('m', Some(string)) => match_expression = string.clone(),
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                Opt('p', Some(string)) => prune_expression = string.clone(),
                Opt('x', Some(string)) => match_command = string.clone(),
                _ => filter_help(),
            },
        }
    }

    // TODO: Support this someday.
    //let input_delimiter = Regex::new(&input_delimiter).unwrap();
    let input_delimiter_bytes = input_delimiter.as_bytes();
    let output_delimiter_bytes = output_delimiter.as_bytes();

    let match_expression = Regex::new(&input_delimiter).unwrap();
    let prune_expression = Regex::new(&input_delimiter).unwrap();

    let (_, arguments) = arguments.split_at(options.index());

    if arguments.is_empty() {
        // TODO: read stdin
    } else {
        for pathname in arguments {
            if let Some(mapped) = map_file(&pathname) {
                let slicer = SubSlicer {
                    slice: &mapped,
                    input_delimiter: &input_delimiter_bytes,
                    start: 0,
                };
                for s in slicer {
                    if true
                    /* TODO */
                    {
                        stdout().write_all(s).unwrap();
                        stdout().write_all(b"\n").unwrap();
                    }
                }
            }
        }
    }
}

fn records_help() {
    eprintln!("TODO: records_help");
    exit(1);
}

fn records_main(arguments: &[String]) {
    let mut options = getopt::Parser::new(&arguments, "d:ho:");

    let mut input_delimiter = String::from(DEFAULT_INPUT_DELIMITER);
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);
    loop {
        match options.next().transpose().unwrap() {
            None => break,
            Some(opt) => match opt {
                Opt('d', Some(string)) => input_delimiter = string.clone(),
                Opt('h', None) => records_help(),
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                _ => records_help(),
            },
        }
    }

    // TODO: Support this someday.
    //let input_delimiter = Regex::new(&input_delimiter).unwrap();
    let input_delimiter_bytes = input_delimiter.as_bytes();
    let output_delimiter_bytes = output_delimiter.as_bytes();

    let (_, arguments) = arguments.split_at(options.index());

    if arguments.is_empty() {
        // TODO: read stdin
    } else {
        for pathname in arguments {
            if let Some(mapped) = map_file(&pathname) {
                let slicer = SubSlicer {
                    slice: &mapped,
                    input_delimiter: &input_delimiter_bytes,
                    start: 0,
                };
                for s in slicer {
                    stdout().write_all(s).unwrap();
                    stdout().write_all(output_delimiter_bytes).unwrap();
                }
            }
        }
    }
}

fn fsutil_help() {
    eprintln!("TODO: fsutil_help");
    exit(1);
}

fn main() {
    let mut arguments = env::args().collect::<Vec<String>>();
    if arguments[0].ends_with("futils") {
        if arguments.len() < 2 {
            fsutil_help();
        }
        arguments.remove(0);
    }

    match arguments[0].as_str() {
        "filter" => filter_main(&arguments),
        "records" => records_main(&arguments),
        _ => fsutil_help(),
    }
}
