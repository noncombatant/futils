use getopt::Opt;
use regex::Regex;
use std::io::{stdout, Write};
use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

use crate::time::{Comparison, Time};
use crate::util::{help, run_command, unescape_backslashes, ShellResult};
use crate::DEFAULT_OUTPUT_DELIMITER;

const HELP_MESSAGE: &str = "files - print the pathnames of matching files

Usage:

    files -h
    files [-av] [-m regex] [-o delimiter] [-p regex] [-t types]
          [-x command] [path [...]]

Searches the given *path*(s) (assuming \".\" if none are given) for files that
match the given specifications:

    -a  Search all paths, including those containing components whose names start
        with \".\".
    -m  Print pathnames that match the given regular expression.
    -p  Do not print (i.e. prune) pathnames that match the given regular
        expression.
    -t  Print only files that are among the given *types*: (d)irectory, (f)ile,
        and (s)ymlink. The default value for *types* is \"dfs\", i.e. *files*
        prints pathnames of all 3 types.
    -x  Print pathnames for which the given *command* exited with status 0.

If you give no specifications, *files* prints all pathnames under the given
*path*s (or \".\"). If you give multiple specifications, they must all be
satisfied for *files* to print the pathname.

Regular expressions use the Rust regex library syntax
(https://docs.rs/regex/latest/regex/).

Additional options:

    -h  Print this help message.
    -o  Use the given output record *delimiter*. The default delimiter is
        \"\\n\".
    -v  Print the standard output of commands given with the -x option. (By
        default, *files* only prints their standard error.)";

fn is_hidden(e: &DirEntry) -> bool {
    match e.path().to_str() {
        Some(s) => s.contains("/."),
        None => false,
    }
}

pub fn files_main(arguments: &[String]) -> ShellResult {
    let mut options = getopt::Parser::new(arguments, "ahm:M:o:p:t:vx:");
    let mut show_all = false;
    let mut match_expressions = Vec::new();
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);
    let mut prune_expressions = Vec::new();
    let mut file_types = String::from("dfs");
    let mut verbose = false;
    let mut match_commands = Vec::new();
    let mut mtime_expressions = Vec::new();

    loop {
        match options.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('a', None) => show_all = true,
                Opt('h', None) => help(0, HELP_MESSAGE),
                Opt('m', Some(string)) => match_expressions.push(Regex::new(&string)?),
                Opt('M', Some(string)) => mtime_expressions.push(Time::new(&string)?),
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                Opt('p', Some(string)) => prune_expressions.push(Regex::new(&string)?),
                Opt('t', Some(string)) => file_types = string.clone(),
                Opt('v', None) => verbose = true,
                Opt('x', Some(string)) => match_commands.push(string.clone()),
                _ => help(-1, HELP_MESSAGE),
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

            let file_type = entry.file_type();
            let is_dir = file_type.is_dir();
            let is_file = file_type.is_file();
            let is_symlink = file_type.is_symlink();
            if (is_dir && !file_types.contains('d'))
                || (is_file && !file_types.contains('f'))
                || (is_symlink && !file_types.contains('s'))
            {
                continue;
            }

            if !show_all && is_hidden(&entry) {
                if is_dir {
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

            for mtime in &mtime_expressions {
                let modified = entry
                    .metadata()
                    .unwrap()
                    .modified()
                    .unwrap()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let timestamp = TryInto::<u64>::try_into(mtime.date_time.timestamp()).unwrap();
                match mtime.comparison {
                    Comparison::After => {
                        // BUG: timestamp is local, not UTC, so these comparisons will be wrong
                        // Something like let converted: DateTime<Local> = DateTime::from(utc);
                        if timestamp > modified {
                            continue 'outer;
                        }
                    }
                    Comparison::Before => {
                        if timestamp < modified {
                            continue 'outer;
                        }
                    }
                    Comparison::Exactly => {
                        if timestamp == modified {
                            continue 'outer;
                        }
                    }
                }
            }

            for command in &match_commands {
                match run_command(command, pathname.as_bytes(), verbose) {
                    Ok(status) => {
                        if status != 0 {
                            continue 'outer;
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        continue 'outer;
                    }
                }
            }

            stdout().write_all(pathname.as_bytes())?;
            stdout().write_all(output_delimiter_bytes)?;
        }
    }
    Ok(0)
}
