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

* `apply`
* `fields`
* `files`
* `filter`
* `help`
* `map`
* `records`
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

For example, to get help for `apply`:

```
futils apply -h
futils help apply
```

You can also invoke `futils` utilities directly, for example:

```
apply -h
files -h
```

The help pages are always marked up in Markdown format. You can pipe them to a
Markdown translator or display program:

```
records -h | glow -p
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

* `-D`: Set the input field delimiter, a regular expression. The default
  delimiter is `r"\s+"`.
* `-d`: Set the input record delimiter, a regular expression. The default
  delimiter is `r"(\r|\n)+"`.
* `-h`: Prints the help page.
* `-j`: Output JSON format.
* `-O`: Set the output field delimiter. The default delimiter is `\t`.
* `-o`: Set the output record delimiter. The default delimiter is `\n`.

### Matching Input And Output Delimiters

Say you wanted to use `futils` to do the equivalent of the classic pipeline

```
find ... -print0 | xargs -0 ...
```

In this example, `find` uses `NUL` (`\0`) as an output record delimiter, and
`xargs` uses it as an input record delimiter. You might think the equivalent
with `futils` would be:

```
files -o '\0' ... | apply -d '\0' ...
```

However, `\0` is not a valid regular expression. You’ll get an error message
like this:

```
regex parse error:
    \0
    ^^
error: backreferences are not supported
```

Instead, express the input delimiter as a valid Rust regular expression; in the
case of `NUL`, the hexadecimal byte literal `\x00` works:

```
files -o '\0' ... | apply -d '\x00' ...
```

This is a bit annoying. TODO: Consider whether to treat strings that don’t parse
as regexes as string literals instead. That could lead to problems; could: print
the error and continue anyway; print the error and ask the person if they want
to continue anyway.

## See Also

* TODO
