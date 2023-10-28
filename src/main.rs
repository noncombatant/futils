// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils` command.

use std::env;
use std::process::exit;

mod common;
mod enumerated_record;
mod fields;
mod files;
mod filter;
mod map;
mod mapx;
mod markdown;
mod records;
mod reduce;
mod shell;
mod status;
mod stream_splitter;
mod time;
mod util;
mod version;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_os = "macos", path = "darwin.rs")]
mod os;

use common::{common_main, COMMON_HELP};
use fields::{fields_main, FIELDS_HELP};
use files::{files_main, FILES_HELP};
use filter::{filter_main, FILTER_HELP};
use map::{map_main, MAP_HELP};
use mapx::{mapx_main, MAPX_HELP};
use markdown::{markdown_main, MARKDOWN_HELP};
use records::{records_main, RECORDS_HELP};
use reduce::{reduce_main, REDUCE_HELP};
use status::{status_main, STATUS_HELP};
use util::{file_name, help};
use version::{version_main, VERSION_HELP};

const MAIN_HELP: &str = include_str!("main.md");

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
        help(-1, MAIN_HELP, false, None);
    } else {
        program_name = file_name(&arguments[0]).unwrap();
    }

    if program_name == "help" || program_name == "-h" || program_name == "--help" {
        if arguments.len() < 2 {
            help(0, MAIN_HELP, false, None);
        } else {
            match arguments[1].as_str() {
                "common" => help(0, COMMON_HELP, true, None),
                "fields" => help(0, FIELDS_HELP, true, None),
                "files" => help(0, FILES_HELP, true, None),
                "filter" => help(0, FILTER_HELP, true, None),
                "map" => help(0, MAP_HELP, true, None),
                "mapx" => help(0, MAPX_HELP, true, None),
                "markdown" => help(0, MARKDOWN_HELP, true, None),
                "records" => help(0, RECORDS_HELP, true, None),
                "reduce" => help(0, REDUCE_HELP, true, None),
                "status" => help(0, STATUS_HELP, true, None),
                "version" => help(0, VERSION_HELP, true, None),
                &_ => help(-1, MAIN_HELP, false, None),
            };
        }
    }

    match match program_name {
        "common" => common_main(&arguments),
        "fields" => fields_main(&arguments),
        "files" => files_main(&arguments),
        "filter" => filter_main(&arguments),
        "map" => map_main(&arguments),
        "mapx" => mapx_main(&arguments),
        "markdown" => markdown_main(&arguments),
        "records" => records_main(&arguments),
        "reduce" => reduce_main(&arguments),
        "status" => status_main(&arguments),
        "version" => version_main(&arguments),
        _ => {
            help(-1, MAIN_HELP, false, None);
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
