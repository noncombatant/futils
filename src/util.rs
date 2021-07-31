use memmap::{Mmap, MmapOptions};
use std::fs::File;
// TODO: `rustc_lexer` might not be the best dependency.
use rustc_lexer::unescape::{unescape_str, EscapeError};

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
