//! An `Iterator` that yields `Records` from streams, delimited by regular
//! expressions.

use std::io::{Read, Result, Write};
//use std::str::from_utf8;

use regex::bytes::Regex;
use serde::Serialize;
//use serde::ser::Serializer;
//use serde_with::{BytesOrString, serde_as};

//use serde::ser::{Serialize, Serializer, SerializeStruct};

/// A record lexed from the input that `StreamSplitter` is splitting.
//#[serde_as]
#[derive(Serialize)]
pub(crate) struct Record {
    /// The bytes lexed from the input.
    //#[serde_as(as = "BytesOrString")]
    // TODO: Rename this to `data`.
    pub(crate) bytes: Vec<u8>,

    /// The delimiter lexed from the input.
    //#[serde_as(as = "BytesOrString")]
    pub(crate) delimiter: Vec<u8>,
}

// TODO: Remove this allow as soon as it's not dead code.
#[allow(dead_code)]
impl Record {
    /// TODO: Document
    pub(crate) fn write_columns(&self, output: &mut dyn Write, delimiter: &[u8]) -> Result<()> {
        if !self.bytes.is_empty() {
            output.write_all(&self.bytes)?;
            output.write_all(delimiter)?;
        }
        Ok(())
    }

    /// TODO: Document
    pub(crate) fn write_json(&self, output: &mut dyn Write, pretty: bool) -> Result<()> {
        let to_json = if pretty {
            serde_json::to_string_pretty
        } else {
            serde_json::to_string
        };
        let json = to_json(self)?;
        output.write_all(json.as_bytes())?;
        Ok(())
    }
}

//impl Serialize for Record {
//    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//    where
//        S: Serializer,
//    {
//        let mut state = serializer.serialize_struct("Record", 2)?;
//        state.serialize_field("is_delimiter", &self.is_delimiter)?;
//        match from_utf8(&self.bytes) {
//            Ok(s) => {
//                state.serialize_field("bytes", s)?;
//            }
//            Err(_) => {
//                state.serialize_field("bytes", &self.bytes)?;
//            }
//        }
//        state.end()
//    }
//}

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

        // TODO: Clarify this whole blob by doing:
        //let section = &self.buffer[self.start..self.end];

        match self.delimiter.find(&self.buffer[self.start..self.end]) {
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
                let r = if m.start() == 0 {
                    // We matched the delimiter at the beginning of the section.
                    let start = self.start;
                    self.start += m.end();
                    Ok(Record {
                        bytes: Vec::new(),
                        delimiter: self.buffer[start + m.start()..start + m.end()].to_vec(),
                    })
                } else {
                    // We matched a record.
                    let start = self.start;
                    self.start += m.end();
                    Ok(Record {
                        bytes: self.buffer[start..start + m.start()].to_vec(),
                        delimiter: self.buffer[start + m.start()..start + m.end()].to_vec(),
                    })
                };
                Some(r)
            }
            None => {
                let start = self.start;
                self.start = self.end;
                Some(Ok(Record {
                    bytes: self.buffer[start..self.end].to_vec(),
                    delimiter: Vec::new(),
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
        assert_eq!(b"hello", r.bytes.as_slice());
        assert_eq!(b"\n\n", r.delimiter.as_slice());

        let r = splitter.next().unwrap().unwrap();
        assert_eq!(b"world", r.bytes.as_slice());
        assert_eq!(b"\n", r.delimiter.as_slice());

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
        assert_eq!(b"greetings", r.bytes.as_slice());
        assert_eq!(spaces, r.delimiter);

        let r = splitter.next().unwrap().unwrap();
        assert_eq!(b"world", r.bytes.as_slice());

        assert!(splitter.next().is_none());
    }
}
