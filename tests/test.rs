use std::process::Command;

#[cfg(test)]
const FUTILS: &str = "target/debug/futils";

#[cfg(test)]
struct TestCase<'a> {
    program: &'a str,
    arguments: &'a [&'a str],
    expected: &'a str,
    expected_status: i32,
}

#[cfg(test)]
impl<'a> TestCase<'a> {
    fn new(
        program: &'a str,
        arguments: &'a [&'a str],
        expected: &'a str,
        expected_status: i32,
    ) -> Self {
        Self {
            program,
            arguments,
            expected,
            expected_status,
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
                assert_eq!(self.expected_status, output.status.code().unwrap());
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
        TestCase::new("apply", &["-x", "ls", "test-data/goat"], "", 0),
        TestCase::new("apply", &["-x", "cat -v", "test-data/Goats"], "", 0),
    ]);
}

#[test]
fn test_files_match_basic() {
    run_tests(&[
        TestCase::new("files", &["-m", "goat", "test-data"], "test-data/goat\n", 0),
        TestCase::new(
            "files",
            &["-m", "(?i)goat", "test-data"],
            "test-data/Goats
test-data/goat
",
            0,
        ),
        TestCase::new(
            "files",
            &["-m", "(?i)goats", "test-data"],
            "test-data/Goats\n",
            0,
        ),
        TestCase::new(
            "files",
            &["-m", "p/y", "test-data"],
            "test-data/lurp/norp/yibb\n",
            0,
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
            0,
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
            0,
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
            0,
        ),
        TestCase::new(
            "fields",
            &["-c0", "-c2", "-n", "test-data/columns.txt"],
            "1	yeah	hey
2	whee	ouch
",
            0,
        ),
        TestCase::new(
            "fields",
            &["-RX", "-FY", "test-data/columns.txt"],
            "yeahYwowYheyYfriendsXwheeYbonkYouchYboingX",
            0,
        ),
        TestCase::new(
            "fields",
            &["-I", "-c1", "test-data/columns.txt"],
            "yeah	hey	friends
whee	ouch	boing
",
            0,
        ),
        TestCase::new(
            "fields",
            &["-c-1", "-c-2", "test-data/columns.txt"],
            "friends	hey
boing	ouch
",
            0,
        ),
        TestCase::new(
            "fields",
            &["-I", "-c-1", "-c-2", "-n", "test-data/columns.txt"],
            "1	yeah	wow
2	whee	bonk
",
            0,
        ),
        TestCase::new(
            "fields",
            &["-c-1", "-c-2", "-n", "test-data/columns.txt"],
            "1	friends	hey
2	boing	ouch
",
            0,
        ),
    ]);
}

#[test]
fn test_filter_basic() {
    run_tests(&[
        TestCase::new(
            "filter",
            &["-m", "(?i)goat", "test-data/farm-animals.txt"],
            "test-data/farm-animals.txt	1	mountain goat	grass, moss, vegetation
test-data/farm-animals.txt	4	billy goats	grass, moss, vegetation, tin cans
",
            0,
        ),
        TestCase::new(
            "filter",
            &["-n", "-m", "(?i)goat", "test-data/farm-animals.txt"],
            "test-data/farm-animals.txt	1	1	mountain goat	grass, moss, vegetation
test-data/farm-animals.txt	2	4	billy goats	grass, moss, vegetation, tin cans
",
            0,
        ),
        TestCase::new(
            "filter",
            &["-n", "-p", "(?i)goat", "test-data/farm-animals.txt"],
            "test-data/farm-animals.txt	3	12	sheep	grass, more grass
test-data/farm-animals.txt	4	1,749	llamas	exclusively human flesh (for some reason)
",
            0,
        ),
        TestCase::new(
            "filter",
            &["-m", "GOAT", "test-data/farm-animals.txt"],
            "",
            1,
        ),
    ]);
}
