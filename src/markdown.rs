// Copyright 2023 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

use crate::{
    shell::{FileOpener, STDIN_PATHNAME, ShellResult, parse_options},
    util::{exit_with_result, help, skin, terminal_text},
};

pub const MARKDOWN_HELP: &str = include_str!("markdown.md");

/// Runs the `markdown` command on `arguments`.
pub fn markdown_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        exit_with_result(help(0, MARKDOWN_HELP, false, None));
    }

    let skin = skin();
    let mut status = 0;
    for file in FileOpener::new(arguments) {
        match file.read {
            Ok(mut read) => {
                let mut buffer = String::new();
                match read.read_to_string(&mut buffer) {
                    Ok(_) => {
                        println!("{}", terminal_text(&buffer, &skin));
                    }
                    Err(error) => {
                        let pathname = file.pathname.unwrap_or(&STDIN_PATHNAME);
                        eprintln!("{pathname}: {error}");
                        status += 1;
                    }
                }
            }
            Err(error) => {
                let pathname = file.pathname.unwrap_or(&STDIN_PATHNAME);
                eprintln!("{pathname}: {error}");
                status += 1;
            }
        }
    }
    Ok(status)
}
