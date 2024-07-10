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

#[cfg(test)]
impl<'a> TestCase<'a> {
    fn new(
        name: &'a str,
        program: &'a str,
        arguments: &'a [&'a str],
        expected: &'a str,
        sorted: bool,
        expected_status: i32,
    ) -> Self {
        Self {
            name,
            program,
            arguments,
            expected,
            sorted,
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
                if self.sorted {
                    let mut lines = from_utf8(&output.stdout)
                        .unwrap()
                        .split('\n')
                        .filter(|l| !l.is_empty())
                        .collect::<Vec<&str>>();
                    lines.sort();
                    let output = lines.join("\n");
                    assert_eq!(self.expected, output);
                } else {
                    println!("{:#?}", self.arguments);
                    assert_eq!(self.expected, from_utf8(&output.stdout).unwrap());
                }
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
        TestCase::new(
            "files match basic simple",
            "files",
            &["-m", "goat", "test-data"],
            "test-data/goat\n",
            false,
            0,
        ),
        TestCase::new(
            "files match basic simple case-insensitive",
            "files",
            &["-m", "(?i)goat", "test-data"],
            "test-data/Goats
test-data/goat",
            true,
            0,
        ),
        TestCase::new(
            "files match basic simple case-insensitive 2",
            "files",
            &["-m", "(?i)goats", "test-data"],
            "test-data/Goats",
            true,
            0,
        ),
        TestCase::new(
            "files match multiple path parts",
            "files",
            &["-m", "p/y", "test-data"],
            "test-data/lurp/norp/yibb",
            true,
            0,
        ),
        TestCase::new(
            "files match depth 1",
            "files",
            &["-d", "1", "test-data"],
            "test-data
test-data/Goats
test-data/columns.txt
test-data/common1.txt
test-data/common2.txt
test-data/farm-animals.txt
test-data/goat
test-data/lurp
test-data/numbers.txt",
            true,
            0,
        ),
    ]);
}

#[test]
fn test_files_prune_basic() {
    run_tests(&[
        TestCase::new(
            "files prune basic simple case-insensitive",
            "files",
            &["-p", "(?i)goat", "test-data"],
            "test-data
test-data/columns.txt
test-data/common1.txt
test-data/common2.txt
test-data/farm-animals.txt
test-data/lurp
test-data/lurp/norp
test-data/lurp/norp/yibb
test-data/numbers.txt",
            true,
            0,
        ),
        TestCase::new(
            "files prune basic simple case-insensitive -i",
            "files",
            &["-i", "-p", "goat", "test-data"],
            "test-data
test-data/columns.txt
test-data/common1.txt
test-data/common2.txt
test-data/farm-animals.txt
test-data/lurp
test-data/lurp/norp
test-data/lurp/norp/yibb
test-data/numbers.txt",
            true,
            0,
        ),
        TestCase::new(
            "files prune basic simple case-insensitive alternation",
            "files",
            &["-p", "(?i)(goat|yibb)", "test-data"],
            "test-data
test-data/columns.txt
test-data/common1.txt
test-data/common2.txt
test-data/farm-animals.txt
test-data/lurp
test-data/lurp/norp
test-data/numbers.txt",
            true,
            0,
        ),
        TestCase::new(
            "files prune complex",
            "files",
            &[
                "-p",
                "(?i)(goat|yibb)",
                "-m",
                "co",
                "-i",
                "-p",
                "MON",
                "test-data",
            ],
            "test-data/columns.txt",
            true,
            0,
        ),
    ]);
}

#[test]
fn test_fields_basic() {
    run_tests(&[
        TestCase::new(
            "fields column 0",
            "fields",
            &["-c0", "test-data/columns.txt"],
            "test-data/columns.txt	    1	yeah
test-data/columns.txt	    2	whee
",
            false,
            0,
        ),
        TestCase::new(
            "fields 2 columns, non-enumerated",
            "fields",
            &["-c0", "-c2", "-n", "test-data/columns.txt"],
            "yeah	hey
whee	ouch
",
            false,
            0,
        ),
        TestCase::new(
            "fields custom output delimiters",
            "fields",
            &["-RX", "-FY", "-n", "test-data/columns.txt"],
            "yeahYwowYheyYfriendsXwheeYbonkYouchYboingX",
            false,
            0,
        ),
        TestCase::new(
            "fields all but column 2",
            "fields",
            &["-I", "-c1", "test-data/columns.txt"],
            "test-data/columns.txt	    1	yeah	hey	friends
test-data/columns.txt	    2	whee	ouch	boing
",
            false,
            0,
        ),
        TestCase::new(
            "fields negative columns, non-enumerated",
            "fields",
            &["-c-1", "-c-2", "-n", "test-data/columns.txt"],
            "friends	hey
boing	ouch
",
            false,
            0,
        ),
        TestCase::new(
            "fields inverted negative columns",
            "fields",
            &["-I", "-c-1", "-c-2", "test-data/columns.txt"],
            "test-data/columns.txt	    1	yeah	wow
test-data/columns.txt	    2	whee	bonk
",
            false,
            0,
        ),
        TestCase::new(
            "fields negative columns",
            "fields",
            &["-c-1", "-c-2", "test-data/columns.txt"],
            "test-data/columns.txt	    1	friends	hey
test-data/columns.txt	    2	boing	ouch
",
            false,
            0,
        ),
    ]);
}

