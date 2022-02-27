use std::str::FromStr;

use crate::errors::ParseFormulaError;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
}

impl FromStr for Operator {
    type Err = ParseFormulaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => return Ok(Self::Add),
            _ => return Err(ParseFormulaError),
        }
    }
}
