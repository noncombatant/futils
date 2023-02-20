use std::env;
use std::process::exit;

mod apply;
mod fields;
mod files;
mod filter;
mod records;
mod shell;
mod status;
mod stream_splitter;
mod sub_slicer;
mod test;
mod time;
mod util;

use apply::{apply_main, APPLY_HELP_MESSAGE};
use fields::{fields_main, FIELDS_HELP_MESSAGE};
use files::{files_main, FILES_HELP_MESSAGE};
use filter::{filter_main, FILTER_HELP_MESSAGE};
use records::{records_main, RECORDS_HELP_MESSAGE};
use status::{status_main, STATUS_HELP_MESSAGE};
use test::test_main;
use util::{file_name, help};

const DEFAULT_INPUT_RECORD_DELIMITER: &str = r"(\r|\n)+";
//const DEFAULT_INPUT_FIELD_DELIMITER: &str = r"\s+";
const DEFAULT_OUTPUT_RECORD_DELIMITER: &str = "\n";
const DEFAULT_OUTPUT_FIELD_DELIMITER: &str = "\t";

const HELP_MESSAGE: &str = "# `futils` - functional shell utilities

## Usage

```
futils -h
futiles help
```

## Description

`futils` is a suite of shell utilities that somewhat resemble functional
programming primitives and operate on streams.

`futils` has various sub-commands:

* `apply`
* `fields`
* `files`
* `filter`
* `records`
* `status`

…and more to come.

To learn more about each one, run

```
futils sub-command -h
```

e.g.

```
futils apply -h
```

You can also invoke `futils` utilities directly, e.g.

```
apply -h
files -h
```

…and so on.";

fn main() {
    let mut arguments = env::args().collect::<Vec<String>>();
    let conversion_error_message = "Need a valid program name";
    let basename = file_name(&arguments[0]).expect(conversion_error_message);
    if basename.eq("futils") {
        if arguments.len() < 2 {
            help(-1, HELP_MESSAGE);
        }
        arguments.remove(0);
        if arguments[0] == "help" || arguments[0] == "-h" {
            if arguments.len() < 2 {
                help(0, HELP_MESSAGE);
            } else {
                match arguments[1].as_str() {
                    "apply" => help(0, APPLY_HELP_MESSAGE),
                    "fields" => help(0, FIELDS_HELP_MESSAGE),
                    "files" => help(0, FILES_HELP_MESSAGE),
                    "filter" => help(0, FILTER_HELP_MESSAGE),
                    "records" => help(0, RECORDS_HELP_MESSAGE),
                    "status" => help(0, STATUS_HELP_MESSAGE),
                    &_ => help(-1, HELP_MESSAGE),
                };
            }
        }
    }

    let basename = file_name(&arguments[0]).expect(conversion_error_message);
    if basename == "help" {
        help(0, HELP_MESSAGE);
    }
    if let Err(e) = match basename {
        "apply" => apply_main(&arguments),
        "fields" => fields_main(&arguments),
        "files" => files_main(&arguments),
        "filter" => filter_main(&arguments),
        "records" => records_main(&arguments),
        "status" => status_main(&arguments),
        "test" => test_main(&arguments),
        _ => {
            help(-1, HELP_MESSAGE);
            unreachable!()
        }
    } {
        eprintln!("{}", e);
        // TODO: Exit with the exit code from the `*_main` callee.
        exit(-1)
    }
}
