# `filter` - filter records from files by patterns

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
  `r"(\r|\n)+"`.
* `-O`: Use the given output field `delimiter`. The default delimiter is `\t`.
* `-o`: Use the given output record `delimiter`. The default delimiter is `\n`.
* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `filter` only prints their standard error.
