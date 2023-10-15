# `mapx` — transform records of input

For each record in `stdin`, runs the shell command `command` with any given `arguments`. Each record is given to `command` and `arguments` as further arguments. Prints the `stdout` and `stderr` each run of the command.

## Usage

```
mapx [-l limit] [-P] command [arguments...]
mapx -hv
```

* `-l`: By default, `map` will give 1 record as an argument to the `command` per invocation. If `limit` is greater than 0, `map` will pass that many records to `command` as arguments. Because it can result in many fewer invocations, this can be a good way to reduce run times when there are many records. (See Examples in the verbose help.)
* `-P`: Run `command`(s) in parallel. The order of output records will not be deterministic when you use this option.
