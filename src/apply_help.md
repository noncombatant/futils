# `apply` - apply commands to records of input

## Usage

```
apply -h
apply [-v] [-d string] [-o string] -x command [pathname [...]]
```

## Description

For each record in each of the given `pathname`(s), runs the shell command
`command`. If no pathnames are given, reads `stdin`. You can give more than 1
instance of `-x command`, to run multiple commands on each input record.

## Additional Options

* `-h`: Print this help message.
* `-d`: Use the given input record delimiter. The default delimiter is
  `r"(\r|\n)+"`.
* `-o`: Use the given output record delimiter. The default delimiter is `\n`.
* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `apply` only prints their standard error.)
