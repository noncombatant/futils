# `common` —  select or reject lines common to two files

Reads `file1` (or `stdin`) and `file2`, which should both be sorted, and produces 3 columns as output: records only in `file1`, records only in `file2`, and records in both files.

## Usage

```
common [-i] [file1] file2
common -hv
```

* `-i`: Compare records case-insensitively.
