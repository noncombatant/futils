use rustc_lexer::unescape::unescape_str;
use std::io::{stderr, stdout, Write};
use std::path::Path;
use std::process::{exit, Command};
use std::str;

use crate::shell::{ShellError, ShellResult};

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

/// Runs the shell command `command`, passing it `argument`. If `verbose` is
/// true, will print any resulting `stdout`. Prints `stderr` unconditionally.
// TODO: `arguments` should be `&[String]`.
pub fn run_command(command: &str, argument: &[u8], verbose: bool) -> ShellResult {
    let argument = str::from_utf8(argument)?;
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .arg(argument)
            .output()?
    } else {
        let command = [command, argument].join(" ");
        Command::new("sh").arg("-c").arg(command).output()?
    };

    if verbose && !output.stdout.is_empty() {
        stdout().write_all(&output.stdout)?;
    }
    if !output.stderr.is_empty() {
        stderr().write_all(&output.stderr)?;
    }
    Ok(output.status.code().unwrap_or(0))
}

/// Lexes `input` according to Rust’s lexical rules for strings, unescaping any
/// backslash escape sequences. See `rustc_lexer::unescape`. Returns
/// `ShellError` for easier compatibility with `ShellResult`.
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

/// Returns the basename of `pathname`. (Rust calls this `file_name` instead of
/// `basename`, so we do, too.)
pub fn file_name(pathname: &str) -> Option<&str> {
    Path::new(pathname).file_name()?.to_str()
}

#[cfg(test)]
mod tests {
    use crate::util::{file_name, unescape_backslashes};

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
}
