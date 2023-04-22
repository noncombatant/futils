# `status` — print the status of files

## Usage

```
status [-v] [pathname [...]]
status -hv
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

* `-v`: Verbose output (all fields).
