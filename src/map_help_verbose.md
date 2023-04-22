## Additional Options

* `-f`: Set the input field delimiter, a regular expression.
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

* `futils help`
* `find`(1), in particular the `-exec` option
* `xargs`(1)
