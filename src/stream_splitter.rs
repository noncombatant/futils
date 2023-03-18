use regex::bytes::Regex;
use std::io::{Read, Result};

/// A record lexed from the input that `StreamSplitter` is splitting.
#[derive(Debug)]
pub(crate) struct Record {
    /// `StreamSplitter` yields both data records and the bytes of the delimiter
    /// that was matched when splitting. Use this field to see which you got.
    pub(crate) is_delimiter: bool,

    /// The bytes lexed from the input.
    pub(crate) bytes: Vec<u8>,
}

/// A convenience for callers who want to filter out delimiters when iterating
/// over a `StreamSplitter`:
///
///     for r in StreamSplitter::new(...).filter(is_not_delimiter) {
///         // Your beautiful, magical functionality here...
///     }
pub(crate) fn is_not_delimiter(r: &Record) -> bool {
    !r.is_delimiter
}

/// An `Iterator` that lexes a `Read`, searching for the `delimiter`, and yields
/// `Record`s containing (alternately) data bytes and delimiter bytes.
///
/// This implementation incurs a new allocation when yielding a `Record`, and
/// the caller owns it. An alternate implementation using generic associated
/// types that avoids the allocation is available in a Git branch, but it cannot
/// implement `Iterator`.
///
/// This implementation uses a private buffer that may, in pathological cases,
/// grow large (depending on how long it takes to match `delimiter`).
pub(crate) struct StreamSplitter<'a> {
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
    /// into `Record`s.
    pub(crate) fn new(reader: &'a mut dyn Read, delimiter: &'a Regex) -> Self {
        Self::with_capacity(reader, delimiter, DEFAULT_CAPACITY)
    }

    /// Creates a new `StreamSplitter` that will split the bytes of `reader`
    /// into `Record`s. The internal buffer will be pre-allocated with
    /// at least `capacity` `u8`s of storage.
    pub(crate) fn with_capacity(
        reader: &'a mut dyn Read,
        delimiter: &'a Regex,
        capacity: usize,
    ) -> Self {
        StreamSplitter {
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
    type Item = Result<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Err(e) = self.fill() {
            eprintln!("{}", e);
            return Some(Err(e));
        }

        if self.start == self.end && self.eof {
            return None;
        }

        match self.delimiter.find(&self.buffer[self.start..self.end]) {
            Some(m) => {
                let r = if m.start() == 0 {
                    if self.start + m.end() == self.end && !self.eof {
                        // `self.buffer` ends in delimiter-matching bytes, yet
                        // we are not at EOF. So we might not have matched the
                        // entirety of the delimiter. Therefore, start back at
                        // the top, which incurs a `fill`, which will grow
                        // `self.buffer`. The `unwrap` is OK because we must at
                        // least match the same match again.
                        self.next().unwrap()
                    } else {
                        // We matched the delimiter. Set us up to start at the
                        // delimiter end next time.
                        let start = self.start;
                        self.start += m.end();
                        Ok(Record {
                            is_delimiter: true,
                            bytes: self.buffer[start..self.start].to_vec(),
                        })
                    }
                } else {
                    // We matched a record. Set us up to start at the delimiter
                    // start next time.
                    let start = self.start;
                    self.start += m.start();
                    Ok(Record {
                        is_delimiter: false,
                        bytes: self.buffer[start..self.start].to_vec(),
                    })
                };
                Some(r)
            }
            None => {
                let start = self.start;
                self.start = self.end;
                Some(Ok(Record {
                    is_delimiter: false,
                    bytes: self.buffer[start..self.end].to_vec(),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::bytes::Regex;
    use std::io::{Seek, SeekFrom, Write};
    use tempfile::tempfile;

    use crate::stream_splitter::{StreamSplitter, DEFAULT_CAPACITY};

    #[test]
    fn test_simple() {
        let mut file = tempfile().unwrap();
        file.write_all(b"hello\n\nworld\n").unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();
        let delimiter = Regex::new(r"\s+").unwrap();
        let mut splitter = StreamSplitter::new(&mut file, &delimiter);

        let r = splitter.next().unwrap().unwrap();
        assert_eq!(false, r.is_delimiter);
        assert_eq!(b"hello", r.bytes.as_slice());

        let r = splitter.next().unwrap().unwrap();
        assert_eq!(true, r.is_delimiter);
        assert_eq!(b"\n\n", r.bytes.as_slice());

        let r = splitter.next().unwrap().unwrap();
        assert_eq!(false, r.is_delimiter);
        assert_eq!(b"world", r.bytes.as_slice());

        let r = splitter.next().unwrap().unwrap();
        assert_eq!(true, r.is_delimiter);
        assert_eq!(b"\n", r.bytes.as_slice());

        assert!(splitter.next().is_none());
    }

    #[test]
    fn test_delimiter_straddles_buffer() {
        let spaces = vec![b' '; DEFAULT_CAPACITY];

        let mut file = tempfile().unwrap();
        file.write_all(b"hello").unwrap();
        file.write_all(&spaces).unwrap();
        file.write_all(b"world").unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();
        let delimiter = Regex::new(r"\s+").unwrap();
        let mut splitter = StreamSplitter::new(&mut file, &delimiter);

        let r = splitter.next().unwrap().unwrap();
        assert_eq!(false, r.is_delimiter);
        assert_eq!(b"hello", r.bytes.as_slice());

        let r = splitter.next().unwrap().unwrap();
        assert_eq!(true, r.is_delimiter);
        assert_eq!(spaces, r.bytes);

        let r = splitter.next().unwrap().unwrap();
        assert_eq!(false, r.is_delimiter);
        assert_eq!(b"world", r.bytes.as_slice());

        assert!(splitter.next().is_none());
    }
}
