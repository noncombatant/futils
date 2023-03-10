#[cfg(test)]
mod tests {
    use std::process::Command;

    const FUTILS: &str = "target/debug/futils";

    struct Expectation<'a> {
        program: &'a str,
        arguments: &'a [&'a str],
        expected: &'a str,
    }

    fn run_test(e: &Expectation) {
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

    fn run_tests(es: &[Expectation]) {
        for e in es {
            run_test(&e);
        }
    }

    #[test]
    fn test_files_match_basic() {
        let expectations = [
            Expectation {
                program: "files",
                arguments: &["-m", "goat"],
                expected: "./test/goat\n",
            },
            Expectation {
                program: "files",
                arguments: &["-m", "(?i)goat"],
                expected: "./test/Goats
./test/goat
",
            },
            Expectation {
                program: "files",
                arguments: &["-m", "(?i)goats"],
                expected: "./test/Goats\n",
            },
            Expectation {
                program: "files",
                arguments: &["-m", "p/y"],
                expected: "./test/lurp/norp/yibb\n",
            },
        ];
        run_tests(&expectations);
    }

    #[test]
    fn test_fields_basic() {
        let expectations = [
            Expectation {
                program: "fields",
                arguments: &["-f1", "test/columns.txt"],
                expected: "yeah
whee
",
            },
            Expectation {
                program: "fields",
                arguments: &["-f1", "-f3", "-n", "test/columns.txt"],
                expected: "1	yeah	hey
2	whee	ouch
",
            },
        ];
        run_tests(&expectations);
    }
}
