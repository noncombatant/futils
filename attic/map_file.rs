use memmap::{Mmap, MmapOptions};
use std::fs::File;

/// Opens the file named by `pathname` and returns a memory mapping of it.
pub fn map_file(pathname: &str) -> Result<Mmap, std::io::Error> {
    let file = File::open(pathname)?;
    unsafe { MmapOptions::new().map(&file) }
}
