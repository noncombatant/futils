# `files` — print the pathnames of matching files

## Usage

```
files [-av] [-M datetime] [-m regex] [-p regex] [-t types]
      [-x command] [pathname [...]]
```

## Description

Searches the given `pathname`(s) (assuming “.” if none are given) for files that
match the given specifications.

If you give no specifications, `files` prints all pathnames (whose basenames do
not begin with `.`) under the given `path`s (or `.`). If you give multiple
specifications, they must all be satisfied for `files` to print the pathname.

## Options

* `-a`: Search all paths, including those containing components whose basenames
  start with “.”. By default, `files` ignores these files and directories.
* `-m`: Print pathnames that match the regular expression.
* `-M`: Print pathnames that refer to files whose modification times match the
  given `datetime` expression (see below).
* `-p`: Do not print (i.e. prune) pathnames that match the regular expression.
* `-t`: Print only pathnames that refer to files that are among the given
  `types`: `d`irectory, `f`ile, and `s`ymlink. The default value for
  `types` is `dfs`, i.e. `files` prints pathnames of all 3 types.
* `-x`: Print pathnames for which `command` exited with status 0.

You can provide more than 1 of any of the `-m`, `-p`, and `-x` options. `files`
prints only files that match all specifications.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

Datetime expressions have 2 parts: a comparison operator (`>` for after, `<` for
before, and `=` for exactly) and a datetime string. `files` first attempts to
parse the string as “YYYY-MM-DD HH:MM:SS”, then as “HH:MM:SS”, then as
“YYYY-MM-DD”.

## Additional Options

* `-h`: Print this help page.
* `-o`: Set the output record delimiter. The default delimiter is `\n`.
* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `files` only prints their standard error.

## Exit Status

| Exit Status    | Meaning            |
|----------------|--------------------|
|              0 | Success            |
|             -1 | Generic failure    |
| greater than 1 | Number of errors   |

## Examples

To show all files that contain a record matching “foo”:

```
files -x 'filter -m foo' ...
```

To show all files that contain a record not matching “foo”:

```
files -x 'filter -p foo' ...
```

To show all files that do not contain a record matching “foo”, we need to treat
the whole file as a single record, and then try to prune (`-p`) that record. So
we need to come up with an input record delimiter that never appears in the
file; `\x00` often works for this purpose, for text files at least. For example:

```
files -x 'filter -d '\x00' -p foo' ...
```

TODO: Update those with the quick-exit feature once it’s specified.

## See Also

* `futils help`
* TODO
