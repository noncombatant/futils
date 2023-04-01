# `map` — transform records of input

## Usage

```
map [-P] -x command [pathname [...]]
```

## Description

For each record in each of the given `pathname`(s) (or `stdin` if no pathnames
are given), runs the shell command `command`. Each field of the record is given
to `command` as a distinct argument. Prints the `stdout` and `stderr` of each
command.

## Options

* `-P`: Run `command`(s) in parallel. The order of output records will not be
  deterministic when you use this option.
* `-x`: Run `command` on each record of input.

You can give more than 1 instance of `-x command`, to run multiple commands on
each input record.

## Additional Options

* `-h`: Print this help page.
* `-f`: Set the input field delimiter, a regular expression.
* `-R`: Set the output record delimiter.
* `-r`: Set the input record delimiter.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

## Exit Status

| Exit Status    | Meaning            |
|----------------|--------------------|
|              0 | Success            |
|             -1 | Generic failure    |
| greater than 1 | Number of errors   |

## Examples

To get the status of each file in the current directory:

```
files | map -x status
```

To get the status of files in JSON format:

```
files | map -x 'status -J'
```

## See Also

* `futils apply`
* `futils help`
* `xargs`(1)
