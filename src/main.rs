use std::env;
use std::process::exit;

mod apply;
mod files;
mod filter;
mod records;
mod status;
mod sub_slicer;
mod time;
mod util;

use apply::{apply_main, APPLY_HELP_MESSAGE};
use files::{files_main, FILES_HELP_MESSAGE};
use filter::{filter_main, FILTER_HELP_MESSAGE};
use records::{records_main, RECORDS_HELP_MESSAGE};
use status::{status_main, STATUS_HELP_MESSAGE};
use util::{file_name, help};

// TODO: Support regex someday.
//static DEFAULT_INPUT_DELIMITER: &str = r"(\r\n|\n|\r)";
static DEFAULT_INPUT_DELIMITER: &str = "\n";
static DEFAULT_OUTPUT_DELIMITER: &str = "\n";

const HELP_MESSAGE: &str = "futils - functional shell utilities

Usage:

```
futils -h
futiles help
```

`futils` has various sub-commands:

* `apply`
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
