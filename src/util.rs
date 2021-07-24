use memmap::{Mmap, MmapOptions};
use std::fs::File;
use rustc_lexer::unescape::{EscapeError, unescape_char};

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

pub fn unescape_backslashes(s: &str) -> Result<char, (usize, EscapeError)> {
    unescape_char(s)
}
