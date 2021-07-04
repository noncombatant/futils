use getopt::Opt;
use memmap::{Mmap, MmapOptions};
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{stdout, Write};
use std::process::exit;

// TODO: Support regex someday.
//static DEFAULT_INPUT_DELIMITER: &str = r"(\r\n|\n|\r)";
static DEFAULT_INPUT_DELIMITER: &str = "\n";
static DEFAULT_OUTPUT_DELIMITER: &str = "\n";

// Utility Functions

// Cribbed from https://stackoverflow.com/posts/35907071/revisions. Thanks,
// Francis Gagné!
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

// TODO: This should be generic and separated out into its own library?
struct SubSlicer<'a> {
    slice: &'a [u8],
    input_delimiter: &'a [u8],
    start: usize,
}

impl<'a> Iterator for SubSlicer<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let i = find_subsequence(&self.slice[self.start..], self.input_delimiter);
        match i {
            Some(i) => {
                let sub_slice = &self.slice[self.start..self.start + i];
                self.start = self.start + i + self.input_delimiter.len();
                Some(sub_slice)
            }
            None => None,
        }
    }
}

fn map_file(pathname: &String) -> Option<Mmap> {
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

// Main functions

fn filter_help() {
    eprintln!("TODO: filter_help");
    exit(1);
}

fn filter_main(arguments: &[String]) {
    let mut options = getopt::Parser::new(&arguments, "d:hm:o:p:x:");

    let mut input_delimiter = String::from(DEFAULT_INPUT_DELIMITER);
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);
    let mut match_expression = String::new();
    let mut prune_expression = String::new();
    let mut match_command = String::new();
    loop {
        match options.next().transpose().unwrap() {
            None => break,
            Some(opt) => match opt {
                Opt('d', Some(string)) => input_delimiter = string.clone(),
                Opt('h', None) => filter_help(),
                Opt('m', Some(string)) => match_expression = string.clone(),
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                Opt('p', Some(string)) => prune_expression = string.clone(),
                Opt('x', Some(string)) => match_command = string.clone(),
                _ => filter_help(),
            },
        }
    }

    // TODO: Support this someday.
    //let input_delimiter = Regex::new(&input_delimiter).unwrap();
    let input_delimiter_bytes = input_delimiter.as_bytes();
    let output_delimiter_bytes = output_delimiter.as_bytes();

    let match_expression = Regex::new(&input_delimiter).unwrap();
    let prune_expression = Regex::new(&input_delimiter).unwrap();

    let (_, arguments) = arguments.split_at(options.index());

    if arguments.len() == 0 {
        // TODO: read stdin
    } else {
        for pathname in arguments {
            match map_file(&pathname) {
                Some(mapped) => {
                    let slicer = SubSlicer {
                        slice: &mapped,
                        input_delimiter: &input_delimiter_bytes,
                        start: 0,
                    };
                    for s in slicer {
                        if true
                        /* TODO */
                        {
                            stdout().write_all(s).unwrap();
                            stdout().write_all(b"\n").unwrap();
                        }
                    }
                }
                None => {}
            }
        }
    }
}

fn records_help() {
    eprintln!("TODO: records_help");
    exit(1);
}

fn records_main(arguments: &[String]) {
    let mut options = getopt::Parser::new(&arguments, "d:ho:");

    let mut input_delimiter = String::from(DEFAULT_INPUT_DELIMITER);
    let mut output_delimiter = String::from(DEFAULT_OUTPUT_DELIMITER);
    loop {
        match options.next().transpose().unwrap() {
            None => break,
            Some(opt) => match opt {
                Opt('d', Some(string)) => input_delimiter = string.clone(),
                Opt('h', None) => records_help(),
                Opt('o', Some(string)) => output_delimiter = string.clone(),
                _ => records_help(),
            },
        }
    }

    // TODO: Support this someday.
    //let input_delimiter = Regex::new(&input_delimiter).unwrap();
    let input_delimiter_bytes = input_delimiter.as_bytes();
    let output_delimiter_bytes = output_delimiter.as_bytes();

    let (_, arguments) = arguments.split_at(options.index());

    if arguments.len() == 0 {
        // TODO: read stdin
    } else {
        for pathname in arguments {
            match map_file(&pathname) {
                Some(mapped) => {
                    let slicer = SubSlicer {
                        slice: &mapped,
                        input_delimiter: &input_delimiter_bytes,
                        start: 0,
                    };
                    for s in slicer {
                        stdout().write_all(s).unwrap();
                        stdout().write_all(output_delimiter_bytes).unwrap();
                    }
                }
                None => {}
            }
        }
    }
}

fn fsutil_help() {
    eprintln!("TODO: fsutil_help");
    exit(1);
}

fn main() {
    let mut arguments = env::args().collect::<Vec<String>>();
    if arguments[0].ends_with("futils") {
        if arguments.len() < 2 {
            fsutil_help();
        }
        arguments.remove(0);
    }

    match arguments[0].as_str() {
        "filter" => filter_main(&arguments),
        "records" => records_main(&arguments),
        _ => fsutil_help(),
    }
}
