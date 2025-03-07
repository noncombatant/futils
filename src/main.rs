// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils` command.

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::single_call_fn,
    clippy::print_stderr,
    clippy::missing_docs_in_private_items,
    clippy::cast_sign_loss,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::multiple_crate_versions,
    clippy::similar_names,
    clippy::trivial_regex,
    clippy::cast_lossless  // TODO: Fix (fails on Linux)
)]
#![deny(warnings)]

mod common;
mod enumerated_record;
mod fields;
mod fileid;
mod files;
mod filter;
mod map;
mod mapx;
mod markdown;
mod records;
mod shell;
mod status;
mod time;
mod util;
mod version;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_os = "macos", path = "darwin.rs")]
mod os;

use common::{COMMON_HELP, common_main};
use fields::{FIELDS_HELP, fields_main};
use fileid::{FILEID_HELP, fileid_main};
use files::{FILES_HELP, files_main};
use filter::{FILTER_HELP, filter_main};
use map::{MAP_HELP, map_main};
use mapx::{MAPX_HELP, mapx_main};
use markdown::{MARKDOWN_HELP, markdown_main};
use records::{RECORDS_HELP, records_main};
use status::{STATUS_HELP, status_main};
use std::{env, process::exit};
use util::{exit_with_result, file_name, help};
use version::{VERSION_HELP, version_main};

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
    for arg in args_os {
        match arg.to_str() {
            Some(string) => arguments.push(String::from(string)),
            None => eprintln!(
                "Could not convert \"{}\" to UTF-8. Skipping.",
                arg.to_string_lossy()
            ),
        }
    }

    // If we were invoked as `futils`, shift the arguments left.
    let program_name = arguments[0].clone();
    let mut program_name = file_name(&program_name).expect("could not get program name");
    if program_name.eq("futils") {
        arguments.remove(0);
    }
    if arguments.is_empty() {
        exit_with_result(help(-1, MAIN_HELP, false, None));
    } else {
        program_name = file_name(&arguments[0]).expect("could not get program name");
    }

    if program_name == "help" || program_name == "-h" || program_name == "--help" {
        exit_with_result(if arguments.len() < 2 {
            help(0, MAIN_HELP, false, None)
        } else {
            match arguments[1].as_str() {
                "common" => help(0, COMMON_HELP, true, None),
                "fields" => help(0, FIELDS_HELP, true, None),
                "fileid" => help(0, FILEID_HELP, true, None),
                "files" => help(0, FILES_HELP, true, None),
                "filter" => help(0, FILTER_HELP, true, None),
                "map" => help(0, MAP_HELP, true, None),
                "mapx" => help(0, MAPX_HELP, true, None),
                "markdown" => help(0, MARKDOWN_HELP, true, None),
                "records" => help(0, RECORDS_HELP, true, None),
                "status" => help(0, STATUS_HELP, true, None),
                "version" => help(0, VERSION_HELP, true, None),
                &_ => help(-1, MAIN_HELP, false, None),
            }
        });
    }

    match match program_name {
        "common" => common_main(&arguments),
        "fields" => fields_main(&arguments),
        "fileid" => fileid_main(&arguments),
        "files" => files_main(&arguments),
        "filter" => filter_main(&arguments),
        "map" => map_main(&arguments),
        "mapx" => mapx_main(&arguments),
        "markdown" => markdown_main(&arguments),
        "records" => records_main(&arguments),
        "status" => status_main(&arguments),
        "version" => version_main(&arguments),
        _ => help(-1, MAIN_HELP, false, None),
    } {
        Ok(status) => exit(status),
        Err(error) => {
            eprintln!("{error}");
            exit(-1)
        }
    }
}
