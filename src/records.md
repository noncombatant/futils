# `records` — splits a stream into records

Reads the given `pathname`s (or `stdin` if none are given), splits them into
records, and prints them.

## Usage

```
records [-eJn] [-l limit] [pathname [...]]
records -hv
```

* `-e`: Print empty records, too.
* `-l`: Limit the number of records printed. If `limit` is < 0, the limit is
  counted back from the last record in the input.
* `-n`: Prefix each record with a record number.
