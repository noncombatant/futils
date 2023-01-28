use regex::bytes::Regex;

use crate::util::run_command;

pub enum Predicate<'a> {
    Nothing,
    MatchCommand(&'a str),
    MatchExpression(&'a Regex),
    PruneExpression(&'a Regex),
}

// TODO: define a global `type Record = &[u8]`.

impl<'a> Predicate<'a> {
    pub fn evaluate(&self, record: &[u8]) -> bool {
        match self {
            Predicate::Nothing => panic!("Some goatery has occurred."),
            Predicate::MatchCommand(c) => run_command(c, record),
            Predicate::MatchExpression(e) => e.is_match(record),
            Predicate::PruneExpression(e) => !e.is_match(record),
        }
    }
}
