use std::env;
use std::process::exit;

mod apply;
mod fields;
mod files;
mod filter;
mod records;
mod shell;
mod status;
mod stream_splitter;
mod test;
mod time;
mod util;

use apply::{apply_main, APPLY_HELP_MESSAGE};
use fields::{fields_main, FIELDS_HELP_MESSAGE};
use files::{files_main, FILES_HELP_MESSAGE};
use filter::{filter_main, FILTER_HELP_MESSAGE};
use records::{records_main, RECORDS_HELP_MESSAGE};
use status::{status_main, STATUS_HELP_MESSAGE};
use util::{file_name, help};

/// Command line usage help.
const HELP_MESSAGE: &str = include_str!("main_help.md");

fn reset_sigpipe() {
    if cfg!(unix) {
        unsafe {
            libc::signal(libc::SIGPIPE, libc::SIG_DFL);
        }
    }
}

fn main() {
    reset_sigpipe();

    // TODO: Make `arguments` be `Vec<OsString>`, and propagate the API change
    // throughout (!). This probably means we can't use `getopt`, which seems to
    // want only `String`s. Perhaps `clap` is the way to go. Until then, the
    // least we can do is warn people:
    let args_os = env::args_os();
    let mut arguments: Vec<String> = Vec::with_capacity(args_os.size_hint().0);
    for a in args_os {
        match a.to_str() {
            Some(s) => arguments.push(String::from(s)),
            None => eprintln!(
                "Could not convert \"{}\" to UTF-8. Skipping.",
                a.to_string_lossy()
            ),
        }
    }

    // If we were invoked as `futils`, shift the arguments left.
    let program_name = arguments[0].clone();
    let mut program_name = file_name(&program_name).unwrap();
    if program_name.eq("futils") {
        arguments.remove(0);
    }
    if arguments.is_empty() {
        help(-1, HELP_MESSAGE);
    } else {
        program_name = file_name(&arguments[0]).unwrap();
    }

    if program_name == "help" || program_name == "-h" || program_name == "--help" {
        if arguments.len() < 2 {
            help(0, HELP_MESSAGE);
        } else {
            match arguments[1].as_str() {
                "apply" => help(0, APPLY_HELP_MESSAGE),
                "fields" => help(0, FIELDS_HELP_MESSAGE),
                "files" => help(0, FILES_HELP_MESSAGE),
                "filter" => help(0, FILTER_HELP_MESSAGE),
                "records" => help(0, RECORDS_HELP_MESSAGE),
                "status" => help(0, STATUS_HELP_MESSAGE),
                &_ => help(-1, HELP_MESSAGE),
            };
        }
    }

    match match program_name {
        "apply" => apply_main(&arguments),
        "fields" => fields_main(&arguments),
        "files" => files_main(&arguments),
        "filter" => filter_main(&arguments),
        "records" => records_main(&arguments),
        "status" => status_main(&arguments),
        _ => {
            help(-1, HELP_MESSAGE);
            unreachable!()
        }
    } {
        Ok(status) => exit(status),
        Err(e) => {
            eprintln!("{}", e);
            exit(-1)
        }
    }
}
