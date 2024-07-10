// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! A simple framework for command line programs: error types, option parsing,
//! and assorted gadgets.

use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{self, stdin, Error, Read, Write};
use std::num::ParseIntError;
use std::str;

use bigdecimal::ParseBigDecimalError;
use chrono::format;
use derive_more::{Display, From};
use getopt::Opt;
use once_cell::sync::Lazy;
use regex::bytes::{Regex, RegexBuilder};
use rustc_lexer::unescape::EscapeError;

use crate::time::Time;
use crate::util::unescape_backslashes;

/// `ShellError` accounts for a variety of errors that can happen when running
/// shell commands, enabling `*_main` to declare they return it and easily use
/// the `?` operator. We can extend this `enum` arbitrarily, as needed.
#[derive(Debug, From)]
pub enum ShellError {
    BigDecimal(ParseBigDecimalError),
    Escape(EscapeError),
    Getopt(getopt::Error),
    IntParse(ParseIntError),
    Io(io::Error),
    Json(serde_json::Error),
    Regex(regex::Error),
    ShellWords(shell_words::ParseError),
    TimeParse(format::ParseError),
    Usage(UsageError),
    Utf8(str::Utf8Error),
}

impl Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::BigDecimal(e) => Display::fmt(e, f),
            Self::Escape(e) => write!(f, "{e:?}"),
            Self::Getopt(e) => Display::fmt(e, f),
            Self::IntParse(e) => Display::fmt(e, f),
            Self::Io(e) => Display::fmt(e, f),
            Self::Json(e) => Display::fmt(e, f),
            Self::Regex(e) => Display::fmt(e, f),
            Self::ShellWords(e) => Display::fmt(e, f),
            Self::TimeParse(e) => Display::fmt(e, f),
            Self::Usage(e) => Display::fmt(e, f),
            Self::Utf8(e) => Display::fmt(e, f),
        }
    }
}

impl std::error::Error for ShellError {}

/// Return this error for invalid invocations of shell commands.
#[derive(Display, Debug)]
pub struct UsageError {
    details: String,
}

impl UsageError {
    /// Return a new `UsageError` from `details`.
    pub fn new(details: &str) -> Self {
        Self {
            details: details.to_string(),
        }
    }
}

/// The various `*_main` functions return this type. `main` catches it and
/// `exit`s with the given `i32` status code. If there is a `ShellError`, `main`
/// will print it to `stderr` and `exit(-1)`.
pub type ShellResult = Result<i32, ShellError>;

/// The default list of command line flags. See `Options`, below.
pub const DEFAULT_OPTION_SPEC: &str = "ad:c:eF:f:hIiJjl:M:m:nP:p:R:r:st:vx:";

/// These are the standard command line options for `futils` programs.
///
/// Not all programs use all options. Some programs may not use this option
/// spec, depending on their needs.
pub struct Options {
    /// `-a`
    pub show_all: bool,

    /// `-c` (“column”, “cut”)
    pub fields: Vec<String>,

    /// `-d`
    pub depth: usize,

    /// `-e`
    pub print_empty: bool,

    /// `-F`
    pub output_field_delimiter: Vec<u8>,

    /// `-f`
    pub input_field_delimiter: Regex,

    /// `-h`
    pub help: bool,

    /// `-I`
    pub invert_fields: bool,

    /// `-i`
    pub insensitive: bool,

    /// `-J`
    pub json_output: bool,

    /// `-j`
    pub json_input: bool,

    /// `-l`
    pub limit: Option<isize>,

    /// `-M`
    pub mtime_expressions: Vec<Time>,

    /// `-m`
    pub match_expressions: Vec<Regex>,

    /// `-n`
    pub no_enumerate: bool,

    /// `-P`
    pub parallel: bool,

    /// `-p`
    pub prune_expressions: Vec<Regex>,

    /// `-R`
    pub output_record_delimiter: Vec<u8>,

    /// `-r`
    pub input_record_delimiter: Regex,

    /// `-s`
    pub skip: bool,

    /// `-t`
    pub file_types: String,

    /// `-v`
    pub verbose: bool,

    /// `-x`
    pub match_commands: Vec<String>,
}

/// The default input record delimiter. This pattern matches 1
/// DOS/Windows/Internet, POSIX, or Mac line break (in that order of
/// preference).
const DEFAULT_INPUT_RECORD_DELIMITER: &str = r"(\r\n|\n|\r)";

/// The default input field delimiter.
const DEFAULT_INPUT_FIELD_DELIMITER: &str = r"\t";

/// The default output record delimiter.
const DEFAULT_OUTPUT_RECORD_DELIMITER: &[u8] = b"\n";

/// The default output field delimiter.
const DEFAULT_OUTPUT_FIELD_DELIMITER: &[u8] = b"\t";

/// The default file types.
const DEFAULT_FILE_TYPES: &str = "dfs";

