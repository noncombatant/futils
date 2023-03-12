# TODO

More programs: add `join` (like `join`(1)), `foldr`, `foldl`, `sum`, `zip`,
`reduce`, `map`, if reasonable.

Add JSON output to all commands, either by default or with `-j`.

Add a `json2fixed` command (or extend `fields`) that turns JSON into a table
view, so that people don’t have to use `vd -f json -b -o -` all the time.

Similarly, provide or find a good Markdown-to-terminal-escape program. Possibly
https://github.com/charmbracelet/glow.

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

Enable negative field indices in `fields`, and a way to suppress fields while
printing all others (perhaps `-F`).
