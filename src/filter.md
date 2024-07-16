# `filter` — filter records from streams using patterns

Searches the given `pathname`(s) (or `stdin`, if none are given) for records that match the given specifications.

If you give no specifications, `filter` prints all records.

## Usage

```
filter [-nSv] [-l limit] [-m regex] [-p regex] [-x command] [pathname [...]]
filter -hv
```

* `-l`: Limit the number of records printed.
  * If `limit` is <= 0, `filter` prints nothing and exits with status 0 if the input contained a matching record, and 1 otherwise.
* `-m`: Print records that match the regular expression.
* `-n`: Do not prefix each record with the file’s pathname and a record number.
* `-p`: Do not print (i.e. prune) records that match the regular expression.
* `-S`: Use case-sensitive regular expressions for `-m` and `-p` expressions that come *after* the `-S` in the argument list.
* `-v`: Print the standard output of commands given with the `-x` option. (By default, `filter` only prints their standard error.)
* `-x`: Print records for which `command` exited with status 0.

You can provide more than 1 of any of the `-m`, `-p`, and `-x` options. `filter` prints only records that match all specifications.

Regular expressions use [the Rust regex library syntax](https://docs.rs/regex/latest/regex/).
