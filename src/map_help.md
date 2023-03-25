# `map` — transform records of input

## Usage

```
map -x command [pathname [...]]
```

## Description

For each record in each of the given `pathname`(s), runs the shell command
`command` with each field of the record as a distinct argument. If no pathnames
are given, reads `stdin`.

## Options

* `-x`: Run `command` on each record of input.

You can give more than 1 instance of `-x command`, to run multiple commands on
each input record.

## Additional Options

* `-h`: Print this help page.
* `-f`: Set the input field delimiter, a regular expression. The default
  delimiter is `r"\s+"`.
* `-R`: Set the output record delimiter. The default delimiter is `\n`.
* `-r`: Set the input record delimiter. The default delimiter is `r"(\r|\n)+"`.
* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `apply` only prints their standard error.)

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

## Exit Status

| Exit Status    | Meaning            |
|----------------|--------------------|
|              0 | Success            |
|             -1 | Generic failure    |
| greater than 1 | Number of errors   |

## Examples

TODO

## See Also

* `futils help`
* TODO
