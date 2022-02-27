use std::str::FromStr;

use crate::errors::ParseFormulaError;

#[derive(Debug, Clone, PartialEq)]
pub enum Formula {
    TerminalSymbol(String),
}

impl FromStr for Formula {
    type Err = ParseFormulaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Formula::TerminalSymbol(s.to_string()));
    }
}
