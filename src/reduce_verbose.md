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
