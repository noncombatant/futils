# `files` - print the pathnames of matching files

## Usage

```
files -h
files [-av] [-M datetime] [-m regex] [-o delimiter] [-p regex] [-t types]
      [-x command] [pathname [...]]
```

## Description

Searches the given `pathname`(s) (assuming “.” if none are given) for files that
match the given specifications:

* `-a`: Search all paths, including those containing components whose basenames
  start with “.”. By default, `files` ignores these files and directories.
* `-m`: Print pathnames that match the given regular expression.
* `-M`: Print pathnames that refer to files whose modification times match the
  given `datetime` expression (see below).
* `-p`: Do not print (i.e. prune) pathnames that match the given regular
  expression.
* `-t`: Print only pathnames that refer to files that are among the given
  `types`: `d`irectory, `f`ile, and `s`ymlink. The default value for
  `types` is `dfs`, i.e. `files` prints pathnames of all 3 types.
* `-x`: Print pathnames for which the given `command` exited with status 0.

If you give no specifications, `files` prints all pathnames (whose basenames do
not begin with `.`) under the given `path`s (or `.`). If you give multiple
specifications, they must all be satisfied for `files` to print the pathname.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

Datetime expressions have 2 parts: a comparison operator (`>` for after, `<` for
before, and `=` for exactly) and a datetime string. `files` first attempts to
parse the string as “YYYY-MM-DD HH:MM:SS”, then as “HH:MM:SS”, then as
“YYYY-MM-DD”.

## Additional Options

* `-h`: Print this help message.
* `-o`: Use the given output record `delimiter`. The default delimiter is `\n`.
* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `files` only prints their standard error.
