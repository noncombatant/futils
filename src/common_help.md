# `common` —  select or reject lines common to two files

## Usage

```
common [-iJ] [file1] file2
common -hv
```

## Description

Reads `file1` (or `stdin`) and `file2`, which should be sorted, and produces 3
columns as output: records only in `file1`, records only in `file2`, and records
in both files.

## Options

* `-i`: Compare records case-insensitively.
* `-J`: Output JSON format.
