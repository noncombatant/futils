use chrono::format;
use rustc_lexer::unescape::EscapeError;
use std::fmt::{Debug, Display};
use std::io;
use std::str;

#[derive(Debug)]
pub enum ShellError {
    Escape(EscapeError),
    Getopt(getopt::Error),
    Io(io::Error),
    Regex(regex::Error),
    TimeParse(format::ParseError),
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
            ShellError::Utf8(e) => Display::fmt(e, f),
        }
    }
}

impl std::error::Error for ShellError {}

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
