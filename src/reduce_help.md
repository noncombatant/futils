# `reduce` — apply a function to reduce input records

For each record in the input, applies a `command` of two arguments cumulatively
so as to reduce the sequence to a single value. For example, `reduce -x +`
produces the numeric sum of all input records.

## Usage

```
reduce [-v] -x command [pathname [...]]
reduce [-v] -x + [pathname [...]]
reduce [-v] -x - [pathname [...]]
reduce [-v] -x '*' [pathname [...]]
reduce [-v] -x / [pathname [...]]
reduce -hv
```

* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `reduce` only prints their standard error.)
* `-x`: Run `command` on each record of input.

TODO: Describe what happens if multiple `-x`s are given.
TODO: Maybe have some way to print the input records too.
