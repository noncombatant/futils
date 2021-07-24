use std::env;
use std::process::exit;

mod filter;
mod records;
mod sub_slicer;
mod test;
mod util;

use filter::filter_main;
use records::records_main;
use test::test_main;

// TODO: Support regex someday.
//static DEFAULT_INPUT_DELIMITER: &str = r"(\r\n|\n|\r)";
static DEFAULT_INPUT_DELIMITER: &str = "\n";
static DEFAULT_OUTPUT_DELIMITER: &str = "\n";

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

        "test" => test_main(&arguments),
        _ => fsutil_help(),
    }
}
