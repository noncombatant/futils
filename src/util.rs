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

pub fn map_file(pathname: &str) -> Result<Mmap, std::io::Error> {
    let file = File::open(pathname)?;
    unsafe { MmapOptions::new().map(&file) }
}

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
    match e {
        EscapeError::ZeroChars => "zero chars",
        EscapeError::MoreThanOneChar => "more than one char",
        EscapeError::LoneSlash => "lone slash",
        EscapeError::InvalidEscape => "invalid escape",
        EscapeError::BareCarriageReturn => "bare carriage return",
        EscapeError::BareCarriageReturnInRawString => "bare carriage return in raw string",
        EscapeError::EscapeOnlyChar => "escape only char",
        EscapeError::TooShortHexEscape => "too short hex escape",
        EscapeError::InvalidCharInHexEscape => "invalid char in hex escape",
        EscapeError::OutOfRangeHexEscape => "out of range hex escape",
        EscapeError::NoBraceInUnicodeEscape => "no brace in Unicode escape",
        EscapeError::InvalidCharInUnicodeEscape => "invalid char in Unicode escape",
        EscapeError::EmptyUnicodeEscape => "empty Unicode escape",
        EscapeError::UnclosedUnicodeEscape => "unclosed Unicode escape",
        EscapeError::LeadingUnderscoreUnicodeEscape => "leading underscore Unicode escape",
        EscapeError::OverlongUnicodeEscape => "overlong Unicode escape",
        EscapeError::LoneSurrogateUnicodeEscape => "lone surrogate Unicode escape",
        EscapeError::OutOfRangeUnicodeEscape => "out of range Unicode escape",
        EscapeError::UnicodeEscapeInByte => "Unicode escape in byte",
        EscapeError::NonAsciiCharInByte => "non-ASCII char in byte",
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
    unescape_str(input, &mut cb);
    match result {
        Ok(s) => Ok(s),
        Err(e) => Err(Box::<dyn Error>::from(escape_error_to_str(e))),
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

pub fn file_name(pathname: &str) -> Option<&str> {
    Path::new(pathname).file_name()?.to_str()
}
