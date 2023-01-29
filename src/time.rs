use chrono::format::ParseError;
use chrono::{Datelike, Local, Timelike};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Before,
    After,
    Exactly,
}

pub struct Time {
    pub date_time: NaiveDateTime,
    pub comparison: Comparison,
}

impl Time {
    pub fn new(string: &str) -> Result<Time, ParseError> {
        if string.is_empty() {
            // TODO: Return an error
            //return None
        }
        let (operator, string) = string.split_at(1);
        let operator = match operator {
            "<" => Comparison::Before,
            ">" => Comparison::After,
            "=" => Comparison::Exactly,
            // TODO: Return an error
            _ => Comparison::Exactly,
        };

        let date_time = NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M:%S");
        match date_time {
            Ok(dt) => {
                return Ok(Time {
                    date_time: dt,
                    comparison: operator,
                })
            }
            Err(_) => {
                let now = Local::now();
                let time = NaiveTime::parse_from_str(string, "%H:%M:%S");
                match time {
                    Ok(time) => {
                        let date_time = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
                            .unwrap()
                            .and_hms_opt(time.hour(), time.minute(), time.second())
                            .unwrap();
                        return Ok(Time {
                            date_time: date_time,
                            comparison: operator,
                        });
                    }
                    Err(_) => {
                        let date = NaiveDate::parse_from_str(string, "%Y-%m-%d");
                        match date {
                            Ok(date) => {
                                let date_time =
                                    NaiveDate::from_ymd_opt(date.year(), date.month(), date.day())
                                        .unwrap()
                                        .and_hms_opt(0, 0, 0)
                                        .unwrap();
                                return Ok(Time {
                                    date_time: date_time,
                                    comparison: operator,
                                });
                            }
                            Err(e) => Err(e),
                        }
                    }
                }
            }
        }
    }
}


#[cfg(test)]
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
}

