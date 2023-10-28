// Copyright 2023 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

use std::io::{Error, Write};

use serde::Serialize;

use crate::shell::Options;
use crate::util::serialize_str_or_bytes;

#[derive(Serialize)]
pub(crate) struct EnumeratedRecord<'a> {
    pub(crate) n: Option<usize>,
    pub(crate) pathname: &'a str,
    #[serde(serialize_with = "serialize_str_or_bytes")]
    pub(crate) r: Vec<u8>,
}

impl EnumeratedRecord<'_> {
    pub(crate) fn write_columns(
        &self,
        output: &mut dyn Write,
        options: &Options,
    ) -> Result<(), Error> {
        if options.print_empty || !self.r.is_empty() {
            if let Some(n) = self.n {
                output.write_all(self.pathname.as_bytes())?;
                output.write_all(&options.output_field_delimiter)?;
                write!(output, "{:>5}", n + 1)?;
                output.write_all(&options.output_field_delimiter)?;
            }
            output.write_all(&self.r)?;
            output.write_all(&options.output_record_delimiter)?;
        }
        Ok(())
    }

    pub(crate) fn write_json(
        &self,
        output: &mut dyn Write,
        pretty: bool,
        options: &Options,
    ) -> Result<(), Error> {
        if options.print_empty || !self.r.is_empty() {
            let to_json = if pretty {
                serde_json::to_writer_pretty
            } else {
                serde_json::to_writer
            };
            to_json(output, &self)?;
        }
        Ok(())
    }
}
