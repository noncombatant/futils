## Additional Options

* `-R`: Set the output record delimiter. The default delimiter is `\n`.
* `-r`: Set the input record delimiter, a regular expression.

## Exit Status

| Exit Status    | Meaning            |
|----------------|--------------------|
|              0 | Success            |
|             -1 | Generic failure    |
| greater than 1 | Number of errors   |

## Examples

Given a numbers.txt that contains a number on each line, this will yield the sum
of those numbers:

```
reduce -x + numbers.txt
```

Similarly, you ccan multiply, divide, or subtract:

```
reduce -x '*' numbers.txt
reduce -x / numbers.txt
reduce -x - numbers.txt
```

## See Also

* `futils help`
* `awk`(1)
