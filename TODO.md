# TODO

More programs: add `join` (like `join`(1)), `foldr`, `foldl`, `sum`, `zip`,
`reduce`, if reasonable. E.g.:

> reduce(function, sequence[, initial]) -> value
>
> Apply a function of two arguments cumulatively to the items of a sequence,
> from left to right, so as to reduce the sequence to a single value. For
> example, reduce(lambda x, y: x+y, [1, 2, 3, 4, 5]) calculates
> ((((1+2)+3)+4)+5).  If initial is present, it is placed before the items of
> the sequence in the calculation, and serves as a default when the sequence is
> empty.

Add JSON input and output to all commands, either by default or with `-j`.

Tests for everything.

Rustdoc for all `pub` identifiers.

Make sure that when options can be given more than once, the help strings for
every program note this.

Create a `trash` program that puts things in a trashcan, for 2-phase delete?
Hard to do safely and correctly, and therefore fun?

Rayon for e.g. `files`, et c.?

Parallelize `-x`, for sure.

Consider switching to using `clap` for parsing options, et c. This is related to
the overall problem of `OsString` vs. `String` for arguments, options,
pathnames, et c.

Use David Cook’s non-copying `StreamSplitter`.

Provide an option for `records` to print delimiters, too.

Implement `map`. Consider e.g. `ls | apply -x status -v`: it creates a new array
(of size 1) of records from the output of `status` for each record in the input.
By contrast, `ls | map -x status` would produce an array of _N_ records for each
record in the input (and no `-v`). That is, `apply` is mostly about the
side-effect; printing is a tangential option. By contrast, `map` is about
transforming records. Additionally, `map` should pass each field of the record
as a distinct argument to the `-x` command.

Every program/most programs should take the following form:

```
parse options and arguments
for each argument:
  records = process the argument
  for each record:
    write_columns or write_json
```
