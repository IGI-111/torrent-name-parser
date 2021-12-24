use crate::pattern::PatternName;
use regex::Match;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ErrorMatch<'t> {
    matches: Vec<(PatternName, Option<Match<'t>>)>,
}

impl<'t> ErrorMatch<'t> {
    pub fn new(matches: Vec<(PatternName, Option<Match<'t>>)>) -> ErrorMatch<'t> {
        ErrorMatch { matches }
    }
}

impl fmt::Display for ErrorMatch<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.matches)
    }
}

impl Error for ErrorMatch<'_> {
    fn description(&self) -> &str {
        "Couldn't find a title."
    }
}
