# TODO

More programs: add `join` (like `join`(1)), `foldr`, `foldl`, `sum`, `zip`,
`reduce`, `reverse`, if reasonable. E.g.:

> reduce(function, sequence[, initial]) -> value
>
> Apply a function of two arguments cumulatively to the items of a sequence,
> from left to right, so as to reduce the sequence to a single value. For
> example, reduce(lambda x, y: x+y, [1, 2, 3, 4, 5]) calculates
> ((((1+2)+3)+4)+5).  If initial is present, it is placed before the items of
> the sequence in the calculation, and serves as a default when the sequence is
> empty.

Enable JSON output for all commands with `-j`.

Enable JSON input for all commands with `-J`.

Unit and integration tests for everything.

Rustdoc for all top-level and `pub` identifiers.

Make sure that when options can be given more than once, the help strings for
every program note this.

Create a `trash` program that puts things in a trashcan, for 2-phase delete?
Hard to do safely and correctly, and therefore fun?

Rayon for e.g. `files`, et c.?

Parallelize `-x` when appropriate.

Consider switching to using `clap` for parsing options, et c. This is related to
the overall problem of `OsString` vs. `String` for arguments, options,
pathnames, et c.

Use David Cook’s non-copying `StreamSplitter`.

Provide an option for `records` (and `fields`?) to print delimiters, too.

Every program/most programs should take the following form:

```
parse options and arguments
for each argument:
  records = process the argument
  for each record:
    write_columns or write_json
```

A quick-exit option for `filter`, like `grep -l`.
