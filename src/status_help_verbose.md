## Examples

To get a nice spreadsheet-like view of a large directory:

```
status | vd
status -J | vd -f json
```

`status` prints the most-often-interesting fields first. To print only the first
few fields, try this:

```
status | fields -c 1 -c 2 -c 0
```

To sort by size (`-c1` is the Size field):

```
status | fields -c1 -c2 -c0 | sort -n
```

You can also use JSON and `jq` to filter fields:

```
status -J | jq '.[] | {name, size}'
```

Or even:

```
status -J | jq '[.[] | {name, size, modified_time}]' | vd -f json
```

## See Also

* `futils help`
* `ls`(1)
* `stat`(1)
* `stat`(2)
* [`exa`](https://the.exa.website/)
* `sort`(1)
* [`vd`](https://www.visidata.org/)
* [`jq`](https://stedolan.github.io/jq)
