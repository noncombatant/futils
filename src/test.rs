#[cfg(test)]
mod tests {
    use std::process::Command;

    const FUTILS: &str = "target/debug/futils";

    struct TestCase<'a> {
        program: &'a str,
        arguments: &'a [&'a str],
        expected: &'a str,
    }

    fn new<'a>(program: &'a str, arguments: &'a [&'a str], expected: &'a str) -> TestCase<'a> {
        TestCase {
            program,
            arguments,
            expected,
        }
    }

    fn run_test(e: &TestCase) {
        let output = Command::new(FUTILS)
            .arg(e.program)
            .args(e.arguments)
            .output();
        match output {
            Ok(output) => {
                assert_eq!(e.expected.as_bytes(), output.stdout);
                assert!(output.stderr.is_empty());
            }
            Err(e) => {
                eprintln!("{}", e);
                assert!(false);
            }
        }
    }

    fn run_tests(es: &[TestCase]) {
        for e in es {
            run_test(&e);
        }
    }

    #[test]
    fn test_apply_basic() {
        run_tests(&[
            new("apply", &["-x", "ls", "test-data/goat"], ""),
            new("apply", &["-x", "cat -v", "test-data/Goats"], ""),
        ]);
    }

    #[test]
    fn test_files_match_basic() {
        run_tests(&[
            new("files", &["-m", "goat", "test-data"], "test-data/goat\n"),
            new(
                "files",
                &["-m", "(?i)goat", "test-data"],
                "test-data/Goats
test-data/goat
",
            ),
            new(
                "files",
                &["-m", "(?i)goats", "test-data"],
                "test-data/Goats\n",
            ),
            new(
                "files",
                &["-m", "p/y", "test-data"],
                "test-data/lurp/norp/yibb\n",
            ),
        ]);
    }

    #[test]
    fn test_files_prune_basic() {
        run_tests(&[
            new(
                "files",
                &["-p", "(?i)goat", "test-data"],
                "test-data
test-data/columns.txt
test-data/lurp
test-data/lurp/norp
test-data/lurp/norp/yibb
",
            ),
            new(
                "files",
                &["-p", "(?i)(goat|yibb)", "test-data"],
                "test-data
test-data/columns.txt
test-data/lurp
test-data/lurp/norp
",
            ),
        ]);
    }

    #[test]
    fn test_fields_basic() {
        run_tests(&[
            new(
                "fields",
                &["-f1", "test-data/columns.txt"],
                "yeah
whee
",
            ),
            new(
                "fields",
                &["-f1", "-f3", "-n", "test-data/columns.txt"],
                "1	yeah	hey
2	whee	ouch
",
            ),
            new(
                "fields",
                &["-oX", "-OY", "test-data/columns.txt"],
                "yeahYwowYheyYfriendsXwheeYbonkYouchYboingX",
            ),
            new(
                "fields",
                &["-F", "-f2", "test-data/columns.txt"],
                "yeah	hey	friends
whee	ouch	boing
",
            ),
        ]);
    }
}
