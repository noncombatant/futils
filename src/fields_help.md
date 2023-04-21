# `fields` — selects and formats the fields from input records

## Usage

```
fields [-IJns] [-c field] [pathname [...]]
```

## Description

Reads the given `pathname`s (or `stdin` if none are given), splits them into
records, splits each record into fields, selects the requested `field`(s), and
prints them. You can give more than 1 instance of `-c field`, to select multiple
fields.

For records that lack a requested field, prints an empty field.

## Options

* `-c`: Select the `field`(s). This option can be given multiple times, and
  fields will be output in the order given on the command line. Field numbering
  starts from 0. Negative field indices count from the end starting at -1; i.e.
  `-c-1` prints the last field of each record. If no `-c` options are given,
  `fields` will print all fields.
* `-I`: Inverts the behavior of `-c`: selects the fields *not* listed. This
  option makes no sense without at least 1 `-f` option.
* `-J`: Output JSON format.
* `-n`: Prefix each record with a record number.
* `-s`: Skip leading space characters in records.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

## Additional Options

* `-h`: Prints this help page.
* `-F`: Set the output field delimiter.
* `-f`: Set the input field delimiter, a regular expression.
* `-R`: Set the output record delimiter.
* `-r`: Set the input record delimiter, a regular expression.

## Exit Status

| Exit Status    | Meaning            |
|----------------|--------------------|
|              0 | Success            |
|             -1 | Generic failure    |
| greater than 1 | Number of errors   |

## Examples

Consider a file named farm-animals.txt with the following records of
tab-delimited fields showing how many of each animal we have, as well as their
diet:

```
1	mountain goat	grass, moss, vegetation
4	billy goats	grass, moss, vegetation, tin cans
12	sheep	grass, more grass
1,749	llamas	exclusively human flesh (for some reason)
```

We can use `fields` to select some of the fields. (The default input field
delimiter is the tab, but if we want to be explicit, we can use `-f '\t'`.) For
example, to print only the count of each animal:

```
fields -f '\t' -c 0 farm-animals.txt
```

Note that field counting begins at 0, so `-c 0` gives us the *first* field.

To print every field *except* the count, we can invert the selection with `-I`:

```
fields -I -c 0 farm-animals.txt
```

If we only want to see the animals’ diets:

```
fields -c 2 farm-animals.txt
```

## See Also

* `futils help`
* `cut`(1)
* `paste`(1)
* `colrm`(1)
