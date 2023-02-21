use regex::bytes::Regex;
use std::fs::File;
use std::io::{stdin, stdout, Write};

use crate::shell::{parse_options, ShellResult};
use crate::stream_splitter::{is_not_delimiter, StreamSplitter};
use crate::util::{help, run_command, unescape_backslashes};

pub const FILTER_HELP_MESSAGE: &str = "# `filter` - filter records from files by patterns

## Usage

```
filter -h
filter [-v] [-d delimiter] [-m regex] [-o delimiter] [-p regex] [-x command]
       [pathname [...]]
```

## Description

Searches the given `pathname`(s) (or `stdin`, if none are given) for records
that match the given specifications:

* `-m`: Print records that match the given regular expression.
* `-p`: Do not print (i.e. prune) records that match the given regular
  expression.
* `-x`: Print records for which the given `command` exited with status 0.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

## Additional Options

* `-h`: Print this help message.
* `-d`: Use the given input record `delimiter`. The default delimiter is
  `r\"(\\r|\\n)+\"`.
* `-o`: Use the given output record `delimiter`. The default delimiter is `\\n`.
* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `filter` only prints their standard error.)";

fn print_matches(
    splitter: StreamSplitter,
    prune_expressions: &[Regex],
    match_expressions: &[Regex],
    match_commands: &[String],
    verbose: bool,
    output_delimiter: &[u8],
) -> ShellResult {
    'outer: for r in splitter.filter(is_not_delimiter) {
        for re in prune_expressions {
            if re.is_match(&r.bytes) {
                continue 'outer;
            }
        }
        for re in match_expressions {
            if !re.is_match(&r.bytes) {
                continue 'outer;
            }
        }
        for command in match_commands {
            match run_command(command, &r.bytes, verbose) {
                Ok(status) => {
                    if status != 0 {
                        continue 'outer;
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{} \"{}\": {}",
                        command,
                        String::from_utf8_lossy(&r.bytes),
                        e
                    );
                    continue 'outer;
                }
            }
        }
        stdout().write_all(&r.bytes)?;
        stdout().write_all(output_delimiter)?;
    }
    Ok(0)
}

pub fn filter_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, FILTER_HELP_MESSAGE);
    }

    let output_delimiter = unescape_backslashes(&options.output_record_delimiter)?;
    let output_delimiter = output_delimiter.as_bytes();

    let mut status = 0;
    if arguments.is_empty() {
        let mut stdin = stdin();
        print_matches(
            StreamSplitter::new(&mut stdin, &options.input_record_delimiter),
            &options.prune_expressions,
            &options.match_expressions,
            &options.match_commands,
            options.verbose,
            output_delimiter,
        )?;
    } else {
        for pathname in arguments {
            match File::open(pathname) {
                Ok(mut file) => {
                    print_matches(
                        StreamSplitter::new(&mut file, &options.input_record_delimiter),
                        &options.prune_expressions,
                        &options.match_expressions,
                        &options.match_commands,
                        options.verbose,
                        output_delimiter,
                    )?;
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
