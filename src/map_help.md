# `map` — transform records of input

## Usage

```
map [-P] -x command [pathname [...]]
map -hv
```

## Description

For each record in each of the given `pathname`(s) (or `stdin` if no pathnames
are given), runs the shell command `command`. Each field of the record is given
to `command` as a distinct argument. Prints the `stdout` and `stderr` of each
command.

## Options

* `-P`: Run `command`(s) in parallel. The order of output records will not be
  deterministic when you use this option.
* `-x`: Run `command` on each record of input.

You can give more than 1 instance of `-x command`, to run multiple commands on
each input record.
