use getopt::Opt;
use regex::Regex;
use std::io::{stdout, Write};
use std::process::exit;
use walkdir::{DirEntry, WalkDir};

use crate::util::{run_command, unescape_backslashes, ShellResult};
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
    // TODO: Add a -t for type: file, directory, symlink, others?
    let mut options = getopt::Parser::new(&arguments, "ahm:o:p:vx:");
    let mut show_all = false;
    let mut match_expressions = Vec::new();
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);
    let mut prune_expressions = Vec::new();
    let mut verbose = false;
    let mut match_commands = Vec::new();

    loop {
        match options.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('a', None) => show_all = true,
                Opt('h', None) => files_help(),
                Opt('m', Some(string)) => match_expressions.push(Regex::new(&string)?),
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                Opt('p', Some(string)) => prune_expressions.push(Regex::new(&string)?),
                Opt('v', None) => verbose = true,
                Opt('x', Some(string)) => match_commands.push(string.clone()),
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

            for re in &prune_expressions {
                if re.is_match(pathname) {
                    if entry.file_type().is_dir() {
                        it.skip_current_dir();
                    }
                    continue 'outer;
                }
            }

            for re in &match_expressions {
                if !re.is_match(pathname) {
                    continue 'outer;
                }
            }

            for command in &match_commands {
                match run_command(command, pathname.as_bytes(), verbose) {
                    Ok(code) => {
                        if code != 0 {
                            continue 'outer;
                        }
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        continue 'outer;
                    },
                }
            }

            stdout().write_all(pathname.as_bytes())?;
            stdout().write_all(output_delimiter_bytes)?;
        }
    }
    Ok(0)
}
