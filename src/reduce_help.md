# `reduce` — apply a function to reduce input records

## Usage

```
reduce [-v] -x command [pathname [...]]
reduce [-v] -x + [pathname [...]]
reduce [-v] -x - [pathname [...]]
reduce [-v] -x '*' [pathname [...]]
reduce [-v] -x / [pathname [...]]
```

## Description

For each record in the input, applies a `command` of two arguments cumulatively
so as to reduce the sequence to a single value. For example, `reduce -x +`
produces the numeric sum of all input records.

## Options

* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `reduce` only prints their standard error.)
* `-x`: Run `command` on each record of input.

## Additional Options

* `-d`: Set the input record delimiter, a regular expression. The default
  delimiter is `r"(\r|\n)+"`.
* `-h`: Prints this help page.
* `-o`: Set the output record delimiter. The default delimiter is `\n`.

## Exit Status

| Exit Status    | Meaning            |
|----------------|--------------------|
|              0 | Success            |
|             -1 | Generic failure    |
| greater than 1 | Number of errors   |

## Examples

Given a numbers.txt that contains a number on each line, this will yield the sum
of those numbers:

```
reduce -x + numbers.txt
```

Similarly, you ccan multiply, divide, or subtract:

```
reduce -x '*' numbers.txt
reduce -x / numbers.txt
reduce -x - numbers.txt
```

## See Also

* `futils help`
* `awk`(1)
