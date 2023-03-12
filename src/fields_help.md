# `fields` — selects and formats the fields from input records

## Usage

```
fields -h
fields [-Fns] [-D delimiter] [-d delimiter] [-O delimiter] [-o delimiter] [-f field] [pathname [...]]
```

## Description

Reads the given `pathname`s (or `stdin` if none are given), splits them into
records using the input delimiter, splits each record into fields using the
field delimiter, selects the requested `field`(s), and prints them, delimiting
them with the output field and record delimiters.

## Options

* `-D`: Use the given input field `delimiter`, a regular expression. The
  default delimiter is `r\"\\s+\"`.
* `-d`: Use the given input record `delimiter`, a regular expression. The
  default delimiter is `r\"(\\r|\\n)+\"`.
* `-F`: Inverts the behavior of `-f`: selects the fields *not* listed. This
  option makes no sense without at least 1 `-f` option.
* `-f`: Select the given `field`(s). This option can be given multiple times,
  and fields will be output in the order given on the command line. Field
  numbering starts from 1. If no `-f` options are given, `fields` will print all
  fields.
* `-n`: Prefix each record with a record number.
* `-s`: Skip leading space characters in records.
* `-O`: Use the given output field `delimiter`. The default delimiter is `\\t`.
* `-o`: Use the given output record `delimiter`. The default delimiter is `\\n`.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

## Additional Options

* `-h`: Prints this help message.
