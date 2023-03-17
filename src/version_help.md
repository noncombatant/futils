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

* `-h`: Print this help message.
* `-D`: Set the input field delimiter, a regular expression. The default
  delimiter is `r"\s+"`.
* `-d`: Set the input record delimiter. The default delimiter is `r"(\r|\n)+"`.
* `-O`: Set the output field delimiter. The default delimiter is `\t`.
* `-o`: Set the output record delimiter. The default delimiter is `\n`.
