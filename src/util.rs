use chrono::format;
use memmap::{Mmap, MmapOptions};
use rustc_lexer::unescape::{unescape_str, EscapeError};
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{self, stderr, stdout, Write};
use std::path::Path;
use std::process::{exit, Command};
use std::str;

// TODO: Move `ShellError` and `ShellResult` into shell.rs.

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

/// Prints `message` and `exit`s with `status`. If `status` is 0, prints
/// `message` to `stdout`, otherwise to `stderr`.
pub fn help(status: i32, message: &str) {
    if status == 0 {
        println!("{}", message);
    } else {
        eprintln!("{}", message);
    }
    exit(status);
}

/// Opens the file named by `pathname` and returns a memory mapping of it.
pub fn map_file(pathname: &str) -> Result<Mmap, std::io::Error> {
    let file = File::open(pathname)?;
    unsafe { MmapOptions::new().map(&file) }
}

/// Runs the shell command `command`, passing it `argument`. If `verbose` is
/// true, will print any resulting `stdout`. Prints `stderr` unconditionally.
// TODO: `argument` should probably be a `&[String]`.
pub fn run_command(command: &str, argument: &[u8], verbose: bool) -> ShellResult {
    let argument = str::from_utf8(argument)?;

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .arg(argument)
            .output()?
    } else {
        Command::new(command).arg(argument).output()?
    };

    let code = output.status.code();
    if verbose && !output.stdout.is_empty() {
        stdout().write_all(&output.stdout)?;
    }
    if !output.stderr.is_empty() {
        stderr().write_all(&output.stderr)?;
    }
    Ok(code.unwrap_or(0))
}

/// Lexes `input` according to Rust’s lexical rules for strings, unescaping any
/// backslash escape sequences. See `rustc_lexer::unescape`. Errors are of type
/// `Box<dyn Error>` for easier compatibility with `ShellResult`.
pub fn unescape_backslashes(input: &str) -> Result<String, ShellError> {
    let mut result = Ok(String::new());
    // Thanks to Steve Checkoway for help:
    let mut cb = |_, ch| match (&mut result, ch) {
        (Ok(s), Ok(ch)) => s.push(ch),
        (Ok(_), Err(e)) => result = Err(e),
        _ => (),
    };
    unescape_str(input, &mut cb);
    match result {
        Ok(s) => Ok(s),
        Err(e) => Err(ShellError::Escape(e)),
    }
}

#[test]
fn test_unescape_backslashes() {
    let r = unescape_backslashes("\\ngoat\\t").expect("Should parse");
    assert_eq!("\ngoat\t", r);
    let r = unescape_backslashes("\\ngoat\t").expect("Should parse");
    assert_eq!("\ngoat	", r);
    let r = unescape_backslashes("\ngoat\t").expect("Should parse");
    assert_eq!(
        "
goat	",
        r
    );
}

/// Returns the basename of `pathname`. (Rust calls this `file_name` instead of
/// `basename`, so we do, too.
pub fn file_name(pathname: &str) -> Option<&str> {
    Path::new(pathname).file_name()?.to_str()
}
