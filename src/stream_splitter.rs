// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! An `Iterator` that yields `Vec<u8>`s from streams, delimited by regular
//! expressions.

use std::io::{Read, Result};

use regex::bytes::Regex;

/// An `Iterator` that lexes a `Read`, searching for the `delimiter`, and yields
/// non-delimiter bytes.
///
/// This implementation incurs a new allocation when yielding, and the caller
/// owns it. An alternate implementation using generic associated types that
/// avoids the allocation is available in a Git branch, but it cannot implement
/// `Iterator`.
///
/// This implementation uses a private buffer that may, in pathological cases,
/// grow large (depending on how long it takes to match `delimiter`).
pub struct StreamSplitter<'a> {
    reader: &'a mut dyn Read,
    delimiter: &'a Regex,
    buffer: Vec<u8>,
    // `buffer[start..end]` is the current slice in which we search for
    // `delimiter`.
    start: usize,
    end: usize,
    eof: bool,
}

/// This value comes from Pumpkin Town. (For those who have never visited:
/// Pumpkin Town is a special place where everyone you meet makes semi-educated
/// guesses about quantities, but has not actually done any measurement.)
const DEFAULT_CAPACITY: usize = 64 * 1024;

impl<'a> StreamSplitter<'a> {
    /// Creates a new `StreamSplitter` that will split the bytes of `reader`
    /// into `Vec<u8>`s.
    pub fn new(reader: &'a mut dyn Read, delimiter: &'a Regex) -> Self {
        Self::with_capacity(reader, delimiter, DEFAULT_CAPACITY)
    }

    /// Creates a new `StreamSplitter` that will split the bytes of `reader`
    /// into `Vec<u8>`s. The internal buffer will be pre-allocated with at least
    /// `capacity` `u8`s of storage.
    pub fn with_capacity(reader: &'a mut dyn Read, delimiter: &'a Regex, capacity: usize) -> Self {
        Self {
            reader,
            delimiter,
            buffer: vec![0; capacity],
            start: 0,
            end: 0,
            eof: false,
        }
    }

    /// Fills the `StreamSplitter`’s buffer, growing it if it is already full.
    fn fill(&mut self) -> Result<()> {
        if self.end == self.buffer.capacity() {
            if self.start == self.end {
                // We have consumed the buffer. Reset it:
                self.start = 0;
                self.end = 0;
            } else {
                // The buffer is full. To read more, we must grow it:
                self.buffer.resize(2 * self.buffer.capacity(), 0);
            }
        }
        let cap = self.buffer.capacity();
        let n = self.reader.read(&mut self.buffer[self.end..cap])?;
        self.end += n;
        if n == 0 {
            self.eof = true;
        }
        Ok(())
    }
}

impl<'a> Iterator for StreamSplitter<'a> {
    type Item = Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Err(error) = self.fill() {
            eprintln!("{error}"); // TODO: the caller should do this, not us
            return Some(Err(error));
        }

        if self.start == self.end && self.eof {
            return None;
        }

        let section = &self.buffer[self.start..self.end];
        match self.delimiter.find(section) {
            Some(m) => {
                if self.start + m.end() == self.end && !self.eof {
                    // `self.buffer` ends in delimiter-matching bytes, yet we
                    // are not at EOF. So we might not have matched the
                    // entirety of the delimiter. Therefore, start back at the
                    // top, which incurs a `fill`, which will grow
                    // `self.buffer`. The `unwrap` is OK because we must at
                    // least match the same match again.
                    return Some(self.next().unwrap());
                }
                self.start += m.end();
                let r = if m.start() == 0 {
                    // We matched the delimiter at the beginning of the section.
                    Ok(Vec::new())
                } else {
                    // We matched a record.
                    Ok(section[0..m.start()].to_vec())
                };
                Some(r)
            }
            None => {
                // Last record, with no trailing delimiter.
                self.start = self.end;
                Some(Ok(section.to_vec()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::bytes::Regex;
    use std::io::{Seek, SeekFrom, Write};
    use tempfile::tempfile;

    use crate::stream_splitter::StreamSplitter;

    // Makes debugging easier than `DEFAULT_CAPACITY`, which fills the terminal
    // with junk.
    const SMALL_CAPACITY: usize = 16;

    #[test]
    fn test_simple() {
        let mut file = tempfile().unwrap();
        file.write_all(b"hello\n\nworld\n").unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();
        let delimiter = Regex::new(r"\s+").unwrap();
        let mut splitter = StreamSplitter::with_capacity(&mut file, &delimiter, SMALL_CAPACITY);

        let r = splitter.next().unwrap().unwrap();
        assert_eq!(b"hello", r.as_slice());

        let r = splitter.next().unwrap().unwrap();
        assert_eq!(b"world", r.as_slice());

        assert!(splitter.next().is_none());
    }

    #[test]
    fn test_delimiter_straddles_buffer() {
        let spaces = vec![b' '; SMALL_CAPACITY];

        let mut file = tempfile().unwrap();
        file.write_all(b"greetings").unwrap();
        file.write_all(&spaces).unwrap();
        file.write_all(b"world").unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();
        let delimiter = Regex::new(r"\s+").unwrap();
        let mut splitter = StreamSplitter::with_capacity(&mut file, &delimiter, SMALL_CAPACITY);

        let r = splitter.next().unwrap().unwrap();
        assert_eq!(b"greetings", r.as_slice());

        let r = splitter.next().unwrap().unwrap();
        assert_eq!(b"world", r.as_slice());

        assert!(splitter.next().is_none());
    }
}
