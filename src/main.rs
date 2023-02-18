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

use apply::apply_main;
use files::files_main;
use filter::filter_main;
use records::records_main;
use status::status_main;
use util::{file_name, help};

// TODO: Support regex someday.
//static DEFAULT_INPUT_DELIMITER: &str = r"(\r\n|\n|\r)";
static DEFAULT_INPUT_DELIMITER: &str = "\n";
static DEFAULT_OUTPUT_DELIMITER: &str = "\n";

const HELP_MESSAGE: &str = "futils - functional shell utilities

Usage:

`futils` has various sub-commands:

* `apply`
* `files`
* `filter`
* `help`
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

If installed correctly, `futils` should also allow you invoke the utilities
directly, e.g.

```
apply -h
files -h
```

…and so on.";

fn main() {
    let mut arguments = env::args().collect::<Vec<String>>();
    // TODO: Parse options and support -h here.
    let conversion_error_message = "Need a valid program name";
    let basename = file_name(&arguments[0]).expect(conversion_error_message);
    if basename.eq("futils") {
        if arguments.len() < 2 {
            help(-1, HELP_MESSAGE);
        }
        arguments.remove(0);
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
