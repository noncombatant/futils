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
mod time;
mod util;

use apply::{apply_main, APPLY_HELP_MESSAGE};
use fields::{fields_main, FIELDS_HELP_MESSAGE};
use files::{files_main, FILES_HELP_MESSAGE};
use filter::{filter_main, FILTER_HELP_MESSAGE};
use records::{records_main, RECORDS_HELP_MESSAGE};
use status::{status_main, STATUS_HELP_MESSAGE};
use util::{file_name, help};

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
    // TODO: Use `args_os`, and propagate the API change throughout (!).
    let mut arguments = env::args().collect::<Vec<String>>();

    // If we were invoked as `futils`, shift the arguments left.
    let program_name = arguments[0].clone();
    let mut program_name = file_name(&program_name).unwrap();
    if program_name.eq("futils") {
        arguments.remove(0);
    }
    if arguments.is_empty() {
        help(-1, HELP_MESSAGE);
    } else {
        program_name = file_name(&arguments[0]).unwrap();
    }

    if program_name == "help" || program_name == "-h" || program_name == "--help" {
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

    if let Err(e) = match program_name {
        "apply" => apply_main(&arguments),
        "fields" => fields_main(&arguments),
        "files" => files_main(&arguments),
        "filter" => filter_main(&arguments),
        "records" => records_main(&arguments),
        "status" => status_main(&arguments),
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
