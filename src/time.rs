use chrono::format::ParseError;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use chrono::{Datelike, Local, Timelike};

/// Given the number of seconds from the Unix epoch in UTC, returns a sortable
/// string representation in the format `%Y-%m-%d %H:%M:%S`. If `utc` cannot be
/// interpreted for some raisin, returns `utc` `format!`ed as a `String`.
pub fn utc_timestamp_to_string(utc: i64) -> String {
    match NaiveDateTime::from_timestamp_opt(utc, 0) {
        Some(naive) => {
            let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
            format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"))
        }
        None => format!("{}", utc),
    }
}

/// Indicates which comparison operation to perform on 2 `NaiveDateTime`s. See
/// `Time`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Comparison {
    Before,
    After,
    Exactly,
}

/// A comparison operation on a `NaiveDateTime`. This is essentially a curried
/// function (or, rather, 1 of 3 curried functions) on `date_time`.
pub struct Time {
    /// A date-time that another date-time will be compared to.
    pub date_time: NaiveDateTime,

    /// What kind of comparison to perform. See `Comparison`.
    pub comparison: Comparison,
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
    pub fn new(string: &str) -> Result<Time, ParseError> {
        let string = string.trim();
        if string.is_empty() {
            return Ok(Time {
                date_time: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
                comparison: Comparison::After,
            });
        }

        let mut operator = Comparison::Exactly;
        let mut string = string;
        let (o, s) = string.split_at(1);
        if ["<", ">", "="].contains(&o) {
            operator = match o {
                "<" => Comparison::Before,
                ">" => Comparison::After,
                _ => Comparison::Exactly,
            };
            string = s.trim();
        }

        let time = Self::from_date_time_string(string, operator);
        match time {
            Ok(time) => Ok(time),
            Err(_) => match Self::from_time_string(string, operator) {
                Ok(time) => Ok(time),
                Err(_) => Self::from_date_string(string, operator),
            },
        }
    }

    fn from_date_time_string(string: &str, operator: Comparison) -> Result<Time, ParseError> {
        let date_time = NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M:%S");
        match date_time {
            Ok(dt) => Ok(Time {
                date_time: dt,
                comparison: operator,
            }),
            Err(e) => Err(e),
        }
    }

    fn from_time_string(string: &str, operator: Comparison) -> Result<Time, ParseError> {
        let now = Local::now();
        let time = NaiveTime::parse_from_str(string, "%H:%M:%S");
        match time {
            Ok(time) => {
                let date_time = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
                    .unwrap()
                    .and_hms_opt(time.hour(), time.minute(), time.second())
                    .unwrap();
                Ok(Time {
                    date_time,
                    comparison: operator,
                })
            }
            Err(e) => Err(e),
        }
    }

    fn from_date_string(string: &str, operator: Comparison) -> Result<Time, ParseError> {
        let date = NaiveDate::parse_from_str(string, "%Y-%m-%d");
        match date {
            Ok(date) => {
                let date_time = NaiveDate::from_ymd_opt(date.year(), date.month(), date.day())
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap();
                Ok(Time {
                    date_time,
                    comparison: operator,
                })
            }
            Err(e) => Err(e),
        }
    }
}

#[test]
fn parse_time() {
    let t = Time::new(">2023-01-27 12:34:56").unwrap();
    assert_eq!(Comparison::After, t.comparison);
    assert_eq!(2023, t.date_time.year());
    assert_eq!(1, t.date_time.month());
    assert_eq!(27, t.date_time.day());
    assert_eq!(12, t.date_time.hour());
    assert_eq!(34, t.date_time.minute());
    assert_eq!(56, t.date_time.second());

    let t = Time::new("<2023-01-27").unwrap();
    assert_eq!(Comparison::Before, t.comparison);
    assert_eq!(2023, t.date_time.year());
    assert_eq!(1, t.date_time.month());
    assert_eq!(27, t.date_time.day());
    assert_eq!(0, t.date_time.hour());
    assert_eq!(0, t.date_time.minute());
    assert_eq!(0, t.date_time.second());

    let t = Time::new("=12:34:56").unwrap();
    let now = Local::now();
    assert_eq!(Comparison::Exactly, t.comparison);
    assert_eq!(now.year(), t.date_time.year());
    assert_eq!(now.month(), t.date_time.month());
    assert_eq!(now.day(), t.date_time.day());
    assert_eq!(12, t.date_time.hour());
    assert_eq!(34, t.date_time.minute());
    assert_eq!(56, t.date_time.second());

    let t = Time::new(" =\n\n12:34:56 ").unwrap();
    let now = Local::now();
    assert_eq!(Comparison::Exactly, t.comparison);
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
