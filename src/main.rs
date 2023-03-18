use std::env;
use std::process::exit;

mod apply;
mod fields;
mod files;
mod filter;
mod map;
mod records;
mod shell;
mod status;
mod stream_splitter;
mod test;
mod time;
mod util;
mod version;

use apply::{apply_main, APPLY_HELP_PAGE};
use fields::{fields_main, FIELDS_HELP_PAGE};
use files::{files_main, FILES_HELP_PAGE};
use filter::{filter_main, FILTER_HELP_PAGE};
use map::{map_main, MAP_HELP_PAGE};
use records::{records_main, RECORDS_HELP_PAGE};
use status::{status_main, STATUS_HELP_PAGE};
use util::{file_name, help};
use version::{version_main, VERSION_HELP_PAGE};

/// Command line usage help.
const HELP_PAGE: &str = include_str!("main_help.md");

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
        help(-1, HELP_PAGE);
    } else {
        program_name = file_name(&arguments[0]).unwrap();
    }

    if program_name == "help" || program_name == "-h" || program_name == "--help" {
        if arguments.len() < 2 {
            help(0, HELP_PAGE);
        } else {
            match arguments[1].as_str() {
                "apply" => help(0, APPLY_HELP_PAGE),
                "fields" => help(0, FIELDS_HELP_PAGE),
                "files" => help(0, FILES_HELP_PAGE),
                "filter" => help(0, FILTER_HELP_PAGE),
                "map" => help(0, MAP_HELP_PAGE),
                "records" => help(0, RECORDS_HELP_PAGE),
                "status" => help(0, STATUS_HELP_PAGE),
                "version" => help(0, VERSION_HELP_PAGE),
                &_ => help(-1, HELP_PAGE),
            };
        }
    }

    match match program_name {
        "apply" => apply_main(&arguments),
        "fields" => fields_main(&arguments),
        "files" => files_main(&arguments),
        "filter" => filter_main(&arguments),
        "map" => map_main(&arguments),
        "records" => records_main(&arguments),
        "status" => status_main(&arguments),
        "version" => version_main(&arguments),
        _ => {
            help(-1, HELP_PAGE);
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
