# `files` — print the pathnames of matching files

Searches the given `pathname`(s) (or the current working directory if none are
given) for files that match the given specifications.

If you give no specifications, `files` prints all pathnames (whose basenames do
not begin with a dot) under the given `path`s. If you give multiple
specifications, they must all be satisfied for `files` to print the pathname.

## Usage

```
files [-aiv] [-d depth] [-M datetime] [-m regex] [-p regex] [-t types]
      [-x command] [pathname [...]]
files -hv
```

* `-a`: Search all paths, including those containing components whose basenames
  start with a dot. By default, `files` ignores these files and directories.
* `-d`: Descend at most `depth` levels below the command line arguments in the
  directory hierarchy.
* `-i`: Use case-insensitive regular expressions for `-m` and `-p` expressions
  that come *after* the `-i` in the argument list.
* `-m`: Print pathnames that match the regular expression.
* `-M`: Print pathnames that refer to files whose modification times match the
  given `datetime` expression (see below).
* `-p`: Do not print (i.e. prune) pathnames that match the regular expression.
* `-t`: Print only pathnames that refer to files that are among the given
  `types`: `d`irectory, `f`ile, and `s`ymlink. The default value for
  `types` is `dfs`, i.e. `files` prints pathnames of all 3 types.
* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `files` only prints their standard error.)
* `-x`: Print pathnames for which `command` exited with status 0.

You can provide more than 1 of any of the `-m`, `-p`, and `-x` options. `files`
prints only files that match all specifications.

Datetime expressions have 2 parts: a comparison operator (`>` for after, `<` for
before, and `=` for exactly) and a datetime string. `files` first attempts to
parse the string as “YYYY-MM-DD HH:MM:SS”, then as “HH:MM:SS”, then as
“YYYY-MM-DD”.
