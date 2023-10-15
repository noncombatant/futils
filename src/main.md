# `futils` — functional shell utilities

`futils` is a suite of shell utilities which somewhat resemble functional programming primitives and which operate on streams.

## Usage

```
futils -h
futils help
futils version
```

## Details

The suite currently consists of the following programs:

* `common`
* `fields`
* `files`
* `filter`
* `help`
* `map`
* `mapx`
* `markdown`
* `records`
* `reduce`
* `status`
* `version`

To learn more about each one, run

```
futils program -h
```

or

```
futils help program
```

For example, to get help for `map`:

```
futils map -h
futils help map
```

You can also invoke `futils` utilities directly, for example:

```
map -h
files -h
...
```

To get verbose help, with more information about options common to all commands, usage examples, and more, use `-v`:

```
map -hv
files -hv
...
```

The help pages are marked up in Markdown format. `futils` renders the Markdown with terminal escape codes when printed to a terminal. By default, if `stdout` is a file or pipe, `futils` prints the text with no formatting. You can set the `MANCOLOR` environment variable to force escape-code rendering:

```
MANCOLOR=yes records -h | less -R
MANCOLOR=yes fields -h > fields.help
```

`futils` will also honor the `MANWIDTH` variable, if set. (See `man`(1) for history.)

TODO: Consider implementing a way to make `futils` print un-rendered Markdown, so that people can process it with some other program.

## Concepts

Most `futils` programs operate on **streams** of bytes, usually either from files on disk (identified by **pathname**) or from the **standard input** (`stdin`).

Most programs treat the stream of bytes as being composed of **records**, each of which contains 0 or more **fields**. Records and fields are lexically **delimited** by regular expressions, called **delimiters**.

When reading a stream, we use regular expressions to describe and scan for the record and field delimiters. (Of course, literal strings also count as regular expressions, if that’s what you need.)

When writing an output stream, the record and field delimiters are string literals.

Classic Unix programs (such as `cut`, `paste`, `lam`, `nl`, `find`, `xargs`, `grep`, `join`, et c.) often have only ad hoc and limited ways to delimit records and fields in input and output. And, of course, there are many dialects of regular expression in use in various Unix programs. With `futils`, strings are always Unicode (UTF-8) and regular expressions have the power (and syntax) of [the Rust regex library syntax](https://docs.rs/regex/latest/regex/).

`futils` programs can also write their output in JSON format.

## Command Line Options

`futils` tries to be as consistent as possible, so most command line flags mean the same thing in most programs.

Some command line flags are common to most `futils` programs. For details on them, pass `-hv` to any `futils` program.

### Matching Input And Output Delimiters

Say you wanted to use `futils` to do the equivalent of the classic pipeline

```
find ... -print0 | xargs -0 ...
```

In this example, `find` uses `NUL` (`\0` or `\x00`) as an output record delimiter, and `xargs` uses it as an input record delimiter. You might think the equivalent with `futils` would be:

```
files -R '\0' ... | map -r '\0' ...
```

However, `\0` is not a valid regular expression. You’ll get an error message like this:

```
regex parse error:
    \0
    ^^
error: backreferences are not supported
```

Rust’s regular expression library (`regex`) doesn’t follow the same rules as its string lexer (specifically `rustc_lexer::unescape`).

To avoid this problem, express the input delimiter as a valid Rust regular expression. In the case of `NUL`, the hexadecimal byte literal `\x00` works:

```
files -R '\0' ... | map -r '\x00' ...
```

That also works for lexing strings, thankfully, so you can be consistent and use it everywhere:

```
files -R '\x00' ... | map -r '\x00' ...
```

## Environment Variables

* `MANWIDTH`: `markdown` and `-h` will limit text output to the number of columns given in this variable’s value. If not present, the text width will be the width of the terminal.
* `MANCOLOR`: `markdown` and `-h` will render Markdown with terminal escape codes if this value is set or if `stdout` is a terminal. Otherwise, they will render Markdown as plain text.

## See Also

* Classic Unix text processing tools, such as
  * `awk`(1)
  * `cut`(1)
  * `paste`(1)
  * `colrm`(1)
* `find`(1)
* `man`(1)
* `xargs`(1)
