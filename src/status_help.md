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

* `-h`: Print this help message.
* `-O`: Set the output field delimiter. The default delimiter is `\t`.
* `-o`: Set the output record delimiter. The default delimiter is `\n`.

## Examples

TODO
