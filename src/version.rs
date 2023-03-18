use atty::Stream;
use serde::Serialize;
use std::io::{stdout, Error, Write};

use crate::shell::{parse_options, ShellResult};
use crate::util::help;

/// Command line usage help.
pub(crate) const VERSION_HELP_PAGE: &str = include_str!("version_help.md");

#[derive(Serialize)]
struct VersionDatum<'a> {
    key: &'a str,
    value: &'a str,
}

impl VersionDatum<'_> {
    // TODO: This should take `output` as a `dyn io::Write`.
    fn write_json(&self) -> Result<(), Error> {
        let to_json = if atty::is(Stream::Stdout) {
            serde_json::to_string_pretty
        } else {
            serde_json::to_string
        };
        // TODO: Don't `unwrap` here; handle the error.
        let json = to_json(self).unwrap();
        let mut output = stdout();
        output.write_all(json.as_bytes())?;
        Ok(())
    }

    // TODO: This should take `output` as a `dyn io::Write`.
    fn write_columns(&self, field_delimiter: &[u8], record_delimiter: &[u8]) -> Result<(), Error> {
        let mut output = stdout();
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
        help(0, VERSION_HELP_PAGE);
    }

    if options.json {
        let count = VERSION_DATA.len();
        println!("[");
        for (i, d) in VERSION_DATA.iter().enumerate() {
            d.write_json()?;
            stdout().write_all(if i < count - 1 { b",\n" } else { b"\n" })?;
        }
        println!("]");
    } else {
        for d in VERSION_DATA {
            d.write_columns(
                &options.output_field_delimiter,
                &options.output_record_delimiter,
            )?;
        }
    }
    Ok(0)
}
