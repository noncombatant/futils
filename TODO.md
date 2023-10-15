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
to the command, as opposed to `map`, which treats the whole record as a single
argument.

For `filter`, `fields`, and `records`, add an option to turn printing the input
pathname on/off.

Organize everything into chained iterators: `StreamSplitter`; but then also
`Matcher`, `Enumerator`, `PathnamePrefixer`, ... Then, each program should
mostly become a thin wrapper around the iterators that chains them in the right
way.
