use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ErrorMatch {
    matches: Vec<(&'static str, Option<String>)>,
}

impl ErrorMatch {
    pub fn new(matches: Vec<(&'static str, Option<String>)>) -> ErrorMatch {
        ErrorMatch { matches }
    }
}

impl fmt::Display for ErrorMatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.matches)
    }
}

impl Error for ErrorMatch {
    fn description(&self) -> &str {
        "Couldn't find a title."
    }
}
