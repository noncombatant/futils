# `map` — transform records of input

For each record in each of the given `pathname`(s) (or `stdin` if no pathnames
are given), runs the shell command `command`. Each record is given to `command`
as a distinct argument. Prints the `stdout` and `stderr` of each command.

## Usage

```
map [-l limit] [-P] -x command [pathname [...]]
map -hv
```

* `-l`: By default, `map` will give 1 record as an argument to the `command`
  per invocation. If `limit` is greater than 0, `map` will pass that many
  records to `command` as arguments. Because it can result in many fewer
  invocations, this can be a good way to reduce run times when there are many
  records. (See Examples in the verbose help.)
* `-P`: Run `command`(s) in parallel. The order of output records will not be
  deterministic when you use this option.
* `-x`: Run `command` on each record of input.

You can give more than 1 instance of `-x command`, to run multiple commands on
each input record.
