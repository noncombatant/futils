// Copyright 2023 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

use crate::shell::{parse_options, FileOpener, ShellResult, STDIN_PATHNAME};
use crate::util::{help, get_skin};

pub(crate) const MARKDOWN_HELP: &str = include_str!("markdown.md");

/// Runs the `markdown` command on `arguments`.
pub(crate) fn markdown_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, MARKDOWN_HELP, false, None);
    }

    let skin = get_skin();
    let mut status = 0;
    for file in FileOpener::new(arguments) {
        match file.read {
            Ok(mut read) => {
                let mut buffer = String::new();
                match read.read_to_string(&mut buffer) {
                    Ok(_) => {
                        println!("{}", skin.text(&buffer, None))
                    }
                    Err(e) => {
                        let p = file.pathname.unwrap_or(&STDIN_PATHNAME);
                        eprintln!("{}: {}", p, e);
                        status += 1;
                    }
                }
            }
            Err(e) => {
                let p = file.pathname.unwrap_or(&STDIN_PATHNAME);
                eprintln!("{}: {}", p, e);
                status += 1;
            }
        }
    }
    Ok(status)
}
