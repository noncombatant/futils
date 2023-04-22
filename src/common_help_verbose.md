## Additional Options

* `-F`: Set the output field delimiter.
* `-f`: Set the input field delimiter, a regular expression.
* `-R`: Set the output record delimiter.
* `-r`: Set the input record delimiter, a regular expression.

## Exit Status

| Exit Status    | Meaning            |
|----------------|--------------------|
|              0 | Success            |
|             -1 | Generic failure    |
| greater than 1 | Number of errors   |

## Examples

Select the 3rd column (`-c 2`), showing those records that occur in both files:

```
common some-file other-file | fields -c 2
```

## See Also

* `futils help`
* `futils help fields`
* `comm`(1)
* `cut`(1)
* `paste`(1)
* `colrm`(1)