impl Options {
    /// Returns an `Options` with all the fields set to their `DEFAULT_*`
    /// values.
    pub fn with_defaults() -> Result<Self, ShellError> {
        Ok(Self {
            show_all: false,
            fields: Vec::new(),
            depth: 0,
            print_empty: false,
            output_field_delimiter: Vec::from(DEFAULT_OUTPUT_FIELD_DELIMITER),
            input_field_delimiter: Regex::new(DEFAULT_INPUT_FIELD_DELIMITER)?,
            help: false,
            invert_fields: false,
            insensitive: false,
            json_output: false,
            json_input: false,
            limit: None,
            mtime_expressions: Vec::new(),
            match_expressions: Vec::new(),
            no_enumerate: false,
            parallel: false,
            prune_expressions: Vec::new(),
            output_record_delimiter: Vec::from(DEFAULT_OUTPUT_RECORD_DELIMITER),
            input_record_delimiter: Regex::new(DEFAULT_INPUT_RECORD_DELIMITER)?,
            skip: false,
            file_types: String::from(DEFAULT_FILE_TYPES),
            verbose: false,
            match_commands: Vec::new(),
        })
    }
}

fn new_regex(pattern: &str, options: &Options) -> Result<Regex, regex::Error> {
    RegexBuilder::new(pattern)
        .case_insensitive(options.insensitive)
        .build()
}

/// Parses `arguments` according to `DEFAULT_OPTION_SPEC`. Returns the parsed
/// `Options` and the remaining positional arguments. Any options not given on
/// the command line will have their `DEFAULT_*` values in the returned
/// `Options` (see `Options::with_defaults`).
pub fn parse_options(arguments: &[String]) -> Result<(Options, &[String]), ShellError> {
    let mut options = Options::with_defaults()?;
    let mut parsed = getopt::Parser::new(arguments, DEFAULT_OPTION_SPEC);

    loop {
        match parsed.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('a', None) => options.show_all = true,
                Opt('c', Some(s)) => options.fields.push(s.clone()),
                Opt('d', Some(s)) => options.depth = str::parse::<usize>(&s)?,
                Opt('e', None) => options.print_empty = true,
                Opt('F', Some(s)) => {
                    options.output_field_delimiter =
                        Vec::from(unescape_backslashes(&s)?.as_bytes());
                }
                Opt('f', Some(s)) => options.input_field_delimiter = new_regex(&s, &options)?,
                Opt('I', None) => options.invert_fields = true,
                Opt('i', None) => options.insensitive = true,
                Opt('h', None) => options.help = true,
                Opt('J', None) => options.json_output = true,
                Opt('j', None) => options.json_input = true,
                Opt('l', Some(s)) => options.limit = Some(str::parse::<isize>(&s)?),
                Opt('M', Some(s)) => options.mtime_expressions.push(Time::new(&s)?),
                Opt('m', Some(s)) => options.match_expressions.push(new_regex(&s, &options)?),
                Opt('n', None) => options.no_enumerate = true,
                Opt('P', None) => options.parallel = true,
                Opt('p', Some(s)) => options.prune_expressions.push(new_regex(&s, &options)?),
                Opt('R', Some(s)) => {
                    options.output_record_delimiter =
                        Vec::from(unescape_backslashes(&s)?.as_bytes());
                }
                Opt('r', Some(s)) => options.input_record_delimiter = new_regex(&s, &options)?,
                Opt('s', None) => options.skip = true,
                Opt('t', Some(s)) => options.file_types.clone_from(&s),
                Opt('v', None) => options.verbose = true,
                Opt('x', Some(s)) => options.match_commands.push(s.clone()),
                Opt(_o, _) => return Err(ShellError::Usage(UsageError::new("Unknown option"))),
            },
        }
    }
    let (_, arguments) = arguments.split_at(parsed.index());
    Ok((options, arguments))
}

pub static STDIN_PATHNAME: Lazy<String> = Lazy::new(|| "<stdin>".to_string());

/// An open `Read`.
pub struct OpenFile<'a> {
    /// The pathname by which the file was opened. If `None`, the file was
    /// already open (e.g. `stdin()`; see ).
    pub pathname: Option<&'a String>,
    /// The `Read`.
    pub read: Result<Box<dyn Read>, io::Error>,
}

/// An `Iterator` that iterates over a slice of pathnames, and yields
/// `OpenFile`s.
pub struct FileOpener<'a> {
    pathnames: &'a [String],
    i: usize,
}

impl<'a> FileOpener<'a> {
    pub const fn new(pathnames: &'a [String]) -> Self {
        FileOpener { pathnames, i: 0 }
    }
}

impl<'a> Iterator for FileOpener<'a> {
    type Item = OpenFile<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pathnames.is_empty() && self.i == 0 {
            self.i += 1;
            Some(OpenFile {
                pathname: None,
                read: Ok(Box::new(stdin()) as Box<dyn Read>),
            })
        } else if self.i < self.pathnames.len() {
            let pathname = &self.pathnames[self.i];
            let r = match File::open(pathname) {
                Ok(f) => Ok(Box::new(f) as Box<dyn Read>),
                Err(e) => Err(e),
            };
            self.i += 1;
            Some(OpenFile {
                pathname: Some(pathname),
                read: r,
            })
        } else {
            None
        }
    }
}

// Ultimately, this should go away and we should use a custom
// `serde::ser::Serialize` for columnar output.
pub trait StructuredWrite {
    fn write(&self, output: &mut dyn Write, options: &Options) -> Result<(), Error>;
}
