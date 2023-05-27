// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils version` command.

use std::io::{stdout, Write};

use atty::Stream;
use serde::Serialize;

use crate::shell::{parse_options, ShellError, ShellResult};
use crate::util::help;

/// Command line usage help.
pub(crate) const VERSION_HELP: &str = include_str!("version_help.md");

pub(crate) const VERSION_HELP_VERBOSE: &str = include_str!("version_help_verbose.md");

#[derive(Serialize)]
struct VersionDatum<'a> {
    key: &'a str,
    value: &'a str,
}

impl VersionDatum<'_> {
    fn write_json(&self, output: &mut dyn Write, pretty: bool) -> Result<(), ShellError> {
        let to_json = if pretty {
            serde_json::to_writer_pretty
        } else {
            serde_json::to_writer
        };
        Ok(to_json(output, &self)?)
    }

    fn write_columns(
        &self,
        output: &mut dyn Write,
        field_delimiter: &[u8],
        record_delimiter: &[u8],
    ) -> Result<(), ShellError> {
        output.write_all(self.key.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.value.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(record_delimiter)?;
        Ok(())
    }
}

const VERSION_DATA: [VersionDatum; 15] = [
    VersionDatum {
        key: "Manifest directory",
        value: env!("CARGO_MANIFEST_DIR"),
    },
    VersionDatum {
        key: "Authors",
        value: env!("CARGO_PKG_AUTHORS"),
    },
    VersionDatum {
        key: "Description",
        value: env!("CARGO_PKG_DESCRIPTION"),
    },
    VersionDatum {
        key: "Homepage",
        value: env!("CARGO_PKG_HOMEPAGE"),
    },
    VersionDatum {
        key: "Name",
        value: env!("CARGO_PKG_NAME"),
    },
    VersionDatum {
        key: "Repository",
        value: env!("CARGO_PKG_REPOSITORY"),
    },
    VersionDatum {
        key: "Version",
        value: env!("CARGO_PKG_VERSION"),
    },
    VersionDatum {
        key: "Version (major)",
        value: env!("CARGO_PKG_VERSION_MAJOR"),
    },
    VersionDatum {
        key: "Version (minor)",
        value: env!("CARGO_PKG_VERSION_MINOR"),
    },
    VersionDatum {
        key: "Version (patch)",
        value: env!("CARGO_PKG_VERSION_PATCH"),
    },
    VersionDatum {
        key: "Version (pre)",
        value: env!("CARGO_PKG_VERSION_PRE"),
    },
    VersionDatum {
        key: "Rust version",
        value: env!("CARGO_PKG_RUST_VERSION"),
    },
    VersionDatum {
        key: "License",
        value: env!("CARGO_PKG_LICENSE"),
    },
    VersionDatum {
        key: "Binary",
        value: env!("CARGO_BIN_NAME"),
    },
    VersionDatum {
        key: "Crate",
        value: env!("CARGO_CRATE_NAME"),
    },
];

pub(crate) fn version_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help || !arguments.is_empty() {
        help(
            0,
            VERSION_HELP,
            if options.verbose {
                Some(VERSION_HELP_VERBOSE)
            } else {
                None
            },
        );
    }

    let mut stdout = stdout();
    if options.json_output {
        let count = VERSION_DATA.len();
        println!("[");
        for (i, d) in VERSION_DATA.iter().enumerate() {
            d.write_json(&mut stdout, atty::is(Stream::Stdout))?;
            stdout.write_all(if i < count - 1 { b",\n" } else { b"\n" })?;
        }
        println!("]");
    } else {
        for d in VERSION_DATA {
            d.write_columns(
                &mut stdout,
                &options.output_field_delimiter,
                &options.output_record_delimiter,
            )?;
        }
    }
    Ok(0)
}
