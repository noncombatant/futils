# `filter` — filter records from streams using patterns

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

* `-h`: Print this help page.
* `-d`: Set the input record delimiter. The default delimiter is `r"(\r|\n)+"`.
* `-O`: Set the output field delimiter. The default delimiter is `\t`.
* `-o`: Set the output record delimiter. The default delimiter is `\n`.
* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `filter` only prints their standard error.

## Examples

Consider a file called farm-animals.txt containing the following records:

```
1	mountain goat
4	billy goats
12	sheep
1,749	llamas
```

You can use `filter` to select records from this file, as follows.

To show only the lines in the file that contain goats:

```
filter -m goat farm-animals.txt
```

As above, but search case-insensitively: this will match “Goats”, “goat”,
“GOATS”, and so on:

```
filter -m '(?i)goat' farm-animals.txt
```

It’s unclear why anyone would want only *non*-goat animals, but this is how to
do that:

```
filter -p '(?i)goat' farm-animals.txt
```

To print a list of the animals for which the (hypothetical) program
`check-if-hungry` succeeds:

```
filter -x check-if-hungry farm-animals.txt
```

If you need to pass arguments to a `-x` command, use a quoted string:

```
filter -x 'check-if-hungry --dinner' farm-animals.txt
```
