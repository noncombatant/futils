# `version` — show program version and build information

## Usage

```
futils [-j] version
```

## Description

Prints the version of `futils`, metadata, and build-time configuration
information.

## Options

* `-j`: Output JSON format.

## Additional Options

* `-h`: Print this help page.
* `-O`: Set the output field delimiter. The default delimiter is `\t`.
* `-o`: Set the output record delimiter. The default delimiter is `\n`.

## Exit Status

0 on success, or -1 if there was an error.

## Examples

```
futils version -O': '
```

Prints the version metadata keys and values separated by a colon and a space,
instead of a tab as usual.

```
futils version | filter -m major
```

Show just the major version key and its value.

## See Also

* `futils help`
* TODO
