// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

use std::process::Command;
use std::str::from_utf8;

#[cfg(test)]
const FUTILS: &str = "target/debug/futils";

#[cfg(test)]
struct TestCase<'a> {
    name: &'a str,
    program: &'a str,
    arguments: &'a [&'a str],
    expected: &'a str,
    sorted: bool,
    expected_status: i32,
}

fn sort_lines(v: &str) -> String {
    let mut lines = v
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();
    lines.sort();
    lines.join("\n")
}

#[cfg(test)]
impl<'a> TestCase<'a> {
    fn run(&self) {
        let output = Command::new(FUTILS)
            .arg(self.program)
            .args(self.arguments)
            .output();
        match output {
            Ok(output) => {
                let lines = from_utf8(&output.stdout).unwrap();
                let lines = if self.sorted {
                    &sort_lines(lines)
                } else {
                    lines
                };
                if self.expected != lines {
                    eprintln!("{}", self.name);
                }
                assert_eq!(self.expected, lines);
                assert!(output.stderr.is_empty());
                assert_eq!(self.expected_status, output.status.code().unwrap());
            }
            Err(e) => {
                eprintln!("{} {}", self.name, e);
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
fn test_files_match_basic() {
    run_tests(&[
        TestCase {
            name: "files match basic simple case-sensitive",
            program: "files",
            arguments: &["-S", "-m", "goat", "test-data"],
            expected: "test-data/goat\n",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "files match basic simple case-insensitive",
            program: "files",
            arguments: &["-m", "goat", "test-data"],
            expected: "test-data/Goats
test-data/goat",
            sorted: true,
            expected_status: 0,
        },
        TestCase {
            name: "files match basic simple case-insensitive 2",
            program: "files",
            arguments: &["-m", "(?i)goats", "test-data"],
            expected: "test-data/Goats",
            sorted: true,
            expected_status: 0,
        },
        TestCase {
            name: "files match multiple path parts",
            program: "files",
            arguments: &["-m", "p/y", "test-data"],
            expected: "test-data/lurp/norp/yibb",
            sorted: true,
            expected_status: 0,
        },
        TestCase {
            name: "files match depth 1",
            program: "files",
            arguments: &["-d", "1", "test-data"],
            expected: "test-data
test-data/Goats
test-data/columns.txt
test-data/common1.txt
test-data/common2.txt
test-data/farm-animals.txt
test-data/goat
test-data/lurp
test-data/numbers.txt",
            sorted: true,
            expected_status: 0,
        },
    ]);
}

#[test]
fn test_files_prune_basic() {
    run_tests(&[
        TestCase {
            name: "files prune basic simple case-insensitive",
            program: "files",
            arguments: &["-p", "goat", "test-data"],
            expected: "test-data
test-data/columns.txt
test-data/common1.txt
test-data/common2.txt
test-data/farm-animals.txt
test-data/lurp
test-data/lurp/norp
test-data/lurp/norp/yibb
test-data/numbers.txt",
            sorted: true,
            expected_status: 0,
        },
        TestCase {
            name: "files prune basic simple case-sensitive",
            program: "files",
            arguments: &["-S", "-p", "goat", "test-data"],
            expected: "test-data
test-data/Goats
test-data/columns.txt
test-data/common1.txt
test-data/common2.txt
test-data/farm-animals.txt
test-data/lurp
test-data/lurp/norp
test-data/lurp/norp/yibb
test-data/numbers.txt",
            sorted: true,
            expected_status: 0,
        },
        TestCase {
            name: "files prune basic simple case-insensitive alternation",
            program: "files",
            arguments: &["-p", "(?i)(goat|yibb)", "test-data"],
            expected: "test-data
test-data/columns.txt
test-data/common1.txt
test-data/common2.txt
test-data/farm-animals.txt
test-data/lurp
test-data/lurp/norp
test-data/numbers.txt",
            sorted: true,
            expected_status: 0,
        },
        TestCase {
            name: "files prune complex",
            program: "files",
            arguments: &[
                "-p",
                "(?i)(goat|yibb)",
                "-m",
                "co",
                "-p",
                "mon",
                "test-data",
            ],
            expected: "test-data/columns.txt",
            sorted: true,
            expected_status: 0,
        },
    ]);
}

#[test]
fn test_fields_basic() {
    run_tests(&[
        TestCase {
            name: "fields column 0",
            program: "fields",
            arguments: &["-c0", "test-data/columns.txt"],
            expected: "test-data/columns.txt	    1	yeah
test-data/columns.txt	    2	whee
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "fields 2 columns, non-enumerated",
            program: "fields",
            arguments: &["-c0", "-c2", "-n", "test-data/columns.txt"],
            expected: "yeah	hey
whee	ouch
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "fields custom output delimiters",
            program: "fields",
            arguments: &["-RX", "-FY", "-n", "test-data/columns.txt"],
            expected: "yeahYwowYheyYfriendsXwheeYbonkYouchYboingX",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "fields all but column 2",
            program: "fields",
            arguments: &["-I", "-c1", "test-data/columns.txt"],
            expected: "test-data/columns.txt	    1	yeah	hey	friends
test-data/columns.txt	    2	whee	ouch	boing
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "fields negative columns, non-enumerated",
            program: "fields",
            arguments: &["-c-1", "-c-2", "-n", "test-data/columns.txt"],
            expected: "friends	hey
boing	ouch
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "fields inverted negative columns",
            program: "fields",
            arguments: &["-I", "-c-1", "-c-2", "test-data/columns.txt"],
            expected: "test-data/columns.txt	    1	yeah	wow
test-data/columns.txt	    2	whee	bonk
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "fields negative columns",
            program: "fields",
            arguments: &["-c-1", "-c-2", "test-data/columns.txt"],
            expected: "test-data/columns.txt	    1	friends	hey
test-data/columns.txt	    2	boing	ouch
",
            sorted: false,
            expected_status: 0,
        },
    ]);
}

#[test]
fn test_filter_basic() {
    run_tests(&[
        TestCase {
            name: "filter non-enumerated case-insensitive combined options",
            program: "filter",
            arguments: &["-nm", "(?i)goat", "test-data/farm-animals.txt"],
            expected: "1	mountain goat	grass, moss, vegetation
4	billy goats	grass, moss, vegetation, tin cans
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "filter case-insensitive",
            program: "filter",
            arguments: &["-m", "goat", "test-data/farm-animals.txt"],
            expected: "test-data/farm-animals.txt	    1	1	mountain goat	grass, moss, vegetation
test-data/farm-animals.txt	    2	4	billy goats	grass, moss, vegetation, tin cans
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "filter non-enumerated case-insensitive",
            program: "filter",
            arguments: &["-n", "-m", "goat", "test-data/farm-animals.txt"],
            expected: "1	mountain goat	grass, moss, vegetation
4	billy goats	grass, moss, vegetation, tin cans
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "filter prune case-insensitive",
            program: "filter",
            arguments: &["-p", "goat", "test-data/farm-animals.txt"],
            expected: "test-data/farm-animals.txt	    3	12	sheep	grass, more grass
test-data/farm-animals.txt	    4	1,749	llamas	exclusively human flesh (for some reason)
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "filter non-enumerated prune case-insensitive",
            program: "filter",
            arguments: &["-n", "-p", "goat", "test-data/farm-animals.txt"],
            expected: "12	sheep	grass, more grass
1,749	llamas	exclusively human flesh (for some reason)
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "filter match capitals",
            program: "filter",
            arguments: &["-S", "-m", "GOAT", "test-data/farm-animals.txt"],
            expected: "",
            sorted: false,
            expected_status: 1,
        },
        TestCase {
            name: "filter match capitals, meaningless -i",
            program: "filter",
            arguments: &["-m", "GOAT", "test-data/farm-animals.txt"],
            expected: "",
            sorted: false,
            expected_status: 1,
        },
    ]);
}

#[test]
fn test_filter_limit0() {
    run_tests(&[
        TestCase {
            name: "filter limit 0 match",
            program: "filter",
            arguments: &["-l", "0", "-m", "chunk", "test-data/farm-animals.txt"],
            expected: "",
            sorted: false,
            expected_status: 1,
        },
        TestCase {
            name: "filter limit 0 prune",
            program: "filter",
            arguments: &["-l", "0", "-p", "chunk", "test-data/farm-animals.txt"],
            expected: "",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "filter limit case-insensitive",
            program: "filter",
            arguments: &["-l", "0", "-m", "(?i)goat", "test-data/farm-animals.txt"],
            expected: "",
            sorted: false,
            expected_status: 0,
        },
    ]);
}

#[test]
fn test_records_basic() {
    run_tests(&[
        TestCase {
            name: "records non-enumerated",
            program: "records",
            arguments: &["-n", "test-data/farm-animals.txt"],
            expected: "1	mountain goat	grass, moss, vegetation
4	billy goats	grass, moss, vegetation, tin cans
12	sheep	grass, more grass
1,749	llamas	exclusively human flesh (for some reason)
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "records enumerated",
            program: "records",
            arguments: &["test-data/farm-animals.txt"],
            expected: "test-data/farm-animals.txt	    1	1	mountain goat	grass, moss, vegetation
test-data/farm-animals.txt	    2	4	billy goats	grass, moss, vegetation, tin cans
test-data/farm-animals.txt	    3	12	sheep	grass, more grass
test-data/farm-animals.txt	    4	1,749	llamas	exclusively human flesh (for some reason)
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "records limit",
            program: "records",
            arguments: &["-l", "2", "test-data/farm-animals.txt"],
            expected: "test-data/farm-animals.txt	    1	1	mountain goat	grass, moss, vegetation
test-data/farm-animals.txt	    2	4	billy goats	grass, moss, vegetation, tin cans
",
            sorted: false,
            expected_status: 0,
        },
    ]);
}

#[test]
fn test_reduce_basic() {
    run_tests(&[
        TestCase {
            name: "reduce add",
            program: "reduce",
            arguments: &["-x", "+", "test-data/numbers.txt"],
            expected: "2102784
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "reduce subtract",
            program: "reduce",
            arguments: &["-x", "-", "test-data/numbers.txt"],
            expected: "-2100736
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "reduce multiply",
            program: "reduce",
            arguments: &["-x", "*", "test-data/numbers.txt"],
            expected: "2361183241434822606848
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "reduce divide",
            program: "reduce",
            arguments: &["-x", "/", "test-data/numbers.txt"],
            expected: "4.44089209850062616169452667236328125E-16
",
            sorted: false,
            expected_status: 0,
        },
    ]);
}

#[test]
fn test_common_basic() {
    run_tests(&[
        TestCase {
            name: "common output field separator -S",
            program: "common",
            arguments: &[
                "-F",
                ",\\t",
                "-S",
                "test-data/common1.txt",
                "test-data/common2.txt",
            ],
            expected: ",	,	Atlanta
,	,	Boston
Cincinnati
,	cincinnati
,	Detroit
",
            sorted: false,
            expected_status: 0,
        },
        TestCase {
            name: "common output field separator meaningless",
            program: "common",
            arguments: &[
                "-F",
                ",\\t",
                "test-data/common1.txt",
                "test-data/common2.txt",
            ],
            expected: ",	,	Atlanta
,	,	Boston
,	,	Cincinnati
,	Detroit
",
            sorted: false,
            expected_status: 0,
        },
    ]);
}
