## Additional Options

* `-F`: Set the output field delimiter.
* `-f`: Set the input field delimiter, a regular expression.
* `-J`: Output JSON format.
* `-j`: Parse the input as JSON.
* `-R`: Set the output record delimiter.
* `-r`: Set the input record delimiter, a regular expression.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

## Exit Status

| Exit Status    | Meaning            |
|----------------|--------------------|
|              0 | Success            |
|             -1 | Generic failure    |
| greater than 1 | Number of errors   |
