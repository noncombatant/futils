# TODO

More programs: `join` (like `join`(1)). A `trash` program that puts things in a
trashcan, for 2-phase delete? Hard to do safely and correctly, and therefore
fun?

Completeness: Unit and integration tests for everything. Rustdoc for all
top-level and `pub` identifiers.

Use David Cook’s non-copying `StreamSplitter`, in some magical future when it
can be made to `impl Iterator`. Currently, we rely too much on the `Iterator`
trait to do without it.

Run all documented examples as actual tests with this form:

```
some command ...
output 1
output 2
output 3
```

Assert that running the command produces the output.

Write a `futils tutorial` that has a more discursive tone and lots of examples.

Resurrect `apply` and have it treat each record’s fields as distinct arguments
to the command, as opposed to `map`/`mapx`, which treats the whole record as a
single argument.

It should be possible to do this:

```sh
status | (head -n1 && tail -n +2 | sort -k 5)
```

with `records` in place of `head` and `tail`. I.e. there need to be options for
skipping ahead.

Maybe there should also be an option to suppress the header.

Make `write_columns` and `write_json` into a trait `StructuredWrite`, and wrap
them into a `write` function of that trait that takes `&Options` (or, `verbose`
and `json_output`) and picks the right one.

Overall pattern: “walk and awk”

Make `pretty: bool` be a member of `Options` and a command-line switch.

Might be better off with `try_for_each` than with `map_while`.

`-J` should not imply `-v`.

Remove `atty` dependency, if possible.
