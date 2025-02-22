// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! A simple framework for command line programs: error types, option parsing,
//! and assorted gadgets.

use crate::{time::Time, util::unescape_backslashes};
use anyhow::Result;
use getopt::Opt;
use once_cell::sync::Lazy;
use regex::bytes::{Regex, RegexBuilder};
use std::{
    fmt::{Debug, Display, Formatter},
    fs::File,
    io::{self, Error, Read, Write, stdin},
    str,
};

/// Return this error for invalid invocations of shell commands.
#[derive(Debug)]
pub struct UsageError {
    description: String,
}

impl UsageError {
    /// Return a new `UsageError` from `description`.
    pub fn new(description: &str) -> Self {
        Self {
            description: description.to_string(),
        }
    }
}

impl Display for UsageError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl std::error::Error for UsageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

/// The various `*_main` functions return this type. `main` catches it and
/// `exit`s with the given `i32` status code. If there is an error, `main` will
/// print it to `stderr` and `exit(-1)`.
pub type ShellResult = anyhow::Result<i32>;

/// A synonym for convenience.
pub type EmptyResult = anyhow::Result<()>;

/// The default list of command line flags. See `Options`, below.
pub const DEFAULT_OPTION_SPEC: &str = "ad:c:eF:f:hIJjl:M:m:nP:p:R:r:Sst:vx:";

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

    /// `-S`
    pub case_sensitive: bool,

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
    pub fn with_defaults() -> Result<Self> {
        Ok(Self {
            show_all: false,
            fields: Vec::new(),
            depth: 0,
            print_empty: false,
            output_field_delimiter: Vec::from(DEFAULT_OUTPUT_FIELD_DELIMITER),
            input_field_delimiter: Regex::new(DEFAULT_INPUT_FIELD_DELIMITER)?,
            help: false,
            invert_fields: false,
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
            case_sensitive: false,
            skip: false,
            file_types: String::from(DEFAULT_FILE_TYPES),
            verbose: false,
            match_commands: Vec::new(),
        })
    }
}

fn new_regex(pattern: &str, options: &Options) -> Result<Regex, regex::Error> {
    RegexBuilder::new(pattern)
        .case_insensitive(!(options.case_sensitive || pattern.chars().any(char::is_uppercase)))
        .build()
}

/// Parses `arguments` according to `DEFAULT_OPTION_SPEC`. Returns the parsed
/// `Options` and the remaining positional arguments. Any options not given on
/// the command line will have their `DEFAULT_*` values in the returned
/// `Options` (see `Options::with_defaults`).
pub fn parse_options(arguments: &[String]) -> Result<(Options, &[String])> {
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
                Opt('S', None) => options.case_sensitive = true,
                Opt('s', None) => options.skip = true,
                Opt('t', Some(s)) => options.file_types.clone_from(&s),
                Opt('v', None) => options.verbose = true,
                Opt('x', Some(s)) => options.match_commands.push(s.clone()),
                Opt(_o, _) => return Err(UsageError::new("Unknown option").into()),
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
