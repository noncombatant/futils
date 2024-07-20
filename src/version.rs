// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils version` command.

use std::io::{stdout, Write};

use atty::Stream;
use serde::Serialize;

use crate::shell::{parse_options, EmptyResult, ShellResult};
use crate::util::{exit_with_result, help};

pub const VERSION_HELP: &str = include_str!("version.md");
pub const VERSION_HELP_VERBOSE: &str = include_str!("version_verbose.md");

#[derive(Serialize)]
struct Metadata {
    name: &'static str,
    description: &'static str,
    version: &'static str,
    version_major: &'static str,
    version_minor: &'static str,
    version_patch: &'static str,
    version_pre: &'static str,
    repository: &'static str,
    license: &'static str,
    authors: &'static str,
    binary_name: &'static str,
    crate_name: &'static str,
}

impl Metadata {
    fn write_json(&self, output: &mut dyn Write, pretty: bool) -> EmptyResult {
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
        verbose: bool,
    ) -> EmptyResult {
        if verbose {
            write!(output, "Name")?;
            output.write_all(field_delimiter)?;
            output.write_all(self.name.as_bytes())?;
            output.write_all(record_delimiter)?;

            write!(output, "Description")?;
            output.write_all(field_delimiter)?;
            output.write_all(self.description.as_bytes())?;
            output.write_all(record_delimiter)?;

            write!(output, "Version")?;
            output.write_all(field_delimiter)?;
            output.write_all(self.version.as_bytes())?;
            output.write_all(record_delimiter)?;

            write!(output, "Version (major)")?;
            output.write_all(field_delimiter)?;
            output.write_all(self.version_major.as_bytes())?;
            output.write_all(record_delimiter)?;

            write!(output, "Version (minor)")?;
            output.write_all(field_delimiter)?;
            output.write_all(self.version_minor.as_bytes())?;
            output.write_all(record_delimiter)?;

            write!(output, "Version (patch)")?;
            output.write_all(field_delimiter)?;
            output.write_all(self.version_patch.as_bytes())?;
            output.write_all(record_delimiter)?;

            write!(output, "Version (pre)")?;
            output.write_all(field_delimiter)?;
            output.write_all(self.version_pre.as_bytes())?;
            output.write_all(record_delimiter)?;

            write!(output, "Repository")?;
            output.write_all(field_delimiter)?;
            output.write_all(self.repository.as_bytes())?;
            output.write_all(record_delimiter)?;

            write!(output, "License")?;
            output.write_all(field_delimiter)?;
            output.write_all(self.license.as_bytes())?;
            output.write_all(record_delimiter)?;

            write!(output, "Authors")?;
            output.write_all(field_delimiter)?;
            output.write_all(self.authors.as_bytes())?;
            output.write_all(record_delimiter)?;

            write!(output, "Binary name")?;
            output.write_all(field_delimiter)?;
            output.write_all(self.binary_name.as_bytes())?;
            output.write_all(record_delimiter)?;

            write!(output, "Crate name")?;
            output.write_all(field_delimiter)?;
            output.write_all(self.crate_name.as_bytes())?;
            output.write_all(record_delimiter)?;
        } else {
            output.write_all(self.version.as_bytes())?;
            output.write_all(record_delimiter)?;
        }
        Ok(())
    }
}

const METADATA: Metadata = Metadata {
    name: env!("CARGO_PKG_NAME"),
    description: env!("CARGO_PKG_DESCRIPTION"),
    version: env!("CARGO_PKG_VERSION"),
    version_major: env!("CARGO_PKG_VERSION_MAJOR"),
    version_minor: env!("CARGO_PKG_VERSION_MINOR"),
    version_patch: env!("CARGO_PKG_VERSION_PATCH"),
    version_pre: env!("CARGO_PKG_VERSION_PRE"),
    repository: env!("CARGO_PKG_REPOSITORY"),
    license: env!("CARGO_PKG_LICENSE"),
    authors: env!("CARGO_PKG_AUTHORS"),
    binary_name: env!("CARGO_BIN_NAME"),
    crate_name: env!("CARGO_CRATE_NAME"),
};

pub fn version_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help || !arguments.is_empty() {
        exit_with_result(help(
            0,
            VERSION_HELP,
            true,
            if options.verbose {
                Some(VERSION_HELP_VERBOSE)
            } else {
                None
            },
        ));
    }

    let mut stdout = stdout();
    if options.json_output {
        METADATA.write_json(&mut stdout, atty::is(Stream::Stdout))?;
    } else {
        METADATA.write_columns(
            &mut stdout,
            &options.output_field_delimiter,
            &options.output_record_delimiter,
            options.verbose,
        )?;
    }
    Ok(0)
}
