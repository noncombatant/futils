use getopt::Opt;
use regex::bytes::Regex;
use std::fs::File;
use std::io::{stdin, stdout, Write};

use crate::shell::ShellResult;
use crate::stream_splitter::StreamSplitter;
use crate::util::{help, run_command, unescape_backslashes};
use crate::{DEFAULT_INPUT_RECORD_DELIMITER, DEFAULT_OUTPUT_RECORD_DELIMITER};

pub const APPLY_HELP_MESSAGE: &str = "# `apply` - apply commands to records of input

## Usage

```
apply -h
apply [-v] [-d string] [-o string] -x command pathname [...]
```

## Description

For each record in each of the given `pathname`(s), runs the shell command
`command`.

TODO: You can only provide 1 instance of the `-x` option. It’d be cool to be
able to pass several.

## Additional Options

* `-h`: Print this help message.
* `-d`: Use the given input record delimiter. The default delimiter is `\\n`.
* `-o`: Use the given output record delimiter. The default delimiter is `\\n`.
* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `apply` only prints their standard error.)";

pub fn apply_main(arguments: &[String]) -> ShellResult {
    let mut options = getopt::Parser::new(arguments, "d:ho:x:v");
    let mut input_delimiter = Regex::new(DEFAULT_INPUT_RECORD_DELIMITER)?;
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_RECORD_DELIMITER);
    let mut command = String::new();
    let mut verbose = false;

    loop {
        match options.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('d', Some(string)) => input_delimiter = Regex::new(&string)?,
                Opt('h', None) => help(0, APPLY_HELP_MESSAGE),
                Opt('o', Some(string)) => output_delimiter = unescape_backslashes(&string)?,
                Opt('x', Some(string)) => command = string.clone(),
                Opt('v', None) => verbose = true,
                _ => help(-1, APPLY_HELP_MESSAGE),
            },
        }
    }

    let output_delimiter = output_delimiter.as_bytes();

    let (_, arguments) = arguments.split_at(options.index());

    let mut status = 0;
    if arguments.is_empty() {
        let mut stdin = stdin();
        for r in StreamSplitter::new(&mut stdin, &input_delimiter).filter(|r| !r.is_delimiter) {
            match run_command(&command, &r.bytes, verbose) {
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
    } else {
        for pathname in arguments {
            match File::open(pathname) {
                Ok(mut file) => {
                    for r in
                        StreamSplitter::new(&mut file, &input_delimiter).filter(|r| !r.is_delimiter)
                    {
                        match run_command(&command, &r.bytes, verbose) {
                            Ok(s) => {
                                if s != 0 {
                                    status += 1;
                                }
                            }
                            Err(e) => {
                                eprintln!("{}: {}", std::str::from_utf8(&r.bytes).unwrap(), e);
                                status += 1;
                            }
                        };
                        if verbose {
                            stdout().write_all(output_delimiter)?;
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
