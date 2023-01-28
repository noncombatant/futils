use getopt::Opt;
use regex::bytes::Regex;
use std::io::{stdout, Write};
use std::process::exit;
use walkdir::{DirEntry, WalkDir};

use crate::predicate::Predicate;
use crate::util::{unescape_backslashes, ShellResult};
use crate::DEFAULT_OUTPUT_DELIMITER;

fn is_hidden(e: &DirEntry) -> bool {
    match e.path().to_str() {
        Some(s) => s.contains("/."),
        None => false,
    }
}

pub fn files_help() {
    eprintln!("TODO: files_help");
    exit(1);
}

pub fn files_main(arguments: &[String]) -> ShellResult {
    let mut options = getopt::Parser::new(&arguments, "ahm:o:p:x:");
    let mut show_all = false;
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);
    let mut match_expression = String::new();
    let mut prune_expression = String::new();
    let mut match_command = String::new();

    loop {
        match options.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('a', None) => show_all = true,
                Opt('h', None) => files_help(),
                Opt('m', Some(string)) => match_expression = string.clone(),
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                Opt('p', Some(string)) => prune_expression = string.clone(),
                Opt('x', Some(string)) => match_command = string.clone(),
                _ => files_help(),
            },
        }
    }

    let conditions = [&match_expression, &prune_expression, &match_command];
    let count = conditions.iter().filter(|i| !i.is_empty()).count();
    if count != 1 {
        // TODO: Make it possible to pass more than 1, and AND them all
        // together.
        //eprintln!("Use exactly 1 of -m, -p, or -x.");
        //files_help();
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

    let output_delimiter = unescape_backslashes(&output_delimiter)?;
    let output_delimiter_bytes = output_delimiter.as_bytes();

    let (_, arguments) = arguments.split_at(options.index());

    let mut pathnames = vec![".".to_string()];
    if !arguments.is_empty() {
        pathnames = arguments.into()
    }
    for p in pathnames {
        // TODO: Separate all this out into a function; it's too deeply nested.
        // TODO: Apply predicate.
        let mut it = WalkDir::new(p).into_iter();
        loop {
            let entry = match it.next() {
                None => break,
                Some(e) => e,
            };
            let entry = match entry {
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
                Ok(e) => e,
            };
            if !show_all && is_hidden(&entry) {
                if entry.file_type().is_dir() {
                    it.skip_current_dir();
                }
                continue;
            }
            let p = entry.path();
            match p.to_str() {
                Some(s) => {
                    stdout().write_all(s.as_bytes())?;
                    stdout().write_all(output_delimiter_bytes)?;
                }
                None => {
                    eprintln!("pathname not valid Unicode: '{}'", p.display());
                }
            }
        }
    }
    Ok(0)
}
