# Futils

A suite of functional(-like) command shell utilities.

The attempt to make modern, rational shell utilities is admittedly somewhat
futile. French speakers may pronounce it « foutils » if they like.

## Goals

* Structured data; JSON everywhere
* Modern: Unicode everywhere, complete Markdown documentation, complete tests
* Consistently flexible: regular expressions everywhere applicable
* Consistent command line structure and behavior
* At least as time- and space- efficient as classic equivalents
* Safe: Memory-safe, typeful, mistake-resistant UX, as side-effect safe as
  possible

## Consistent UX Conventions

### Help And Documentation

`-h` and a `help` sub-command are always available and always print a help
message to `stdout`. Unlike error conditions (e.g. invalid options), an explicit
request for help yields exit status 0.

All programs print their help in valid Markdown format. `foo -h | md | less -F`,
for example, should always produce readable results.

### Exit Status

Programs that succeed exit with status 0. Non-zero statuses indicate program
failure.

+-------------+--------------------+
| Exit Status | Meaning            |
+-------------+--------------------+
|           0 | Success            |
+          -1 | Generic failure    +
+         > 1 | Number of errors   |
+-------------+--------------------+

When a program fails, it always prints a meaningful error message to `stderr`.
If a program prints its help message as a result of failure, it prints to
`stderr`.

### File Arguments And `stdin`

The normal mode of `futils` programs is stream processing. They always stream
`stdin` when given no arguments. Arguments, when present, are the pathnames of
files that the program will stream (or directories that the program will crawl,
as appropriate).

### Regular Expressions

Wherever possible and appropriate, `futils` programs use Rust regular
expressions for searching and lexical analysis.

## Development

Every `pub` identifier has a Rustdoc comment. Rustdoc is always well-formed
Markdown.

Every function has thorough unit tests.

Every command line program has thorough functional tests.

Never `panic!`.

# TODO: Fold the examples below into the help strings of each program

<a name="apply"></a>
<h2><code>apply</code></h2>

<p>Given 1 or more input files, splits the files into records (as delimited by a
given regular expression) and passes each record to a given command as its
argument. If no files are given, reads from the standard input.</p>

<h3>Synopsis</h3>

<pre>
apply -h
apply [-d <var>expression</var>] [-o <var>string</var>] -x <var>command</var> [<var>file</var> [...]]
</pre>

<h3>Options</h3>

<dl>

<dt><code>-h</code></dt>

<dd>Print help message and exit.</dd>

<dt><code>-d <var>expression</var></code></dt>

<dd>Use the regular expression <var>expression</var> to <b>d</b>elimit records
in the input. If no <var>expression</var> is given, the default value is
<code>(\r\n|\n|\r)</code>.</dd>

<dt><code>-o <var>string</var></code></dt>

<dd>Use the string <var>string</var> to delimit records in the <b>o</b>utput. If
no <var>string</var> is given, the default value is the platform’s native new
line sequence (e.g. <code>\n</code> on POSIX, <code>\r\n</code> on
Windows).</dd>

<dt><code>-x <var>command</var></code></dt>

<dd>For each record in the input, e<b>x</b>ecute the given <var>command</var>
with the record as the argument.</dd>

</dl>

<h3>Examples</h3>

<p>TODO</p>


<a name="filter"></a>
<h2><code>filter</code></h2>

<p>Given 1 or more input files, splits the files into records (as delimited by a
given regular expression) and prints each record (delimited by a given string)
that satisfies a given condition. If no files are given, reads from the standard
input.</p>

<h3>Synopsis</h3>

<pre>
filter -h
filter [-d <var>expression</var>] [-o <var>string</var>] -m <var>expression</var> [<var>file</var> [...]]
filter [-d <var>expression</var>] [-o <var>string</var>] -p <var>expression</var> [<var>file</var> [...]]
filter [-d <var>expression</var>] [-o <var>string</var>] -x <var>command</var> [<var>file</var> [...]]
</pre>

<p>Note that you must pass exactly 1 of <code>-m</code>, <code>-p</code>, or
<code>-x</code>.</p>

<h3>Options</h3>

<dl>

<dt><code>-h</code></dt>

<dd>Print help message and exit.</dd>

<dt><code>-d <var>expression</var></code></dt>

<dd>Use the regular expression <var>expression</var> to <b>d</b>elimit records
in the input. If no <var>expression</var> is given, the default value is
<code>(\r\n|\n|\r)</code>.</dd>

<dt><code>-m <var>expression</var></code></dt>

<dd>Print records that <b>m</b>atch the given regular expression
<var>expression</var>.</dd>

<dt><code>-o <var>string</var></code></dt>

<dd>Use the string <var>string</var> to delimit records in the <b>o</b>utput. If
no <var>string</var> is given, the default value is the platform’s native new
line sequence (e.g. <code>\n</code> on POSIX, <code>\r\n</code> on
Windows).</dd>

<dt><code>-p <var>expression</var></code></dt>

<dd>Print records that do not match the given regular expression
<var>expression</var> (i.e. <b>p</b>rune them).</dd>

<dt><code>-x <var>command</var></code></dt>

<dd>For each record in the input, e<b>x</b>ecute the given <var>command</var>.
If <var>command</var> exits normally (e.g. with status 0 on POSIX), print the
record.</dd>

</dl>

<h3>Examples</h3>

<p>TODO</p>


<a name="records"></a>
<h2><code>records</code></h2>

<p>Given 1 or more input files, splits the files into records (as delimited by a
given regular expression) and prints each record (delimited by a given string).
If no files are given, reads from the standard input.</p>

<h3>Synopsis</h3>

<pre>
records -h
records [-d <var>expression</var>] [-o <var>string</var>] [<var>file</var> [...]]
</pre>

<h3>Options</h3>

<dl>

<dt><code>-h</code></dt>

<dd>Print help message and exit.</dd>

<dt><code>-d <var>expression</var></code></dt>

<dd>Use the regular expression <var>expression</var> to <b>d</b>elimit records
in the input. If no <var>expression</var> is given, the default value is
<code>(\r\n|\n|\r)</code>.</dd>

<dt><code>-o <var>string</var></code></dt>

<dd>Use the string <var>string</var> to delimit records in the <b>o</b>utput. If
no <var>string</var> is given, the default value is the platform’s native new
line sequence (e.g. <code>\n</code> on POSIX, <code>\r\n</code> on
Windows).</dd>

<h3>Examples</h3>

<pre>
% <b>records some-file.txt</b>
</pre>

<p>This is similar to POSIX <code>cat</code>, except that it will convert any
new line sequences into your platform’s native sequence.</p>

<pre>
% <b>records -d '\r\n' -o '\n' some-file.txt</b>
</pre>

<p>As above, but explicitly convert Windows new line sequences (only) into
POSIX.</p>

<pre>
% <b>records -o '\0' some-file.txt</b>
</pre>

<p>Delimit records in some-file.txt with the NUL character (<code>\0</code>).
This is typically used together with other utilities that use NUL to delimit
records in a more robust way (such as when the other utilities may treat the
file’s existing delimiters as as syntactic metacharacters of some kind). For
example,</p>

<pre>
% <b>records -o '\0' list-of-files.txt | xargs -0 foo...</b>
</pre>

<p>(See for example <a href="#filter"><code>filter</code></a>, and the
<code>xargs</code>(1) and <code>find</code>(1) manual pages.)</p>
