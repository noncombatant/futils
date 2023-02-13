use getopt::Opt;
use regex::bytes::Regex;
use std::io::{stdout, Write};

use crate::predicate::Predicate;
use crate::sub_slicer::SubSlicer;
use crate::util::{help, map_file, unescape_backslashes, ShellResult};
use crate::{DEFAULT_INPUT_DELIMITER, DEFAULT_OUTPUT_DELIMITER};

const HELP_MESSAGE: &str = "filter - filter records from files by patterns

Usage:

    filter -h
    filter [-d string] [-m regex] [-o string] [-p regex] [-x command] file [...]

Searches the given *file*(s) for records that match the given specifications:

    -m  Print records that match the given regular expression.
    -p  Do not print (i.e. prune) records that match the given regular
        expression.
    -x  Print records for which the given *command* exited with status 0.

BUG: Currently, you must, and can only, supply, exactly 1 of -m, -p, or -x. I’ll
fix this soon.

Regular expressions use the Rust regex library syntax
(https://docs.rs/regex/latest/regex/).

Additional options:

    -h  Print this help message.
    -d  Use the given input record delimiter. The default delimiter is “\\n”.
    -o  Use the given output record delimiter. The default delimiter is “\\n”.
    -v  Print the standard output of commands given with the -x option. (By
        default, *files* only prints their standard error.)";

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
                Opt('h', None) => help(0, HELP_MESSAGE),
                Opt('m', Some(string)) => match_expression = string.clone(),
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                Opt('p', Some(string)) => prune_expression = string.clone(),
                Opt('x', Some(string)) => match_command = string.clone(),
                _ => help(-1, HELP_MESSAGE),
            },
        }
    }

    let conditions = [&match_expression, &prune_expression, &match_command];
    let count = conditions.iter().filter(|i| !i.is_empty()).count();
    if count != 1 {
        // TODO: Make it possible to pass more than 1, and AND them all
        // together.
        eprintln!("Use exactly 1 of -m, -p, or -x.");
        help(-1, HELP_MESSAGE);
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
        help(-1, HELP_MESSAGE);
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
