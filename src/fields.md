# `fields` — selects and formats the fields from input records

Reads the given `pathname`s (or `stdin` if none are given), splits them into records, splits each record into fields, selects the requested `field`(s), and prints them. You can give more than 1 instance of `-c field`, to select multiple fields.

For records that lack a requested field, prints an empty field.

## Usage

```
fields [-Ins] [-c field] [pathname [...]]
fields -hv
```

* `-c`: Select the `field`(s). This option can be given multiple times, and fields will be output in the order given on the command line. Field numbering starts from 0. Negative field indices count from the end starting at -1; i.e. `-c-1` prints the last field of each record. If no `-c` options are given, `fields` will print all fields.
* `-I`: Inverts the behavior of `-c`: selects the fields *not* listed. This option makes no sense without at least 1 `-c` option.
* `-n`: Prefix each record with the file’s pathname and a record number.
* `-s`: Skip leading space characters in records.
