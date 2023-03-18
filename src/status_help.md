# `status` — print the status of files

## Usage

```
status [-j] [pathname [...]]
```

## Description

Prints the filesystem metadata for each of the given `pathname`s in JSON format.
If no pathnames are given, prints the status for each file in `.`.

The metadata elements are:

* `name`: name
* `device`: device number
* `mode`: type and permissions
* `links`: number of hard links
* `inode`: inode number
* `user`: user-owner
* `group`: group-owner
* `accessed_time`: last accessed time
* `modified_time`: last modified time
* `changed_time`: last changed time (metadata change)
* `birth_time`: birth time
* `size`: size in bytes
* `blocks`: number of storage blocks used
* `block_size`: size of storage blocks

## Options

* `-j`: Output JSON format.

## Additional Options

* `-h`: Print this help page.
* `-O`: Set the output field delimiter. The default delimiter is `\t`.
* `-o`: Set the output record delimiter. The default delimiter is `\n`.

## Exit Status

| Exit Status    | Meaning            |
|----------------|--------------------|
|              0 | Success            |
|             -1 | Generic failure    |
| greater than 1 | Number of errors   |

## Examples

To get a nice spreadsheet-like view of a large directory:

```
status | vd
status -j | vd -f json
```

`status` prints the most-often-interesting fields first. To print only the first
few fields, try this:

```
status | fields -D '\t' -f 1 -f 2 -f 0
```

To sort by size (`-f1` is the Size field):

```
status | fields -D'\t' -f1 -f2 -f0 | sort -n
```

You can also use JSON and `jq` to filter fields:

```
status -j | jq '.[] | {name, size}'
```

Or even:

```
status -j | jq '[.[] | {name, size, modified_time}]' | vd -f json
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
