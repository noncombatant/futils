## Examples

To get the status of each file in the current directory:

```
files | mapx -l 20 status
```

To get the status of files in JSON format:

```
files | mapx status -J
```

To search all Go files for `Foo`:

```
files -m '\.go$' | mapx -l 100 filter -m Foo
```

## See Also

* `futils help`
* `map -hv`
* `find`(1), in particular the `-exec` option
* `xargs`(1)
