// Copyright 2024 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils fileid` command.

use std::fs;
use std::io::{copy, stdout, Error, Write};
use std::os::unix::fs::MetadataExt;

use atty::Stream;
use base64ct::{Base64, Encoding};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::shell::{parse_options, Options, ShellResult, StructuredWrite};
use crate::util::help;

pub(crate) const FILEID_HELP: &str = include_str!("fileid.md");
pub(crate) const FILEID_HELP_VERBOSE: &str = include_str!("fileid_verbose.md");

#[derive(Serialize)]
struct FileID {
    hash: Option<String>,
    device: u64,
    inode: u64,
    size: u64,
    pathname: String,
}

impl FileID {
    fn write_columns(&self, output: &mut dyn Write, options: &Options) -> Result<(), Error> {
        match &self.hash {
            Some(hash) => {
                write!(output, "{:<44}", hash)?;
                output.write_all(&options.output_field_delimiter)?;
            }
            None => {}
        }
        write!(output, "{:>9}", self.device)?;
        output.write_all(&options.output_field_delimiter)?;
        write!(output, "{:>9}", self.inode)?;
        output.write_all(&options.output_field_delimiter)?;
        write!(output, "{:>9}", self.size)?;
        output.write_all(&options.output_field_delimiter)?;
        output.write_all(self.pathname.as_bytes())?;
        output.write_all(&options.output_field_delimiter)?;
        output.write_all(&options.output_record_delimiter)?;
        Ok(())
    }

    fn write_json(&self, output: &mut dyn Write, pretty: bool) -> Result<(), Error> {
        let to_json = if pretty {
            serde_json::to_writer_pretty
        } else {
            serde_json::to_writer
        };
        to_json(output, &self)?;
        Ok(())
    }
}

impl StructuredWrite for FileID {
    fn write(&self, output: &mut dyn Write, options: &Options) -> Result<(), Error> {
        if options.json_output {
            // TODO: `pretty` should be a command-line switch.
            self.write_json(output, atty::is(Stream::Stdout))
        } else {
            self.write_columns(output, options)
        }
    }
}

fn get_fileid(pathname: &str, verbose: bool) -> std::io::Result<FileID> {
    let mut file = fs::File::open(pathname)?;
    let metadata = file.metadata()?;
    Ok(FileID {
        hash: if verbose {
            Some(if metadata.is_symlink() {
                "symlink".to_string()
            } else if metadata.is_file() {
                let mut hasher = Sha256::new();
                let _ = copy(&mut file, &mut hasher)?;
                Base64::encode_string(&hasher.finalize())
            } else if metadata.is_dir() {
                "directory".to_string()
            } else {
                "?".to_string()
            })
        } else {
            None
        },
        device: metadata.dev(),
        inode: metadata.ino(),
        size: metadata.len(),
        pathname: pathname.to_string(),
    })
}

/// Runs the `fileid` command on `arguments`.
pub(crate) fn fileid_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(
            0,
            FILEID_HELP,
            true,
            if options.verbose {
                Some(FILEID_HELP_VERBOSE)
            } else {
                None
            },
        );
    }

    let mut status = 0;
    for pathname in arguments {
        match get_fileid(pathname, options.verbose) {
            Ok(file_id) => {
                file_id.write(&mut stdout(), &options)?;
            }
            Err(e) => {
                eprintln!("{}", e);
                status += 1;
            }
        }
    }
    Ok(status)
}
