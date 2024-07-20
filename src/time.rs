// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! A not-very-good way to represent and compare times provided as text strings
//! on the command line.

use std::cmp::Ordering;

use chrono::format::ParseError;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::{Datelike, Local, Timelike};

use crate::shell::UsageError;

/// Given the number of seconds from the Unix epoch in UTC, returns a sortable
/// string representation in the format `%Y-%m-%d %H:%M:%S`. If `utc` cannot be
/// interpreted for some raisin, returns `utc` `format!`ed as a `String`.
pub fn format_utc_timestamp(utc: i64) -> String {
    DateTime::from_timestamp(utc, 0).map_or(format!("{utc}"), |datetime| {
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    })
}

/// A comparison operation on a `NaiveDateTime`. This is essentially a curried
/// function (or, rather, 1 of 3 curried functions) on `date_time`.
pub struct Time {
    /// A date-time that another date-time will be compared to.
    pub date_time: NaiveDateTime,

    /// What kind of comparison to perform.
    pub ordering: Ordering,
}

impl Time {
    /// Parses `string`, which is parsed as having come from a grammar not
    /// entirely unlike:
    ///
    ///     s ::= space* <operator> space* <datetime> space*
    ///     operator ::= "<" | ">" | "="
    ///     datetime ::= <date> " " <time>
    ///         | <time>
    ///         | <date>
    ///     date ::= digit{4} "-" digit{2} "-" digit{2}
    ///     time ::= digit{2} ":" digit{2} ":" digit{2}
    ///
    /// and returns a `Time`.
    ///
    /// This function also accepts the empty string as a special case, in which
    /// case it returns a `Time` indicating 0 in the Unix epoch.
    pub fn new(string: &str) -> anyhow::Result<Self> {
        let string = string.trim();
        if string.is_empty() {
            return Ok(Self {
                date_time: DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
                ordering: Ordering::Greater,
            });
        }

        let (operator, string) = string.split_at(1);
        let operator = match operator {
            "<" => Ordering::Less,
            "=" => Ordering::Equal,
            ">" => Ordering::Greater,
            _ => return Err(UsageError::new("Invalid datetime expression").into()),
        };
        let string = string.trim();

        let time = Self::from_date_time_string(string, operator);
        match time {
            Ok(time) => Ok(time),
            Err(_) => match Self::from_time_string(string, operator) {
                Ok(time) => Ok(time),
                Err(_) => Ok(Self::from_date_string(string, operator)?),
            },
        }
    }

    fn from_date_time_string(string: &str, operator: Ordering) -> Result<Self, ParseError> {
        Ok(Self {
            date_time: NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M:%S")?,
            ordering: operator,
        })
    }

    fn from_time_string(string: &str, operator: Ordering) -> Result<Self, ParseError> {
        let now = Local::now();
        let time = NaiveTime::parse_from_str(string, "%H:%M:%S");
        match time {
            Ok(time) => {
                let date_time = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
                    .unwrap()
                    .and_hms_opt(time.hour(), time.minute(), time.second())
                    .unwrap();
                Ok(Self {
                    date_time,
                    ordering: operator,
                })
            }
            Err(error) => Err(error),
        }
    }

    fn from_date_string(string: &str, operator: Ordering) -> Result<Self, ParseError> {
        let date = NaiveDate::parse_from_str(string, "%Y-%m-%d");
        match date {
            Ok(date) => {
                let date_time = NaiveDate::from_ymd_opt(date.year(), date.month(), date.day())
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap();
                Ok(Self {
                    date_time,
                    ordering: operator,
                })
            }
            Err(e) => Err(e),
        }
    }
}

#[test]
fn parse_time() {
    let t = Time::new(">2023-01-27 12:34:56").unwrap();
    assert_eq!(Ordering::Greater, t.ordering);
    assert_eq!(2023, t.date_time.year());
    assert_eq!(1, t.date_time.month());
    assert_eq!(27, t.date_time.day());
    assert_eq!(12, t.date_time.hour());
    assert_eq!(34, t.date_time.minute());
    assert_eq!(56, t.date_time.second());

    let t = Time::new("<2023-01-27").unwrap();
    assert_eq!(Ordering::Less, t.ordering);
    assert_eq!(2023, t.date_time.year());
    assert_eq!(1, t.date_time.month());
    assert_eq!(27, t.date_time.day());
    assert_eq!(0, t.date_time.hour());
    assert_eq!(0, t.date_time.minute());
    assert_eq!(0, t.date_time.second());

    let t = Time::new("=12:34:56").unwrap();
    let now = Local::now();
    assert_eq!(Ordering::Equal, t.ordering);
    assert_eq!(now.year(), t.date_time.year());
    assert_eq!(now.month(), t.date_time.month());
    assert_eq!(now.day(), t.date_time.day());
    assert_eq!(12, t.date_time.hour());
    assert_eq!(34, t.date_time.minute());
    assert_eq!(56, t.date_time.second());

    let t = Time::new(" =\n\n12:34:56 ").unwrap();
    let now = Local::now();
    assert_eq!(Ordering::Equal, t.ordering);
    assert_eq!(now.year(), t.date_time.year());
    assert_eq!(now.month(), t.date_time.month());
    assert_eq!(now.day(), t.date_time.day());
    assert_eq!(12, t.date_time.hour());
    assert_eq!(34, t.date_time.minute());
    assert_eq!(56, t.date_time.second());

    assert!(Time::new("charbity bimborfs?").is_err());
    assert!(Time::new("<charbity bimborfs?").is_err());
    assert!(Time::new("=1, 2, 3, 4, I declare a thumb war").is_err());
}
