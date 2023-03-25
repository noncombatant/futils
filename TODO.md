# TODO

More programs: add `join` (like `join`(1)), `foldr`, `foldl`, `sum`, `zip`,
`reverse`, if reasonable.

Unit and integration tests for everything.

Rustdoc for all top-level and `pub` identifiers.

Make sure that when options can be given more than once, the help strings for
every program note this.

Create a `trash` program that puts things in a trashcan, for 2-phase delete?
Hard to do safely and correctly, and therefore fun?

Create a Markdown printing function and program, like `glow`, and use it when
printing all help messages.

Rayon for e.g. `files`, et c.?

Parallelize `-x` when appropriate.

Consider switching to using `clap` for parsing options, et c. This is related to
the overall problem of `OsString` vs. `String` for arguments, options,
pathnames, et c.

Use David Cook’s non-copying `StreamSplitter`, in some magical future when it
can be made to `impl Iterator`. Currently, we rely too much on the `Iterator`
trait to do without it.

Provide an option for `records`, `fields`, `filter`, et c. to print the matched
input delimiters, too. Alternately: if we end up never wanting that, get rid of
the `.delimiter` field. (Especially since it costs another allocation.)

Every program/most programs should take the following form:

```
parse options and arguments
for each argument:
  records = process the argument
  for each record:
    write_columns or write_json
```

A quick-exit option for `filter`, like `grep -l`. Maybe `-q` for quick or `-b`
for Boolean.
