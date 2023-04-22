# TODO

More programs: `join` (like `join`(1)). A `trash` program that puts things in a
trashcan, for 2-phase delete? Hard to do safely and correctly, and therefore
fun? A Markdown printing function and program, like `glow`, and use it when
printing all help messages.

Completeness: Unit and integration tests for everything. Rustdoc for all
top-level and `pub` identifiers.

Use David Cook’s non-copying `StreamSplitter`, in some magical future when it
can be made to `impl Iterator`. Currently, we rely too much on the `Iterator`
trait to do without it.

Combine documentation for all common options into a single Markdown file, and
include it in verbose help.

Run all documented examples as actual tests with this form:

```
some command ...
output 1
output 2
output 3
```

Assert that running the command produces the output.

Write a `futils tutorial` that has a more discursive tone and lots of examples.
