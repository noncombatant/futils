## Additional Options

* `-F`: Set the output field delimiter.
* `-n`: Prefix each match with its record number.
* `-R`: Set the output record delimiter.
* `-r`: Set the input record delimiter.
* `-v`: Print the standard output of commands given with the `-x` option. (By
  default, `filter` only prints their standard error.)

## Exit Status

| Exit Status    | Meaning            |
|----------------|--------------------|
|              0 | Success            |
|             -1 | Generic failure    |
| greater than 1 | Number of errors   |

## Examples

Consider a file called farm-animals.txt containing the following records:

```
1	mountain goat
4	billy goats
12	sheep
1,749	llamas
```

You can use `filter` to select records from this file, as follows.

To show only the lines in the file that contain goats:

```
filter -m goat farm-animals.txt
```

As above, but search case-insensitively: this will match “Goats”, “goat”,
“GOATS”, and so on:

```
filter -m '(?i)goat' farm-animals.txt
```

Equivalently:

```
filter -i -m goat farm-animals.txt
```

It’s unclear why anyone would want only *non*-goat animals, but this is how to
do that:

```
filter -p '(?i)goat' farm-animals.txt
filter -i -p goat farm-animals.txt
```

Show records that match “moss”, but exclude billy goats, regardless of how
“billy” is capitalized:

```
filter -m moss -i -p billy farm-animals.txt
```

To print a list of the animals for which the (hypothetical) program
`check-if-hungry` succeeds:

```
filter -x check-if-hungry farm-animals.txt
```

If you need to pass arguments to a `-x` command, use a quoted string:

```
filter -x 'check-if-hungry --dinner' farm-animals.txt
```

## See Also

* `futils help`
* `grep`(1)
* `find`(1)
