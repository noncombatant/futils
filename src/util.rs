use memmap::{Mmap, MmapOptions};
use std::fs::File;
// TODO: `rustc_lexer` might not be the best dependency.
use rustc_lexer::unescape::{EscapeError, unescape_str};

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
    let mut result = String::new();
    let mut cb = |_, ch| {
        if let Ok(c) = ch {
            result.push(c);
        } else {
            // TODO: Don't just fail silently.
        }
    };
    unescape_str(&input, &mut cb);
    Ok(result)
}
