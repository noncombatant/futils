use chrono::format;
use getopt::Opt;
use regex::bytes::Regex;
use rustc_lexer::unescape::EscapeError;
use std::fmt::{Debug, Display};
use std::{io, str};

use crate::time::Time;

/// `ShellError` accounts for a variety of errors that can happen when running
/// shell commands, enabling many `main` `fn`s for shell programs to declare
/// they return it and easily use the `?` operator. We can extend this `enum`
/// arbitrarily, as needed.
#[derive(Debug)]
pub enum ShellError {
    Escape(EscapeError),
    Getopt(getopt::Error),
    Io(io::Error),
    Regex(regex::Error),
    TimeParse(format::ParseError),
    Usage(UsageError),
    Utf8(str::Utf8Error),
}

impl Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ShellError::Escape(e) => write!(f, "{:?}", e),
            ShellError::Getopt(e) => Display::fmt(e, f),
            ShellError::Io(e) => Display::fmt(e, f),
            ShellError::Regex(e) => Display::fmt(e, f),
            ShellError::TimeParse(e) => Display::fmt(e, f),
            ShellError::Usage(e) => Display::fmt(e, f),
            ShellError::Utf8(e) => Display::fmt(e, f),
        }
    }
}

impl std::error::Error for ShellError {}

/// Return this error for invalid invocations of shell commands.
#[derive(Debug)]
pub struct UsageError {
    details: String,
}

impl UsageError {
    fn new(details: &str) -> UsageError {
        UsageError {
            details: details.to_string(),
        }
    }
}

impl Display for UsageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.details)
    }
}

impl From<io::Error> for ShellError {
    fn from(e: io::Error) -> ShellError {
        ShellError::Io(e)
    }
}

impl From<EscapeError> for ShellError {
    fn from(e: EscapeError) -> ShellError {
        ShellError::Escape(e)
    }
}

impl From<getopt::Error> for ShellError {
    fn from(e: getopt::Error) -> ShellError {
        ShellError::Getopt(e)
    }
}

impl From<regex::Error> for ShellError {
    fn from(e: regex::Error) -> ShellError {
        ShellError::Regex(e)
    }
}

impl From<format::ParseError> for ShellError {
    fn from(e: format::ParseError) -> ShellError {
        ShellError::TimeParse(e)
    }
}

impl From<UsageError> for ShellError {
    fn from(e: UsageError) -> ShellError {
        ShellError::Usage(e)
    }
}

impl From<str::Utf8Error> for ShellError {
    fn from(e: str::Utf8Error) -> ShellError {
        ShellError::Utf8(e)
    }
}

/// The various `*_main` functions return this type. `main` catches it and
/// `exit`s with the given `i32` status code. If there is a `ShellError`, `main`
/// will print it to `stderr` and `exit(-1)`.
pub type ShellResult = Result<i32, ShellError>;

/// These are the standard command line options for `futils` programs. Their
/// meanings are:
///
///   -D  `Regex`   input field delimiter
///   -d  `Regex`   input record delimiter
///   -f  `String`  field
///   -h  `bool`    help
///   -m  `Regex`   match
///   -n  `bool`    enumerate
///   -O  `String`  output field delimiter
///   -o  `String`  output record delimiter
///   -p  `Regex`   prune
///   -t  `String`  file or object types
///   -v  `bool`    verbose
///   -x  `String`  command
///
/// Not all programs use all options. Some programs may not use this option
/// spec, depending on their needs.
pub const DEFAULT_OPTION_SPEC: &str = "D:d:f:hm:nO:o:p:t:vx:";

/// The default input record delimiter.
pub const DEFAULT_INPUT_RECORD_DELIMITER: &str = r"(\r|\n)+";

/// The default input field delimiter.
pub const DEFAULT_INPUT_FIELD_DELIMITER: &str = r"\s+";

/// The default output record delimiter.
pub const DEFAULT_OUTPUT_RECORD_DELIMITER: &str = "\n";

/// The default output field delimiter.
pub const DEFAULT_OUTPUT_FIELD_DELIMITER: &str = "\t";

/// The default file types.
pub const DEFAULT_FILE_TYPES: &str = "dfs";

/// Gathers all the command line options into a single handy object.
pub struct Options {
    pub input_record_delimiter: Regex,
    pub input_field_delimiter: Regex,
    pub output_record_delimiter: String,
    pub output_field_delimiter: String,

    pub match_expressions: Vec<Regex>,
    pub prune_expressions: Vec<Regex>,
    pub match_commands: Vec<String>,
    pub mtime_expressions: Vec<Time>,

    pub fields: Vec<String>,
    pub file_types: String,

    pub show_all: bool,
    pub enumerate: bool,
    pub help: bool,
    pub verbose: bool,
}

impl Options {
    /// Returns an `Options` with all the fields set to their `DEFAULT_*`
    /// values.
    pub fn with_defaults() -> Result<Options, ShellError> {
        Ok(Options {
            input_record_delimiter: Regex::new(DEFAULT_INPUT_RECORD_DELIMITER)?,
            input_field_delimiter: Regex::new(DEFAULT_INPUT_FIELD_DELIMITER)?,
            output_record_delimiter: String::from(DEFAULT_OUTPUT_RECORD_DELIMITER),
            output_field_delimiter: String::from(DEFAULT_OUTPUT_FIELD_DELIMITER),

            match_expressions: Vec::new(),
            prune_expressions: Vec::new(),
            match_commands: Vec::new(),
            mtime_expressions: Vec::new(),

            fields: Vec::new(),
            file_types: String::from(DEFAULT_FILE_TYPES),

            show_all: false,
            enumerate: false,
            help: false,
            verbose: false,
        })
    }
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
                Opt('D', Some(s)) => options.input_field_delimiter = Regex::new(&s)?,
                Opt('d', Some(s)) => options.input_record_delimiter = Regex::new(&s)?,
                Opt('f', Some(s)) => options.fields.push(s.clone()),
                Opt('h', None) => options.help = true,
                Opt('M', Some(s)) => options.mtime_expressions.push(Time::new(&s)?),
                Opt('m', Some(s)) => options.match_expressions.push(Regex::new(&s)?),
                Opt('n', None) => options.enumerate = true,
                Opt('O', Some(s)) => options.output_field_delimiter = s.clone(),
                Opt('o', Some(s)) => options.output_record_delimiter = s.clone(),
                Opt('p', Some(s)) => options.prune_expressions.push(Regex::new(&s)?),
                Opt('t', Some(s)) => options.file_types = s.clone(),
                Opt('v', None) => options.verbose = true,
                Opt('x', Some(s)) => options.match_commands.push(s.clone()),
                Opt(_o, _) => return Err(ShellError::Usage(UsageError::new("Unknown option"))),
            },
        }
    }
    let (_, arguments) = arguments.split_at(parsed.index());
    Ok((options, arguments))
}
