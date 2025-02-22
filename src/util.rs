// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

use crate::shell::ShellResult;
use atty::Stream;
use bigdecimal::BigDecimal;
use bstr::ByteSlice;
use locale::Numeric;
use rustc_lexer::unescape::{EscapeError, unescape_str};
use serde::Serializer;
use std::{
    cmp::{Ordering, min},
    env,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    io::{Write, stderr, stdout},
    iter::zip,
    path::Path,
    process::{Command, exit},
    str::{self, FromStr, from_utf8},
};
use termimad::{Alignment, FmtText, MadSkin, terminal_size};

fn text_width() -> usize {
    let (terminal_width, _) = terminal_size();
    let terminal_width = terminal_width as usize;
    let man_width = env::var("MANWIDTH").map_or(String::new(), |v| v);
    let man_width = man_width.parse::<usize>().map_or(80, |w| w);
    min(terminal_width, man_width)
}

fn terminal_text<'a>(s: &'a str, skin: &'a MadSkin) -> FmtText<'a, 'a> {
    skin.text(s, Some(text_width()))
}

pub fn get_skin(stream: Stream) -> MadSkin {
    let color = env::var("MANCOLOR").is_ok();
    let mut skin = if color || atty::is(stream) {
        MadSkin::default()
    } else {
        MadSkin::no_style()
    };
    skin.headers[0].align = Alignment::Left;
    skin
}

/// Prints `message`, the contents of `common_options.md` if `common` is true,
/// and `verbose` if it is present. Prints to the standard output if `status` is
/// 0; otherwise prints to the standard error. `exit`s with `status`.
pub fn help(status: i32, message: &str, common: bool, verbose: Option<&str>) -> ShellResult {
    let mut output: &mut dyn Write = match status {
        0 => &mut stdout(),
        _ => &mut stderr(),
    };
    let skin = get_skin(match status {
        0 => Stream::Stdout,
        _ => Stream::Stderr,
    });
    writeln!(&mut output, "{}", terminal_text(message, &skin))?;
    if common {
        write!(
            &mut output,
            "{}",
            terminal_text(include_str!("common_options.md"), &skin)
        )?;
    }
    if let Some(v) = verbose {
        write!(&mut output, "{}", terminal_text(v, &skin))?;
    }
    Ok(status)
}

/// Runs the shell command `command`, passing it `argument`. If `verbose` is
/// true, will print any resulting `stdout`. Prints `stderr` unconditionally.
// TODO: `arguments` should be `&[OsString]`.
pub fn run_command(command: &str, arguments: &[&[u8]], verbose: bool) -> ShellResult {
    let words = shell_words::split(command)?;
    let arguments = arguments
        .iter()
        .map(|a| str::from_utf8(a))
        .collect::<Result<Vec<&str>, str::Utf8Error>>()?;

    let mut command = Command::new(&words[0]);
    if words.len() > 1 {
        command.args(&words[1..]);
    }
    command.args(arguments);
    let output = command.output()?;

    if verbose && !output.stdout.is_empty() {
        stdout().write_all(&output.stdout)?;
    }
    if !output.stderr.is_empty() {
        stderr().write_all(&output.stderr)?;
    }
    Ok(output.status.code().unwrap_or(0))
}

// `EscapeError` does not actually implement `Error`, so we have to do it
// ourselves.
pub struct MyEscapeError {
    #[allow(unused)]
    error: EscapeError,
}

impl Error for MyEscapeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &'static str {
        stringify!(self.error)
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl Display for MyEscapeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", stringify!(self.error))
    }
}

impl Debug for MyEscapeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", stringify!(self.error))
    }
}

/// Lexes `input` according to Rust’s lexical rules for strings, unescaping any
/// backslash escape sequences. See `rustc_lexer::unescape`.
pub fn unescape_backslashes(input: &str) -> Result<String, MyEscapeError> {
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
        Err(e) => Err(MyEscapeError { error: e }),
    }
}

/// Returns the basename of `pathname`. (Rust calls this `file_name` instead of
/// `basename`, so we do, too.)
pub fn file_name(pathname: &str) -> Option<&str> {
    Path::new(pathname).file_name()?.to_str()
}

/// Parses `value` and returns a `BigDecimal`.
pub fn parse_number(value: &[u8]) -> Result<BigDecimal, Box<dyn Error>> {
    let separator = match Numeric::load_user_locale() {
        Ok(numeric) => numeric.thousands_sep,
        Err(_) => String::new(),
    };
    let value = from_utf8(value)?;
    let value = value.replace(&separator, "");
    Ok(BigDecimal::from_str(&value)?)
}

/// Compares case-insensitively, without allocating.
pub fn icmp(a: &[u8], b: &[u8]) -> Ordering {
    for (a, b) in zip(a.chars(), b.chars()) {
        let o = a.to_lowercase().cmp(b.to_lowercase());
        if o != Ordering::Equal {
            return o;
        }
    }
    a.len().cmp(&b.len())
}

/// Serializes `string` as a UTF-8 string if possible, or as an array of bytes
/// otherwise.
pub fn serialize_str_or_bytes<S>(string: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match string.to_str() {
        Ok(s) => serializer.serialize_str(s),
        Err(_) => serializer.serialize_bytes(string.as_bytes()),
    }
}

pub fn exit_with_result(result: ShellResult) {
    match result {
        Ok(status) => exit(status),
        Err(error) => {
            eprint!("{error}");
            exit(-1)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::{file_name, icmp, unescape_backslashes};
    use std::cmp::Ordering;

    #[test]
    fn test_unescape_backslashes() {
        let r = unescape_backslashes("\\0").unwrap();
        assert_eq!("\0", r);
        let r = unescape_backslashes("\\ngoat\\t").unwrap();
        assert_eq!("\ngoat\t", r);
        let r = unescape_backslashes("\\ngoat\t").unwrap();
        assert_eq!("\ngoat	", r);
        let r = unescape_backslashes("\ngoat\t").unwrap();
        assert_eq!(
            "
goat	",
            r
        );
    }

    #[test]
    fn test_file_name() {
        assert_eq!("test", file_name("test").unwrap());
        assert_eq!("test", file_name("./test").unwrap());
        assert_eq!("test", file_name("/leg/foot/../test").unwrap());
        assert_eq!("test", file_name("twerb/twib/test noodle/test").unwrap());
        assert_eq!(
            "test.exe",
            file_name("twerb/twib/test noodle/test.exe").unwrap()
        );
    }

    #[test]
    fn test_icmp_basic() {
        assert_eq!(Ordering::Equal, icmp(b"goat", b"goat"));
        assert_eq!(Ordering::Equal, icmp(b"Goat", b"goaT"));
        assert_eq!(Ordering::Less, icmp(b"boat", b"goat"));
        assert_eq!(Ordering::Less, icmp(b"goat", b"goats"));
        assert_eq!(Ordering::Less, icmp(b"Boat", b"goat"));
        assert_eq!(Ordering::Greater, icmp(b"goatee", b"goat"));
        assert_eq!(Ordering::Greater, icmp(b"goat", b"boat"));
        assert_eq!(Ordering::Greater, icmp(b"goat", b"BOAT"));
    }
}
