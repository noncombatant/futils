use getopt::Opt;
use regex::bytes::Regex;
use std::io::{stdout, Write};
use std::process::exit;

use crate::predicate::Predicate;
use crate::sub_slicer::SubSlicer;
use crate::util::{map_file, unescape_backslashes, ShellResult};
use crate::{DEFAULT_INPUT_DELIMITER, DEFAULT_OUTPUT_DELIMITER};

pub fn filter_help() {
    eprintln!("TODO: filter_help");
    exit(1);
}

pub fn filter_main(arguments: &[String]) -> ShellResult {
    // TODO: Somehow, make this whole options parsing chunk reusable.
    let mut options = getopt::Parser::new(arguments, "d:hm:o:p:x:");

    let mut input_delimiter = String::from(DEFAULT_INPUT_DELIMITER);
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);
    let mut match_expression = String::new();
    let mut prune_expression = String::new();
    let mut match_command = String::new();

    loop {
        match options.next().transpose()? {
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
        // TODO: Make it possible to pass more than 1, and AND them all
        // together.
        eprintln!("Use exactly 1 of -m, -p, or -x.");
        filter_help();
    }

    let re: Regex;
    let predicate = if !match_command.is_empty() {
        Predicate::MatchCommand(&match_command)
    } else if !match_expression.is_empty() {
        re = Regex::new(&match_expression)?;
        Predicate::MatchExpression(&re)
    } else if !prune_expression.is_empty() {
        re = Regex::new(&prune_expression)?;
        Predicate::PruneExpression(&re)
    } else {
        Predicate::Nothing
    };

    let input_delimiter = unescape_backslashes(&input_delimiter)?;
    let input_delimiter_bytes = input_delimiter.as_bytes();
    let output_delimiter = unescape_backslashes(&output_delimiter)?;
    let output_delimiter_bytes = output_delimiter.as_bytes();

    let (_, arguments) = arguments.split_at(options.index());

    let mut status = 0;
    if arguments.is_empty() {
        eprintln!("TODO: Reading from stdin not implemented yet. Sorry!");
        filter_help();
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
                        if predicate.evaluate(s) {
                            stdout().write_all(s)?;
                            stdout().write_all(output_delimiter_bytes)?;
                        }
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
