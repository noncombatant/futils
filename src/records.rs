use std::fs::File;
use std::io::{stdin, stdout, Write};

use crate::shell::{parse_options, ShellResult};
use crate::stream_splitter::{is_not_delimiter, StreamSplitter};
use crate::util::{help, unescape_backslashes};

pub(crate) const RECORDS_HELP_MESSAGE: &str = "# `records` - splits a file into records

## Usage

```
records -h
records [-n] [-d delimiter] [-o delimiter] [pathname [...]]
```

## Description

Reads the given `pathname`s (or `stdin` if none are given), splits them into
records using the input delimiter, and prints them, delimiting them with the
output delimiter.

## Options

* `-d`: Use the given input record `delimiter`, a regular expression. The
  default delimiter is `r\"(\\r|\\n)+\"`.
* `-n`: Prefix each record with a record number.
* `-o`: Use the given output record `delimiter`. The default delimiter is `\\n`.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

## Additional Options

    -h  Prints this help message.

## Examples

```
records some-file.txt
```

This is similar to POSIX `cat`, except that it will convert any new line
sequences into your platform’s native sequence.

```
records -d '\\r\\n' -o '\\n' some-file.txt
```

As above, but explicitly convert Windows new line sequences (only) into POSIX.

```
records -o '\\0' some-file.txt
```

Delimit records in some-file.txt with the `NUL` character (`\\0`). This is
typically used together with other utilities that use `NUL` to delimit records
in a more robust way (such as when the other utilities may treat the file’s
existing delimiters as as syntactic metacharacters of some kind). For example,

```
records -o '\\0' list-of-files.txt | xargs -0 foo...
```

(See for example `filter -h`, and the `xargs`(1) and `find`(1) manual pages.)";

fn print_record(n: usize, record: &[u8], enumerate: bool, output_delimiter: &[u8]) -> ShellResult {
    if !record.is_empty() {
        if enumerate {
            let s = format!("{:05}: ", n);
            stdout().write_all(s.as_bytes())?;
        }
        stdout().write_all(record)?;
        stdout().write_all(output_delimiter)?;
    }
    Ok(0)
}

pub(crate) fn records_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, RECORDS_HELP_MESSAGE);
    }

    let output_delimiter = unescape_backslashes(&options.output_record_delimiter)?;
    let output_delimiter = output_delimiter.as_bytes();

    let mut status = 0;
    if arguments.is_empty() {
        let mut stdin = stdin();
        for (n, r) in StreamSplitter::new(&mut stdin, &options.input_field_delimiter)
            .filter(is_not_delimiter)
            .enumerate()
        {
            print_record(n + 1, &r.bytes, options.enumerate, output_delimiter)?;
        }
    } else {
        for pathname in arguments {
            match File::open(pathname) {
                Ok(mut file) => {
                    for (n, r) in StreamSplitter::new(&mut file, &options.input_field_delimiter)
                        .filter(is_not_delimiter)
                        .enumerate()
                    {
                        print_record(n + 1, &r.bytes, options.enumerate, output_delimiter)?;
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
