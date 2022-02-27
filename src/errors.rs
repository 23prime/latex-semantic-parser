use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseFormulaError;

impl fmt::Display for ParseFormulaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse a formula")
    }
}

impl error::Error for ParseFormulaError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
