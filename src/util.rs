use memmap::{Mmap, MmapOptions};
use rustc_lexer::unescape::{unescape_str, EscapeError};
use std::error::Error;
use std::fs::File;
use std::io::{stderr, stdout, Write};
use std::path::Path;
use std::process::{exit, Command};
use std::str;

pub type ShellResult = Result<i32, Box<dyn Error>>;

pub fn help(status: i32, message: &str) {
    println!("{}", message);
    exit(status);
}

pub fn map_file(pathname: &str) -> Option<Mmap> {
    let file = File::open(pathname);
    match file {
        Ok(file) => {
            let mapped = unsafe {
                let m = MmapOptions::new().map(&file);
                match m {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("{}: {}", pathname, e);
                        return None;
                    }
                }
            };
            Some(mapped)
        }
        Err(e) => {
            eprintln!("{}: {}", pathname, e);
            None
        }
    }
}

pub fn run_command(command: &str, argument: &[u8], verbose: bool) -> ShellResult {
    let argument = str::from_utf8(argument)?;

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .arg(argument)
            .output()?
    } else {
        Command::new(command).arg(argument).output()?
    };

    let code = output.status.code();
    if verbose {
        stdout().write_all(&output.stdout)?;
    }
    if !output.stderr.is_empty() {
        stderr().write_all(&output.stderr)?;
    }
    match code {
        Some(code) => Ok(code),
        None => panic!("Total goatery in effect"),
    }
}

fn escape_error_to_str(e: EscapeError) -> &'static str {
    // TODO
    match e {
        EscapeError::ZeroChars => "",
        EscapeError::MoreThanOneChar => "",
        EscapeError::LoneSlash => "",
        EscapeError::InvalidEscape => "",
        EscapeError::BareCarriageReturn => "",
        EscapeError::BareCarriageReturnInRawString => "",
        EscapeError::EscapeOnlyChar => "",
        EscapeError::TooShortHexEscape => "",
        EscapeError::InvalidCharInHexEscape => "",
        EscapeError::OutOfRangeHexEscape => "",
        EscapeError::NoBraceInUnicodeEscape => "",
        EscapeError::InvalidCharInUnicodeEscape => "",
        EscapeError::EmptyUnicodeEscape => "",
        EscapeError::UnclosedUnicodeEscape => "",
        EscapeError::LeadingUnderscoreUnicodeEscape => "",
        EscapeError::OverlongUnicodeEscape => "",
        EscapeError::LoneSurrogateUnicodeEscape => "",
        EscapeError::OutOfRangeUnicodeEscape => "",
        EscapeError::UnicodeEscapeInByte => "",
        EscapeError::NonAsciiCharInByte => "",
        // Documented, but apparently not implemented:
        //EscapeError::UnskippedWhitespaceWarning => "",
        //EscapeError::MultipleSkippedLinesWarning => "",
        _ => "",
    }
}

pub fn unescape_backslashes(input: &str) -> Result<String, Box<dyn Error>> {
    let mut result = Ok(String::new());
    // Thanks to Steve Checkoway for help:
    let mut cb = |_, ch| match (&mut result, ch) {
        (Ok(s), Ok(ch)) => s.push(ch),
        (Ok(_), Err(e)) => result = Err(e),
        _ => (),
    };
    unescape_str(&input, &mut cb);
    match result {
        Ok(s) => Ok(s),
        Err(e) => Err(Box::<dyn Error>::from(escape_error_to_str(e))),
    }
}

pub fn file_name(pathname: &str) -> Option<&str> {
    Path::new(pathname).file_name()?.to_str()
}
