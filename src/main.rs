use std::env;
use std::error::Error;
use std::process::exit;

mod apply;
mod files;
mod filter;
mod records;
mod sub_slicer;
mod test;
mod util;

use apply::apply_main;
use files::files_main;
use filter::filter_main;
use records::records_main;
use test::test_main;
use crate::util::{file_name, ShellResult};

// TODO: Support regex someday.
//static DEFAULT_INPUT_DELIMITER: &str = r"(\r\n|\n|\r)";
static DEFAULT_INPUT_DELIMITER: &str = "\n";
static DEFAULT_OUTPUT_DELIMITER: &str = "\n";

fn futils_help(i: i32) -> ShellResult {
    eprintln!("TODO: futils_help");
    if i == 0 {
        Ok(i)
    } else {
        Err(Box::<dyn Error>::from("invalid invocation"))
    }
}

fn main() {
    let mut arguments = env::args().collect::<Vec<String>>();
    let basename = file_name(&arguments[0]).expect("Need a valid program name");
    if basename.eq("futils") {
        if arguments.len() < 2 {
            futils_help(-1).expect("NOTREACHED");
            exit(-1);
        }
        arguments.remove(0);
    }

    let basename = file_name(&arguments[0]).expect("Need a valid program name");
    let r = match basename {
        "apply" => apply_main(&arguments),
        "files" => files_main(&arguments),
        "filter" => filter_main(&arguments),
        "help" => futils_help(0),
        "records" => records_main(&arguments),

        "test" => test_main(&arguments),
        _ => futils_help(-1),
    };
    match r {
        Ok(i) => {
            exit(i)
        },
        Err(e) => {
            eprintln!("{}", e);
            exit(-1)
        }
    };
}
