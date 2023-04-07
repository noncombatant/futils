use std::process::Command;
// TODO: https://docs.rs/assert_cmd/latest/assert_cmd/cmd/struct.Command.html
// instead.

#[cfg(test)]
const FUTILS: &str = "target/debug/futils";

#[cfg(test)]
struct TestCase<'a> {
    program: &'a str,
    arguments: &'a [&'a str],
    expected: &'a str,
}

#[cfg(test)]
impl<'a> TestCase<'a> {
    fn new(program: &'a str, arguments: &'a [&'a str], expected: &'a str) -> Self {
        Self {
            program,
            arguments,
            expected,
        }
    }

    fn run(&self) {
        let output = Command::new(FUTILS)
            .arg(self.program)
            .args(self.arguments)
            .output();
        match output {
            Ok(output) => {
                assert_eq!(self.expected.as_bytes(), output.stdout);
                assert!(output.stderr.is_empty());
            }
            Err(e) => {
                eprintln!("{}", e);
                assert!(false);
            }
        }
    }
}

#[cfg(test)]
fn run_tests(cases: &[TestCase]) {
    for c in cases {
        c.run()
    }
}

#[test]
fn test_apply_basic() {
    run_tests(&[
        TestCase::new("apply", &["-x", "ls", "test-data/goat"], ""),
        TestCase::new("apply", &["-x", "cat -v", "test-data/Goats"], ""),
    ]);
}

#[test]
fn test_files_match_basic() {
    run_tests(&[
        TestCase::new("files", &["-m", "goat", "test-data"], "test-data/goat\n"),
        TestCase::new(
            "files",
            &["-m", "(?i)goat", "test-data"],
            "test-data/Goats
test-data/goat
",
        ),
        TestCase::new(
            "files",
            &["-m", "(?i)goats", "test-data"],
            "test-data/Goats\n",
        ),
        TestCase::new(
            "files",
            &["-m", "p/y", "test-data"],
            "test-data/lurp/norp/yibb\n",
        ),
    ]);
}

#[test]
fn test_files_prune_basic() {
    // TODO BUG: These tests assume the OS orders the files the same way,
    // which of course is not guaranteed.
    run_tests(&[
        TestCase::new(
            "files",
            &["-p", "(?i)goat", "test-data"],
            "test-data
test-data/columns.txt
test-data/farm-animals.txt
test-data/lurp
test-data/lurp/norp
test-data/lurp/norp/yibb
",
        ),
        TestCase::new(
            "files",
            &["-p", "(?i)(goat|yibb)", "test-data"],
            "test-data
test-data/columns.txt
test-data/farm-animals.txt
test-data/lurp
test-data/lurp/norp
",
        ),
    ]);
}

#[test]
fn test_fields_basic() {
    run_tests(&[
        TestCase::new(
            "fields",
            &["-c0", "test-data/columns.txt"],
            "yeah
whee
",
        ),
        TestCase::new(
            "fields",
            &["-c0", "-c2", "-n", "test-data/columns.txt"],
            "1	yeah	hey
2	whee	ouch
",
        ),
        TestCase::new(
            "fields",
            &["-RX", "-FY", "test-data/columns.txt"],
            "yeahYwowYheyYfriendsXwheeYbonkYouchYboingX",
        ),
        TestCase::new(
            "fields",
            &["-I", "-c1", "test-data/columns.txt"],
            "yeah	hey	friends
whee	ouch	boing
",
        ),
        TestCase::new(
            "fields",
            &["-c-1", "-c-2", "test-data/columns.txt"],
            "friends	hey
boing	ouch
",
        ),
        TestCase::new(
            "fields",
            &["-I", "-c-1", "-c-2", "-n", "test-data/columns.txt"],
            "1	yeah	wow
2	whee	bonk
",
        ),
        TestCase::new(
            "fields",
            &["-c-1", "-c-2", "-n", "test-data/columns.txt"],
            "1	friends	hey
2	boing	ouch
",
        ),
    ]);
}
