## Examples

To show all files that contain a record matching “foo”:

```
files -x 'filter -l0 -m foo' ...
```

To show all files that contain a record not matching “foo”:

```
files -x 'filter -l0 -p foo' ...
```

To show all files that do not contain a record matching “foo”, we need to treat the whole file as a single record, and then try to prune (`-p`) that record. So we need to come up with an input record delimiter that never appears in the file; `\x00` often works for this purpose, for text files at least. For example:

```
files -x 'filter -l0 -R '\x00' -p foo' ...
```

To show (`-v`), for all Rust source code files (`.rs`), lines matching “goat” (`-m goat`) and the line numbers (`-n`):

```
files -m '\.rs$' -x 'filter -n -m goat' -v
```

Another way to do this is:

```
files -m '\.rs$' | map -x 'filter -n -m goat'
```

Match all Markdown files, even if the file extension is not lowercase:

```
files -m '\.md$'
```

Match all Markdown files, but only if the extension is lowercase:

```
files -S -m '\.md$'
```

Show all Markdown files, except those whose names case-insensitively match “goat”:

```
files -m '\.md$' -p goat
```

## See Also

* `futils help`
* `find`(1)
* `ls`(1)
