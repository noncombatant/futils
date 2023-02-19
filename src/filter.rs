use getopt::Opt;
use regex::bytes::Regex;
use std::io::{stdout, Write};

use crate::sub_slicer::SubSlicer;
use crate::util::{help, map_file, run_command, unescape_backslashes, ShellResult};
use crate::{DEFAULT_INPUT_RECORD_DELIMITER, DEFAULT_OUTPUT_RECORD_DELIMITER};

pub const FILTER_HELP_MESSAGE: &str = "filter - filter records from files by patterns

Usage:

```
filter -h
filter [-v] [-d delimeter] [-m regex] [-o delimiter] [-p regex] [-x command]
       pathname [...]
```

Searches the given `pathname`(s) for records that match the given
specifications:

* `-m`: Print records that match the given regular expression.
* `-p`: Do not print (i.e. prune) records that match the given regular
  expression.
* `-x`: Print records for which the given `command` exited with status 0.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

Additional options:

* `-h`: Print this help message.
* `-d`: Use the given input record `delimiter`. The default delimiter is `\\n`.
* `-o`: Use the given output record `delimiter`. The default delimiter is `\\n`.
* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `filter` only prints their standard error.)";

pub fn filter_main(arguments: &[String]) -> ShellResult {
    // TODO: Somehow, make this whole options parsing chunk reusable.
    let mut options = getopt::Parser::new(arguments, "d:hm:o:p:x:");
    let mut input_delimiter = String::from(DEFAULT_INPUT_RECORD_DELIMITER);
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_RECORD_DELIMITER);
    let mut match_expressions = Vec::new();
    let mut prune_expressions = Vec::new();
    let mut match_commands = Vec::new();
    let mut verbose = false;

    loop {
        match options.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('d', Some(string)) => input_delimiter = string.clone(),
                Opt('h', None) => help(0, FILTER_HELP_MESSAGE),
                Opt('m', Some(string)) => match_expressions.push(Regex::new(&string)?),
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                Opt('p', Some(string)) => prune_expressions.push(Regex::new(&string)?),
                Opt('x', Some(string)) => match_commands.push(string.clone()),
                Opt('v', None) => verbose = true,
                _ => help(-1, FILTER_HELP_MESSAGE),
            },
        }
    }

    let input_delimiter = unescape_backslashes(&input_delimiter)?;
    let input_delimiter_bytes = input_delimiter.as_bytes();
    let output_delimiter = unescape_backslashes(&output_delimiter)?;
    let output_delimiter_bytes = output_delimiter.as_bytes();

    let (_, arguments) = arguments.split_at(options.index());

    let mut status = 0;
    if arguments.is_empty() {
        eprintln!("TODO: Reading from stdin not implemented yet. Sorry!");
        help(-1, FILTER_HELP_MESSAGE);
    } else {
        for pathname in arguments {
            // TODO: Separate all this out into a function; it's too deeply nested.
            match map_file(pathname) {
                Ok(mapped) => {
                    let slicer = SubSlicer {
                        slice: &mapped,
                        input_delimiter: input_delimiter_bytes,
                        start: 0,
                    };

                    'outer: for s in slicer {
                        for re in &prune_expressions {
                            if re.is_match(s) {
                                continue 'outer;
                            }
                        }
                        for re in &match_expressions {
                            if !re.is_match(s) {
                                continue 'outer;
                            }
                        }
                        for command in &match_commands {
                            match run_command(command, s, verbose) {
                                Ok(status) => {
                                    if status != 0 {
                                        continue 'outer;
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "{} \"{}\": {}",
                                        command,
                                        String::from_utf8_lossy(s),
                                        e
                                    );
                                    continue 'outer;
                                }
                            }
                        }

                        stdout().write_all(s)?;
                        stdout().write_all(output_delimiter_bytes)?;
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
