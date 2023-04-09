# `futils` — functional shell utilities

## Usage

```
futils -h
futils help
futils version
```

## Description

`futils` is a suite of shell utilities that somewhat resemble functional
programming primitives and operate on streams. The suite currently consists of
the following programs:

* `fields`
* `files`
* `filter`
* `help`
* `map`
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
futils help sub-command
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
```

The help pages are always marked up in Markdown format. You can pipe them to a
Markdown translator or display program:

```
records -h | glow -p
map -h | bat -l md
fields -h | md-to-html > fields-help.html
```

## Concepts

Most `futils` programs operate on **streams** of bytes, usually either from
files on disk (identified by **pathname**) or from the **standard input**
(`stdin`).

Most programs treat the stream of bytes as being composed of **records**, each
of which contains 0 or more **fields**. Records and fields are lexically
**delimited** by strings, called **delimiters**.

When reading a stream, we use regular expressions to describe and scan for the
record and field delimiters. (Of course, literal strings also count as regular
expressions, if that’s what you need.)

When writing an output stream, the record and field delimiters are string
literals.

Classic Unix programs (such as `cut`, `paste`, `lam`, `nl`, `find`, `xargs`,
`grep`, `join`, et c.) often have only ad hoc and limited ways to delimit
records and fields in input and output. And, of course, there are many dialects
of regular expression in use in various Unix programs. With `futils`, strings
are always Unicode (UTF-8) and regular expressions have the power (and syntax)
of [the Rust regex library syntax](https://docs.rs/regex/latest/regex/).

`futils` programs can also write their output in JSON format.

## Command Line Options

`futils` tries to be as consistent as possible, so most command line flags mean
the same thing in most programs.

Some command line flags are common to most `futils` programs. For example:

* `-F`: Set the output field delimiter. The default delimiter is `\t`.
* `-f`: Set the input field delimiter, a regular expression. The default
  delimiter is `r"\t"`.
* `-h`: Prints the help page.
* `-J`: Output JSON format.
* `-j`: Input JSON format.
* `-R`: Set the output record delimiter. The default delimiter is `\n`.
* `-r`: Set the input record delimiter, a regular expression. The default
  delimiter is `r"(\r\n|\n|\r)"`.

### Matching Input And Output Delimiters

Say you wanted to use `futils` to do the equivalent of the classic pipeline

```
find ... -print0 | xargs -0 ...
```

In this example, `find` uses `NUL` (`\0` or `\x00`) as an output record
delimiter, and `xargs` uses it as an input record delimiter. You might think the
equivalent with `futils` would be:

```
files -R '\0' ... | map -r '\0' ...
```

However, `\0` is not a valid regular expression. You’ll get an error message
like this:

```
regex parse error:
    \0
    ^^
error: backreferences are not supported
```

Rust’s regular expression library (`regex`) doesn’t follow the same rules as its
string lexer (specifically `rustc_lexer::unescape`).

The way to avoid this problem, express the input delimiter as a valid Rust
regular expression; in the case of `NUL`, the hexadecimal byte literal `\x00`
works:

```
files -R '\0' ... | map -r '\x00' ...
```

That also works for lexing strings, thankfully, so you can be consistent and use
it everywhere:

```
files -R '\x00' ... | map -r '\x00' ...
```

## See Also

* Classic Unix text processing tools, such as
  * `awk`(1)
  * `cut`(1)
  * `paste`(1)
  * `colrm`(1)
* `find`(1), `xargs`(1)
