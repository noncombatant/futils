## Additional Options

* `-R`: Set the output record delimiter. The default delimiter is `\n`.
* `-r`: Set the input record delimiter, a regular expression.

Regular expressions use [the Rust regex library
syntax](https://docs.rs/regex/latest/regex/).

## Exit Status

| Exit Status    | Meaning            |
|----------------|--------------------|
|              0 | Success            |
|             -1 | Generic failure    |
| greater than 1 | Number of errors   |

## Examples

```
records some-file.txt
```

This is similar to POSIX `cat`, except that it will convert any line break
sequences into `\n`. This is due to the default values of `-d` and `-o`.

```
records -r '\r\n' -R '\n' some-file.txt
```

As above, but explicitly convert DOS/Windows/Internet new line sequences (only)
into POSIX.

```
records -R '\0' some-file.txt
```

Delimit records in some-file.txt with the `NUL` character (`\0`). This is
typically used together with other utilities that use `NUL` to delimit records
in a more robust way (such as when the other utilities may treat the file’s
existing delimiters as as syntactic metacharacters of some kind). For example,

```
records -R '\0' list-of-files.txt | xargs -0 foo...
```

With its `-l` option, `records` can work somewhat like `head` and `tail`.
Positive limits work like `head`, while negative limits work like `tail`. Try
these examples:

```
head -n5 your-file.txt
records -l5 your-file.txt
tail -n5 your-file.txt
records -l-5 your-file.txt
```

## See Also

* `futils help`
* `filter -h`
* `xargs`(1)
* `find`(1)
* `head`(1)
* `tail`(1)
