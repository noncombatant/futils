use getopt::Opt;
use memmap::MmapOptions;
//use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{stdout, Write};
use std::process::exit;

// TODO: Support regex someday.
//static DEFAULT_INPUT_DELIMITER: &str = r"(\r\n|\n|\r)";
static DEFAULT_INPUT_DELIMITER: &str = "\n";
static DEFAULT_OUTPUT_DELIMITER: &str = "\n";

// Cribbed from https://stackoverflow.com/posts/35907071/revisions. Thanks,
// Francis Gagné!
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn find_and_write_subsequences(haystack: &[u8], input_delimiter: &[u8], output_delimiter: &[u8]) {
    let mut start: usize = 0;
    let mut out = stdout();
    let input_delimiter_length = input_delimiter.len();
    loop {
        if start >= haystack.len() {
            break;
        }
        let i = find_subsequence(&haystack[start..], input_delimiter);
        match i {
            Some(i) => {
                out.write_all(&haystack[start..start + i]).unwrap();
                out.write_all(output_delimiter).unwrap();
                start = start + i + input_delimiter_length;
            }
            None => {
                out.write_all(&haystack[start..]).unwrap();
                out.write_all(output_delimiter).unwrap();
                break;
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
            let file = File::open(&pathname);
            match file {
                Ok(file) => {
                    let mapped = unsafe {
                        let m = MmapOptions::new().map(&file);
                        match m {
                            Ok(m) => m,
                            Err(e) => {
                                eprintln!("{}: {}", &pathname, e);
                                continue;
                            }
                        }
                    };
                    find_and_write_subsequences(
                        &mapped,
                        input_delimiter_bytes,
                        output_delimiter_bytes,
                    );
                }
                Err(e) => eprintln!("{}: {}", &pathname, e),
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
        "records" => records_main(&arguments),
        _ => fsutil_help(),
    }
}
