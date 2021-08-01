use memmap::{Mmap, MmapOptions};
// TODO: `rustc_lexer` might not be the best dependency.
use rustc_lexer::unescape::{unescape_str, EscapeError};
use std::fs::File;
use std::io::{stderr, stdout, Write};
use std::process::{exit, Command};
use std::str;

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

pub fn run_command(command: &str, argument: &[u8]) -> bool {
    let argument = str::from_utf8(argument).unwrap();
    let error_message = "failed to execute process";

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .arg(argument)
            .output()
            .expect(error_message)
    } else {
        Command::new(command)
            .arg(argument)
            .output()
            .expect(error_message)
    };

    let success = output.status.success();
    // TODO: Add a `verbose` option to control whether to write `output.stdout`.
    stdout().write_all(&output.stdout).unwrap();
    stderr().write_all(&output.stderr).unwrap();
    if !success {
        match output.status.code() {
            Some(code) => exit(code),
            None => exit(1),
        }
    }
    success
}

pub fn unescape_backslashes(input: &str) -> Result<String, EscapeError> {
    let mut result = Ok(String::new());
    // Thanks to Steve Checkoway for help:
    let mut cb = |_, ch| match (&mut result, ch) {
        (Ok(s), Ok(ch)) => s.push(ch),
        (Ok(_), Err(e)) => result = Err(e),
        _ => (),
    };
    unescape_str(&input, &mut cb);
    result
}
