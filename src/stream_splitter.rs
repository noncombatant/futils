use regex::bytes::Regex;
use std::io::{Read, Result};

const DEFAULT_CAPACITY: usize = 64 * 1024;

// TODO: With some lifetime magic, we might be able to make `bytes` a `&'a [u8]`
// and avoid the copy.
#[derive(Debug)]
pub struct Record {
    pub is_delimiter: bool,
    pub bytes: Vec<u8>,
}

pub fn is_not_delimiter(r: &Record) -> bool {
    !r.is_delimiter
}

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

impl<'a> StreamSplitter<'a> {
    pub fn new(reader: &'a mut dyn Read, delimiter: &'a Regex) -> StreamSplitter<'a> {
        StreamSplitter {
            reader,
            delimiter,
            buffer: vec![0; DEFAULT_CAPACITY],
            start: 0,
            end: 0,
            eof: false,
        }
    }
}

fn fill(s: &mut StreamSplitter) -> Result<()> {
    if s.end == s.buffer.capacity() {
        if s.start == s.end {
            // We have consumed the buffer. Reset it:
            s.start = 0;
            s.end = 0;
        } else {
            // The buffer is full. To read more, we must grow it:
            s.buffer.resize(2 * s.buffer.capacity(), 0);
        }
    }
    let cap = s.buffer.capacity();
    let n = s.reader.read(&mut s.buffer[s.end..cap])?;
    s.end += n;
    if n == 0 {
        s.eof = true;
    }
    Ok(())
}

impl<'a> Iterator for StreamSplitter<'a> {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        if let Err(e) = fill(self) {
            // TODO: This is not great.
            eprintln!("{}", e);
            return None;
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
                        Record {
                            is_delimiter: true,
                            bytes: self.buffer[start..self.start].to_vec(),
                        }
                    }
                } else {
                    // We matched a record. Set us up to start at the delimiter
                    // start next time.
                    let start = self.start;
                    self.start += m.start();
                    Record {
                        is_delimiter: false,
                        bytes: self.buffer[start..self.start].to_vec(),
                    }
                };
                Some(r)
            }
            None => {
                let start = self.start;
                self.start = self.end;
                Some(Record {
                    is_delimiter: false,
                    bytes: self.buffer[start..self.end].to_vec(),
                })
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

        let r = splitter.next().unwrap();
        assert_eq!(false, r.is_delimiter);
        assert_eq!(b"hello", r.bytes.as_slice());

        let r = splitter.next().unwrap();
        assert_eq!(true, r.is_delimiter);
        assert_eq!(b"\n\n", r.bytes.as_slice());

        let r = splitter.next().unwrap();
        assert_eq!(false, r.is_delimiter);
        assert_eq!(b"world", r.bytes.as_slice());

        let r = splitter.next().unwrap();
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

        let r = splitter.next().unwrap();
        assert_eq!(false, r.is_delimiter);
        assert_eq!(b"hello", r.bytes.as_slice());

        let r = splitter.next().unwrap();
        assert_eq!(true, r.is_delimiter);
        assert_eq!(spaces, r.bytes);

        let r = splitter.next().unwrap();
        assert_eq!(false, r.is_delimiter);
        assert_eq!(b"world", r.bytes.as_slice());

        assert!(splitter.next().is_none());
    }
}
