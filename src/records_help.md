# `records` — splits a file into records

## Usage

```
records [-n] [-l limit] [pathname [...]]
```

## Description

Reads the given `pathname`s (or `stdin` if none are given), splits them into
records, and prints them.

## Options

* `-l`: Limit the number of records printed. If `limit` is < 0, the limit is
  counted back from the last record in the input.
* `-n`: Prefix each record with a record number.

## Additional Options

* `-d`: Set the input record delimiter, a regular expression. The default
  delimiter is `r"(\r|\n)+"`.
* `-h`: Prints this help message.
* `-o`: Set the output record delimiter. The default delimiter is `\n`.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

## Examples

```
records some-file.txt
```

This is similar to POSIX `cat`, except that it will convert any line break
sequences into `\n`.

```
records -d '\r\n' -o '\n' some-file.txt
```

As above, but explicitly convert Windows new line sequences (only) into POSIX.

```
records -o '\0' some-file.txt
```

Delimit records in some-file.txt with the `NUL` character (`\0`). This is
typically used together with other utilities that use `NUL` to delimit records
in a more robust way (such as when the other utilities may treat the file’s
existing delimiters as as syntactic metacharacters of some kind). For example,

```
records -o '\0' list-of-files.txt | xargs -0 foo...
```

See also `filter -h`, and the `xargs`(1) and `find`(1) manual pages.
