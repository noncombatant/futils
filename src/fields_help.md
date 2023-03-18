# `fields` — selects and formats the fields from input records

## Usage

```
fields [-Fns] [-f field] [pathname [...]]
```

## Description

Reads the given `pathname`s (or `stdin` if none are given), splits them into
records, splits each record into fields, selects the requested `field`(s), and
prints them. You can give more than 1 instance of `-f field`, to select multiple
fields.

TODO: Add `-n` and `-l`, like `records`? At that point, `records` is just an
alias for/simplified form of `fields`. That might be ok!

## Options

* `-F`: Inverts the behavior of `-f`: selects the fields *not* listed. This
  option makes no sense without at least 1 `-f` option.
* `-f`: Select the `field`(s). This option can be given multiple times, and
  fields will be output in the order given on the command line. Field numbering
  starts from 0. Negative field indices count from the end starting at -1; i.e.
  `-f-1` prints the last field of each record. If no `-f` options are given,
  `fields` will print all fields.
* `-n`: Prefix each record with a record number.
* `-s`: Skip leading space characters in records.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

## Additional Options

* `-D`: Set the input field delimiter, a regular expression. The default
  delimiter is `r"\s+"`.
* `-d`: Set the input record delimiter, a regular expression. The default
  delimiter is `r"(\r|\n)+"`.
* `-h`: Prints this help page.
* `-O`: Set the output field delimiter. The default delimiter is `\t`.
* `-o`: Set the output record delimiter. The default delimiter is `\n`.

## Examples

TODO

## See Also

* `futils help`
* TODO
