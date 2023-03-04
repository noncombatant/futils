# Futils

A suite of functional(-like) command shell utilities.

The attempt to make modern, rational shell utilities is admittedly somewhat
futile. French speakers may pronounce it « foutils » if they like.

## Goals

* Structured data; JSON everywhere
* Modern: Unicode everywhere, complete Markdown documentation, complete tests
* Consistently flexible: regular expressions everywhere applicable
* Consistent command line structure and behavior
* At least as time- and space- efficient as classic equivalents
* Safe: Memory-safe, typeful, mistake-resistant UX, as side-effect safe as
  possible

## Consistent UX Conventions

### Help And Documentation

`-h` and a `help` sub-command are always available and always print a help
message to `stdout`. Unlike error conditions (e.g. invalid options), an explicit
request for help yields exit status 0.

All programs print their help in valid Markdown format. `foo -h | md | less -F`,
for example, should always produce readable results.

All help messages give examples.

### Exit Status

Programs that succeed exit with status 0. Non-zero statuses indicate program
failure.

+-------------+--------------------+
| Exit Status | Meaning            |
+-------------+--------------------+
|           0 | Success            |
|          -1 | Generic failure    |
|         > 1 | Number of errors   |
+-------------+--------------------+

When a program fails, it always prints a meaningful error message to `stderr`.
If a program prints its help message as a result of failure, it prints to
`stderr`.

### File Arguments And `stdin`

The normal mode of `futils` programs is stream processing. They always stream
`stdin` when given no arguments. Arguments, when present, are the pathnames of
files that the program will stream (or directories that the program will crawl,
as appropriate).

### Regular Expressions

Wherever possible and appropriate, `futils` programs use Rust regular
expressions for searching and lexical analysis.

## Development

Every `pub` identifier has a Rustdoc comment. Rustdoc is always well-formed
Markdown.

Every function has thorough unit tests.

Every command line program has thorough functional tests.

Never `panic!`.