#[test]
fn test_filter_basic() {
    run_tests(&[
        TestCase::new(
            "filter non-enumerated case-insensitive combined options",
            "filter",
            &["-nm", "(?i)goat", "test-data/farm-animals.txt"],
            "1	mountain goat	grass, moss, vegetation
4	billy goats	grass, moss, vegetation, tin cans
",
            false,
            0,
        ),
        TestCase::new(
            "filter case-insensitive -i",
            "filter",
            &["-i", "-m", "goat", "test-data/farm-animals.txt"],
            "test-data/farm-animals.txt	    1	1	mountain goat	grass, moss, vegetation
test-data/farm-animals.txt	    2	4	billy goats	grass, moss, vegetation, tin cans
",
            false,
            0,
        ),
        TestCase::new(
            "filter non-enumerated case-insensitive",
            "filter",
            &["-n", "-m", "(?i)goat", "test-data/farm-animals.txt"],
            "1	mountain goat	grass, moss, vegetation
4	billy goats	grass, moss, vegetation, tin cans
",
            false,
            0,
        ),
        TestCase::new(
            "filter prune case-insensitive",
            "filter",
            &["-p", "(?i)goat", "test-data/farm-animals.txt"],
            "test-data/farm-animals.txt	    3	12	sheep	grass, more grass
test-data/farm-animals.txt	    4	1,749	llamas	exclusively human flesh (for some reason)
",
            false,
            0,
        ),
        TestCase::new(
            "filter non-enumerated prune case-insensitive",
            "filter",
            &["-n", "-i", "-p", "goat", "test-data/farm-animals.txt"],
            "12	sheep	grass, more grass
1,749	llamas	exclusively human flesh (for some reason)
",
            false,
            0,
        ),
        TestCase::new(
            "filter match capitals",
            "filter",
            &["-m", "GOAT", "test-data/farm-animals.txt"],
            "",
            false,
            1,
        ),
        TestCase::new(
            "filter match capitals, meaningless -i",
            "filter",
            &["-m", "GOAT", "-i", "test-data/farm-animals.txt"],
            "",
            false,
            1,
        ),
    ]);
}

#[test]
fn test_filter_limit0() {
    run_tests(&[
        TestCase::new(
            "filter limit 0 match",
            "filter",
            &["-l", "0", "-m", "chunk", "test-data/farm-animals.txt"],
            "",
            false,
            1,
        ),
        TestCase::new(
            "filter limit 0 prune",
            "filter",
            &["-l", "0", "-p", "chunk", "test-data/farm-animals.txt"],
            "",
            false,
            0,
        ),
        TestCase::new(
            "filter limit case-insensitive",
            "filter",
            &["-l", "0", "-m", "(?i)goat", "test-data/farm-animals.txt"],
            "",
            false,
            0,
        ),
    ]);
}

#[test]
fn test_records_basic() {
    run_tests(&[
        TestCase::new(
            "records non-enumerated",
            "records",
            &["-n", "test-data/farm-animals.txt"],
            "1	mountain goat	grass, moss, vegetation
4	billy goats	grass, moss, vegetation, tin cans
12	sheep	grass, more grass
1,749	llamas	exclusively human flesh (for some reason)
",
            false,
            0,
        ),
        TestCase::new(
            "records enumerated",
            "records",
            &["test-data/farm-animals.txt"],
            "test-data/farm-animals.txt	    1	1	mountain goat	grass, moss, vegetation
test-data/farm-animals.txt	    2	4	billy goats	grass, moss, vegetation, tin cans
test-data/farm-animals.txt	    3	12	sheep	grass, more grass
test-data/farm-animals.txt	    4	1,749	llamas	exclusively human flesh (for some reason)
",
            false,
            0,
        ),
        TestCase::new(
            "records limit",
            "records",
            &["-l", "2", "test-data/farm-animals.txt"],
            "test-data/farm-animals.txt	    1	1	mountain goat	grass, moss, vegetation
test-data/farm-animals.txt	    2	4	billy goats	grass, moss, vegetation, tin cans
",
            false,
            0,
        ),
    ]);
}

#[test]
fn test_reduce_basic() {
    run_tests(&[
        TestCase::new(
            "reduce add",
            "reduce",
            &["-x", "+", "test-data/numbers.txt"],
            "2102784
",
            false,
            0,
        ),
        TestCase::new(
            "reduce subtract",
            "reduce",
            &["-x", "-", "test-data/numbers.txt"],
            "-2100736
",
            false,
            0,
        ),
        TestCase::new(
            "reduce multiply",
            "reduce",
            &["-x", "*", "test-data/numbers.txt"],
            "2361183241434822606848
",
            false,
            0,
        ),
        TestCase::new(
            "reduce divide",
            "reduce",
            &["-x", "/", "test-data/numbers.txt"],
            "4.44089209850062616169452667236328125E-16
",
            false,
            0,
        ),
    ]);
}

#[test]
fn test_common_basic() {
    run_tests(&[
        TestCase::new(
            "common output field separator",
            "common",
            &[
                "-F",
                ",\\t",
                "test-data/common1.txt",
                "test-data/common2.txt",
            ],
            ",	,	Atlanta
,	,	Boston
Cincinnati
,	cincinnati
,	Detroit
",
            false,
            0,
        ),
        TestCase::new(
            "common output field separator meaningless -i",
            "common",
            &[
                "-F",
                ",\\t",
                "-i",
                "test-data/common1.txt",
                "test-data/common2.txt",
            ],
            ",	,	Atlanta
,	,	Boston
,	,	Cincinnati
,	Detroit
",
            false,
            0,
        ),
    ]);
}
