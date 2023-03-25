# `version` — show program version and build information

## Usage

```
futils [-J] version
```

## Description

Prints the version of `futils`, metadata, and build-time configuration
information.

## Options

* `-J`: Output JSON format.

## Additional Options

* `-F`: Set the output field delimiter.
* `-h`: Print this help page.
* `-R`: Set the output record delimiter.

## Exit Status

0 on success, or -1 if there was an error.

## Examples

```
futils version -F': '
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
