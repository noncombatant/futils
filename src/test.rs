use regex::bytes::Regex;
use std::io::{stdin, stdout, Write};

use crate::shell::ShellResult;
use crate::stream_splitter::*;

pub fn test_main(_: &[String]) -> ShellResult {
    let input_delimiter = Regex::new(r"(\r\n|\n|\r)+")?;
    let output_delimiter = String::from("--\n");
    let mut stdin = stdin();
    let splitter = StreamSplitter::new(&mut stdin, &input_delimiter);

    for r in splitter {
        if r.is_delimiter {
            stdout().write_all("DELIMITER--".as_bytes())?;
        } else {
            stdout().write_all("DATA--".as_bytes())?;
        }
        stdout().write_all(&r.bytes)?;
        stdout().write_all(output_delimiter.as_bytes())?;
    }

    Ok(0)
}
