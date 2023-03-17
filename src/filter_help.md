# `filter` — filter records from files by patterns

## Usage

```
filter [-v] [-m regex] [-p regex] [-x command] [pathname [...]]
```

## Description

Searches the given `pathname`(s) (or `stdin`, if none are given) for records
that match the given specifications.

If you give no specifications, `filter` prints all records.

## Options

* `-m`: Print records that match the regular expression.
* `-p`: Do not print (i.e. prune) records that match the regular expression.
* `-x`: Print records for which `command` exited with status 0.

You can provide more than 1 of any of the `-m`, `-p`, and `-x` options. `filter`
prints only records that match all specifications.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

## Additional Options

* `-h`: Print this help message.
* `-d`: Set the input record delimiter. The default delimiter is `r"(\r|\n)+"`.
* `-O`: Set the output field delimiter. The default delimiter is `\t`.
* `-o`: Set the output record delimiter. The default delimiter is `\n`.
* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `filter` only prints their standard error.

## Examples

TODO
