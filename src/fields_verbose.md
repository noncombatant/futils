## Examples

Consider a file named farm-animals.txt with the following records of tab-delimited fields showing how many of each animal we have, as well as their diet:

```
1	mountain goat	grass, moss, vegetation
4	billy goats	grass, moss, vegetation, tin cans
12	sheep	grass, more grass
1,749	llamas	exclusively human flesh (for some reason)
```

We can use `fields` to select some of the fields. (The default input field delimiter is the tab, but if we want to be explicit, we can use `-f '\t'`.) For example, to print only the count of each animal:

```
fields -f '\t' -c 0 farm-animals.txt
```

Note that field counting begins at 0, so `-c 0` gives us the *first* field.

To print every field *except* the count, we can invert the selection with `-I`:

```
fields -I -c 0 farm-animals.txt
```

If we only want to see the animals’ diets:

```
fields -c 2 farm-animals.txt
```

## See Also

* `futils help`
* `cut`(1)
* `paste`(1)
* `colrm`(1)
