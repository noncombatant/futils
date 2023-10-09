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

BUG: `-l` is not working for `filter`.

Make a mode for `map` such that, if there is no `-x`, treat the arguments as a
command.

Consider adding an option to force Markdown formatting on/off.

For `filter`, `fields`, and `records`, add an option to turn printing the input
pathname on/off.

Wrap `terminal_text` into a new program, for viewing Markdown in the terminal.
