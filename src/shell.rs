use chrono::format;
use getopt::Opt;
use regex::bytes::Regex;
use rustc_lexer::unescape::EscapeError;
use std::fmt::{Debug, Display};
use std::{io, str};

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
/// `exit`s with the given `status`. If there is any `error`, `main` will print
/// it to `stderr`.
// TODO: This should be a struct and the error an Option.
pub type ShellResult = Result<i32, ShellError>;

pub const DEFAULT_OPTION_SPEC: &str = "D:d:hm:O:o:p:vx:";
pub const DEFAULT_INPUT_RECORD_DELIMITER: &str = r"(\r|\n)+";
pub const DEFAULT_INPUT_FIELD_DELIMITER: &str = r"\s+";
pub const DEFAULT_OUTPUT_RECORD_DELIMITER: &str = "\n";
pub const DEFAULT_OUTPUT_FIELD_DELIMITER: &str = "\t";

pub struct Options {
    pub input_record_delimiter: Option<Regex>,
    pub input_field_delimiter: Option<Regex>,
    pub output_record_delimiter: Option<String>,
    pub output_field_delimiter: Option<String>,

    pub match_expressions: Vec<Regex>,
    pub prune_expressions: Vec<Regex>,
    pub match_commands: Vec<String>,

    pub enumerate: bool,
    pub help: bool,
    pub verbose: bool,
}

impl Options {
    pub fn with_defaults() -> Result<Options, ShellError> {
        Ok(Options {
            input_record_delimiter: Some(Regex::new(DEFAULT_INPUT_RECORD_DELIMITER)?),
            input_field_delimiter: Some(Regex::new(DEFAULT_INPUT_FIELD_DELIMITER)?),
            output_record_delimiter: Some(String::from(DEFAULT_OUTPUT_RECORD_DELIMITER)),
            output_field_delimiter: Some(String::from(DEFAULT_OUTPUT_FIELD_DELIMITER)),

            match_expressions: Vec::new(),
            prune_expressions: Vec::new(),
            match_commands: Vec::new(),

            enumerate: false,
            help: false,
            verbose: false,
        })
    }
}

/// Given `options`, pre-populated with relevant defaults, parses `arguments`
/// according to `DEFAULT_OPTION_SPEC` and populates the fields of `options`.
///
/// Returns the remaining positional arguments.
pub fn parse_options(arguments: &[String]) -> Result<(Options, &[String]), ShellError> {
    let mut options = Options::with_defaults()?;
    let mut parsed = getopt::Parser::new(arguments, DEFAULT_OPTION_SPEC);
    loop {
        match parsed.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('D', Some(s)) => options.input_field_delimiter = Some(Regex::new(&s)?),
                Opt('d', Some(s)) => options.input_record_delimiter = Some(Regex::new(&s)?),
                Opt('h', None) => options.help = true,
                Opt('m', Some(s)) => options.match_expressions.push(Regex::new(&s)?),
                Opt('n', None) => options.enumerate = true,
                Opt('O', Some(s)) => options.output_field_delimiter = Some(s.clone()),
                Opt('o', Some(s)) => options.output_record_delimiter = Some(s.clone()),
                Opt('p', Some(s)) => options.prune_expressions.push(Regex::new(&s)?),
                Opt('v', None) => options.verbose = true,
                Opt('x', Some(s)) => options.match_commands.push(s.clone()),
                Opt(_o, _) => return Err(ShellError::Usage(UsageError::new("Unknown option"))),
            },
        }
    }
    let (_, arguments) = arguments.split_at(parsed.index());
    Ok((options, arguments))
}
