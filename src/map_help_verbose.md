## Examples

To get the status of each file in the current directory:

```
files | map -x status
```

To get the status of files in JSON format:

```
files | map -x 'status -J'
```

## See Also

* `futils help`
* `find`(1), in particular the `-exec` option
* `xargs`(1)