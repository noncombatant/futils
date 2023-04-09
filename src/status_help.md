# `status` — print the status of files

## Usage

```
status [-v] [pathname [...]]
status [-J] [pathname [...]]
```

## Description

Prints the filesystem metadata for each of the given `pathname`s in JSON format.
If no pathnames are given, prints the status for each file in `.`.

The metadata elements are:

* `name`: name
* `file_type`: type: regular file, directory, et c.
* `size`: size in bytes
* `modified_time`: last modified time
* `user`: user-owner
* `group`: group-owner
* `permissions`: type and permissions
* `links`: number of hard links
* `device`: device number
* `inode`: inode number
* `accessed_time`: last accessed time
* `changed_time`: last changed time (metadata change)
* `birth_time`: birth time
* `mode`: underlying `mode` field
* `blocks`: number of storage blocks used
* `block_size`: size of storage blocks

For columns output (no `-J`), by default, only the `file_type`, `permissions`,
`links`, `user`, `group`, `size`, `modified_time`, and `name` fields are printed
(in that order). To see all fields, pass the `-v` option. For JSON output
(`-J`), all fields are printed.

## Options

* `-J`: Output JSON format.
* `-v`: Verbose output (all fields).

## Additional Options

* `-F`: Set the output field delimiter.
* `-h`: Print this help page.
* `-R`: Set the output record delimiter.

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
