## Examples

Print the version metadata keys and values separated by a colon and a space, instead of a tab as usual:

```
futils version -F': '
```

Show just the major version key and its value:

```
futils version | filter -m major
```

## See Also

* `futils help`
* [Semantic Versioning](https://semver.org/)
