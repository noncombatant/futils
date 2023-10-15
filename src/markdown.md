# `markdown` — display Markdown in the terminal

Prints the given Markdown files (or `stdin`) in the terminal using terminal escape codes for formatting.

## Usage

```
markdown [pathname [...]]
markdown -h
```

## Environment Variables

* `MANWIDTH`: `markdown` will limit text output to the number of columns given in this variable’s value. If not present, the text width will be the width of the terminal.
* `MANCOLOR`: `markdown` will render Markdown with terminal escape codes if this value is set or if `stdout` is a terminal. Otherwise, they will render Markdown as plain text.
