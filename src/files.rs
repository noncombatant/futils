use chrono::NaiveDateTime;
use regex::bytes::Regex;
use std::io::{stdout, Write};
use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

use crate::shell::{parse_options, ShellResult};
use crate::time::{Comparison, Time};
use crate::util::{help, run_command, unescape_backslashes};

pub const FILES_HELP_MESSAGE: &str = "# `files` - print the pathnames of matching files

## Usage

```
files -h
files [-av] [-M datetime] [-m regex] [-o delimiter] [-p regex] [-t types]
      [-x command] [pathname [...]]
```

## Description

Searches the given `pathname`(s) (assuming “.” if none are given) for files that
match the given specifications:

* `-a`: Search all paths, including those containing components whose basenames
  start with “.”. By default, `files` ignores these files and directories.
* `-m`: Print pathnames that match the given regular expression.
* `-M`: Print pathnames that refer to files whose modification times match the
  given `datetime` expression (see below).
* `-p`: Do not print (i.e. prune) pathnames that match the given regular
  expression.
* `-t`: Print only pathnames that refer to files that are among the given
  `types`: `d`irectory, `f`ile, and `s`ymlink. The default value for
  `types` is `dfs`, i.e. `files` prints pathnames of all 3 types.
* `-x`: Print pathnames for which the given `command` exited with status 0.

If you give no specifications, `files` prints all pathnames (whose basenames do
not begin with `.`) under the given `path`s (or `.`). If you give multiple
specifications, they must all be satisfied for `files` to print the pathname.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

Datetime expressions have 2 parts: a comparison operator (`>` for after, `<` for
before, and `=` for exactly) and a datetime string. `files` first attempts to
parse the string as “YYYY-MM-DD HH:MM:SS”, then as “HH:MM:SS”, then as
“YYYY-MM-DD”.

## Additional Options

* `-h`: Print this help message.
* `-o`: Use the given output record `delimiter`. The default delimiter is `\\n`.
* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `files` only prints their standard error.)";

fn is_hidden(e: &DirEntry) -> bool {
    match e.path().to_str() {
        Some(s) => s.contains("/."),
        None => false,
    }
}

fn compare_times(e: &DirEntry, t: &Time) -> Result<bool, std::io::Error> {
    let metadata = e.metadata()?;
    let modified = metadata.modified()?;
    let modified = modified
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let modified = NaiveDateTime::from_timestamp_opt(modified.try_into().unwrap(), 0).unwrap();
    let given = t.date_time;
    Ok(match t.comparison {
        Comparison::After => given <= modified,
        Comparison::Before => given >= modified,
        Comparison::Exactly => given == modified,
    })
}

#[allow(clippy::too_many_arguments)]
fn print_matches(
    pathname: &str,
    show_all: bool,
    file_types: &str,
    prune_expressions: &[Regex],
    match_expressions: &[Regex],
    mtime_expressions: &[Time],
    match_commands: &[String],
    verbose: bool,
    output_delimiter: &[u8],
) -> ShellResult {
    let mut it = WalkDir::new(pathname).into_iter();
    let mut status = 0;
    'outer: loop {
        let entry = match it.next() {
            None => break Ok(status),
            Some(e) => e,
        };
        let entry = match entry {
            Err(e) => {
                eprintln!("{}", e);
                status += 1;
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
                status += 1;
                continue;
            }
        };

        for re in prune_expressions {
            if re.is_match(pathname.as_bytes()) {
                if entry.file_type().is_dir() {
                    it.skip_current_dir();
                }
                continue 'outer;
            }
        }

        for re in match_expressions {
            if !re.is_match(pathname.as_bytes()) {
                continue 'outer;
            }
        }

        for mtime in mtime_expressions {
            match compare_times(&entry, mtime) {
                Ok(true) => continue,
                Ok(false) => continue 'outer,
                Err(e) => {
                    eprintln!("{}", e);
                    status += 1;
                    continue 'outer;
                }
            }
        }

        for command in match_commands {
            match run_command(command, pathname.as_bytes(), verbose) {
                Ok(status) => {
                    if status != 0 {
                        continue 'outer;
                    }
                }
                Err(e) => {
                    eprintln!("{} \"{}\": {}", command, pathname, e);
                    status += 1;
                    continue 'outer;
                }
            }
        }

        stdout().write_all(pathname.as_bytes())?;
        stdout().write_all(output_delimiter)?;
    }
}

pub fn files_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, FILES_HELP_MESSAGE);
    }

    let output_delimiter = options.output_record_delimiter;
    let output_delimiter = unescape_backslashes(&output_delimiter)?;
    let output_delimiter = output_delimiter.as_bytes();

    let mut pathnames = vec![".".to_string()];
    if !arguments.is_empty() {
        pathnames = arguments.into()
    }
    let mut status = 0;
    for p in pathnames {
        match print_matches(
            &p,
            options.show_all,
            &options.file_types,
            &options.prune_expressions,
            &options.match_expressions,
            &options.mtime_expressions,
            &options.match_commands,
            options.verbose,
            output_delimiter,
        ) {
            Ok(s) => status += s,
            Err(e) => {
                eprintln!("{}: {}", p, e);
                status += 1;
            }
        }
    }
    Ok(status)
}
