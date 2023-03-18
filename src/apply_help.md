# `apply` — apply commands to records of input

## Usage

```
apply [-v] -x command [pathname [...]]
```

## Description

For each record in each of the given `pathname`(s), runs the shell command
`command`. If no pathnames are given, reads `stdin`.

## Options

* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `apply` only prints their standard error.)
* `-x`: Run `command` on each record of input.

You can give more than 1 instance of `-x command`, to run multiple commands on
each input record.

## Additional Options

* `-d`: Set the input record delimiter. The default delimiter is `r"(\r|\n)+"`.
* `-h`: Print this help page.
* `-o`: Set the output record delimiter. The default delimiter is `\n`.

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
* TODO
