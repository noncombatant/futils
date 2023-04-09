# `apply` — apply commands to records of input

## Usage

```
apply [pathname [...]]
```

## Description

For each record in each of the given `pathname`(s), treats the first field of
the record as a shell command, and runs it with the rest of the fields (if any)
as its arguments. If no pathnames are given, reads `stdin`.

## Additional Options

* `-f`: Set the input field delimiter.
* `-h`: Print this help page.
* `-R`: Set the output record delimiter.
* `-r`: Set the input record delimiter.

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
* `futils map`
* `find`(1), in particular the `-exec` option
* `xargs`(1)
