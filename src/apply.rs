//! The `apply` command.

use std::fs::File;
use std::io::{stdin, stdout, Write};

use crate::shell::{parse_options, ShellResult};
use crate::stream_splitter::{is_not_delimiter, StreamSplitter};
use crate::util::{help, run_command, unescape_backslashes};

/// Command line usage help.
pub const APPLY_HELP_MESSAGE: &str = "# `apply` - apply commands to records of input

## Usage

```
apply -h
apply [-v] [-d string] [-o string] -x command [pathname [...]]
```

## Description

For each record in each of the given `pathname`(s), runs the shell command
`command`. If no pathnames are given, reads `stdin`. You can give more than 1
instance of `-x command`, to run multiple commands on each input record.

## Additional Options

* `-h`: Print this help message.
* `-d`: Use the given input record delimiter. The default delimiter is
  `r\"(\\r|\\n)+\"`.
* `-o`: Use the given output record delimiter. The default delimiter is `\\n`.
* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `apply` only prints their standard error.)";

/// Iterates over `StreamSplitter` and runs each of the `commands` on each
/// record. `verbose` enables printing `stdout` from the `commands`. Each
/// record’s output is delimited by `output_delimiter`.
fn apply(
    splitter: StreamSplitter,
    commands: &[String],
    verbose: bool,
    output_delimiter: &[u8],
) -> ShellResult {
    let mut status = 0;
    for r in splitter.filter(is_not_delimiter) {
        for command in commands {
            match run_command(command, &r.bytes, verbose) {
                Ok(s) => {
                    if s != 0 {
                        status += 1;
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", std::str::from_utf8(&r.bytes).unwrap(), e);
                    status += 1;
                }
            }
            if verbose {
                stdout().write_all(output_delimiter)?;
            }
        }
    }
    Ok(status)
}

/// Runs the `apply` command on `arguments`.
pub fn apply_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, APPLY_HELP_MESSAGE);
    }

    let input_delimiter = options.input_record_delimiter;
    let output_delimiter = unescape_backslashes(&options.output_record_delimiter)?;
    let output_delimiter = output_delimiter.as_bytes();

    let mut status = 0;
    if arguments.is_empty() {
        let mut stdin = stdin();
        apply(
            StreamSplitter::new(&mut stdin, &input_delimiter),
            &options.match_commands,
            options.verbose,
            output_delimiter,
        )?;
    } else {
        for pathname in arguments {
            match File::open(pathname) {
                Ok(mut file) => {
                    match apply(
                        StreamSplitter::new(&mut file, &input_delimiter),
                        &options.match_commands,
                        options.verbose,
                        output_delimiter,
                    ) {
                        Ok(s) => status += s,
                        Err(e) => {
                            eprintln!("{}", e);
                            status += 1;
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
