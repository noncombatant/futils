# `version` — show program version and build information

## Usage

```
futils [-j] version
```

## Description

Prints the version of `futils`, metadata, and build-time configuration
information.

## Additional Options

* `-h`: Print this help message.
* `-D`: Use the given input field `delimiter`, a regular expression. The
  default delimiter is `r"\s+"`.
* `-d`: Use the given input record delimiter. The default delimiter is
  `r"(\r|\n)+"`.
* `-O`: Use the given output field `delimiter`. The default delimiter is `\t`.
* `-o`: Use the given output record delimiter. The default delimiter is `\n`.
