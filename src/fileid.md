# `fileid` — prints information uniquely identifying files

For each `pathname` given, prints the file’s device number, inode number, size, and pathname. Prints the cryptographic hash of each file’s contents (in the first column, in columnar output) if the `-v` option is given.

## Usage

```
fileid [-v] [pathname [...]]
fileid -h [-v]
```

* `-v`: Print the cryptographic hash (currently, SHA-256) of the file’s contents.
