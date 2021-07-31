use getopt::Opt;
use regex::bytes::Regex;
use std::io::{stderr, stdout, Write};
use std::process::{exit, Command};
use std::str;

use crate::sub_slicer::SubSlicer;
use crate::util::{map_file, unescape_backslashes};
use crate::{DEFAULT_INPUT_DELIMITER, DEFAULT_OUTPUT_DELIMITER};

enum Predicate<'a> {
    Nothing,
    MatchCommand(&'a str),
    MatchExpression(&'a Regex),
    PruneExpression(&'a Regex),
}

// TODO: define a global `type Record = &[u8]`.

impl<'a> Predicate<'a> {
    fn evaluate(&self, record: &[u8]) -> bool {
        match self {
            Predicate::Nothing => panic!("Some goatery has occurred."),
            Predicate::MatchCommand(c) => run_command(c, record),
            Predicate::MatchExpression(e) => e.is_match(record),
            Predicate::PruneExpression(e) => !e.is_match(record),
        }
    }
}

fn run_command(command: &str, argument: &[u8]) -> bool {
    let argument = str::from_utf8(argument).unwrap();
    let error_message = "failed to execute process";

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .arg(argument)
            .output()
            .expect(error_message)
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .arg(argument)
            .output()
            .expect(error_message)
    };

    let success = output.status.success();
    if !success {
        stderr().write_all(&output.stderr).unwrap();
        match output.status.code() {
            Some(code) => exit(code),
            None => exit(1),
        }
    }
    success
}

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

    let re: Regex;
    let predicate = if !match_command.is_empty() {
        Predicate::MatchCommand(&match_command)
    } else if !match_expression.is_empty() {
        re = Regex::new(&match_expression).unwrap();
        Predicate::MatchExpression(&re)
    } else if !prune_expression.is_empty() {
        re = Regex::new(&prune_expression).unwrap();
        Predicate::PruneExpression(&re)
    } else {
        Predicate::Nothing
    };

    // TODO: Support this someday.
    //let input_delimiter = Regex::new(&input_delimiter).unwrap();
    let input_delimiter = unescape_backslashes(&input_delimiter).unwrap();
    let input_delimiter_bytes = input_delimiter.as_bytes();
    let output_delimiter = unescape_backslashes(&output_delimiter).unwrap();
    let output_delimiter_bytes = output_delimiter.as_bytes();

    let (_, arguments) = arguments.split_at(options.index());

    if arguments.is_empty() {
        eprintln!("TODO: Reading from stdin not implemented yet. Sorry!");
        filter_help();
    } else {
        for pathname in arguments {
            if let Some(mapped) = map_file(&pathname) {
                let slicer = SubSlicer {
                    slice: &mapped,
                    input_delimiter: &input_delimiter_bytes,
                    start: 0,
                };
                for s in slicer {
                    if predicate.evaluate(s) {
                        stdout().write_all(s).unwrap();
                        stdout().write_all(output_delimiter_bytes).unwrap();
                    }
                }
            }
        }
    }
}
