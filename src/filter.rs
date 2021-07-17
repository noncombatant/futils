use getopt::Opt;
use regex::bytes::Regex;
use std::io::{stdout, Write};
use std::process::exit;

use crate::sub_slicer::SubSlicer;
use crate::util::map_file;
use crate::{DEFAULT_INPUT_DELIMITER, DEFAULT_OUTPUT_DELIMITER};

pub fn filter_help() {
    eprintln!("TODO: filter_help");
    exit(1);
}

pub fn filter_main(arguments: &[String]) {
    let mut options = getopt::Parser::new(&arguments, "d:hm:o:p:x:");

    let mut input_delimiter = String::from(DEFAULT_INPUT_DELIMITER);
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);
    let mut match_expression = String::new();
    let mut prune_expression = String::new();
    let mut match_command = String::new();

    // TODO: Since exactly 1 predicate (`match_expression`, `prune_expression`,
    // `match_command`) is allowed, we should use an `enum` instead of 3
    // variables, and then `match` on it in the main processing loop below.

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

    let conditions = [&match_expression, &prune_expression, &match_command];
    let count = conditions.iter().filter(|i| !i.is_empty()).count();
    if count != 1 {
        eprintln!("Use exactly 1 of -m, -p, or -x.");
        filter_help();
    }

    // TODO: Support this someday.
    //let input_delimiter = Regex::new(&input_delimiter).unwrap();
    let input_delimiter_bytes = input_delimiter.as_bytes();
    let output_delimiter_bytes = output_delimiter.as_bytes();

    let match_re = Regex::new(&match_expression).unwrap();
    let prune_re = Regex::new(&prune_expression).unwrap();

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
                // TODO: This should be a `match` on an `enum`, as described
                // above.
                for s in slicer {
                    if !match_expression.is_empty() {
                        if match_re.is_match(s) {
                            stdout().write_all(s).unwrap();
                            stdout().write_all(output_delimiter_bytes).unwrap();
                        }
                    } else if !prune_expression.is_empty() {
                        if !prune_re.is_match(s) {
                            stdout().write_all(s).unwrap();
                            stdout().write_all(output_delimiter_bytes).unwrap();
                        }
                    } else if !match_command.is_empty() {
                        // TODO
                    }
                }
            }
        }
    }
}
