use getopt::Opt;
use regex::Regex;
use std::io::{stdout, Write};
use std::process::exit;
use walkdir::{DirEntry, WalkDir};

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
    let mut options = getopt::Parser::new(&arguments, "ahm:o:");
    let mut show_all = false;
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);

    let mut match_expressions = Vec::new();

    loop {
        match options.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('a', None) => show_all = true,
                Opt('h', None) => files_help(),
                Opt('m', Some(string)) => match_expressions.push(Regex::new(&string)?),
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                _ => files_help(),
            },
        }
    }

    let output_delimiter = unescape_backslashes(&output_delimiter)?;
    let output_delimiter_bytes = output_delimiter.as_bytes();

    let (_, arguments) = arguments.split_at(options.index());

    let mut pathnames = vec![".".to_string()];
    if !arguments.is_empty() {
        pathnames = arguments.into()
    }
    for p in pathnames {
        // TODO: Separate all this out into a function; it's too deeply nested.
        let mut it = WalkDir::new(p).into_iter();
        'outer: loop {
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
            let pathname = match p.to_str() {
                Some(s) => s,
                None => {
                    eprintln!("pathname not valid Unicode: '{}'", p.display());
                    continue;
                }
            };

            for re in &match_expressions {
                if !re.is_match(pathname) {
                    continue 'outer;
                }
            }

            stdout().write_all(pathname.as_bytes())?;
            stdout().write_all(output_delimiter_bytes)?;
        }
    }
    Ok(0)
}
