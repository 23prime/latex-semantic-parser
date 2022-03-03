use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

use crate::errors::ParseFormulaError;

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub enum Formula {
    TerminalSymbol(String),
    Add { formulas: Vec<Formula> },
    Mul { formulas: Vec<Formula> },
}

impl Formula {
    pub fn parse(s: &str) -> Result<Self, ParseFormulaError> {
        return Self::parse_by_add(s);
    }

    fn parse_by_add(s: &str) -> Result<Self, ParseFormulaError> {
        let pattern = Regex::new(r"\+").unwrap();
        let splitted = Self::split_by_operator(s, &pattern);

        if splitted.len() == 1 {
            return Self::parse_by_mul(splitted[0]);
        }

        return Ok(Formula::Add {
            formulas: splitted.into_iter().map(|s| Self::parse(s).unwrap()).collect_vec(),
        });
    }

    fn parse_by_mul(s: &str) -> Result<Self, ParseFormulaError> {
        let pattern = Regex::new(r"\*").unwrap();
        let splitted = Self::split_by_operator(s, &pattern);

        if splitted.len() == 1 {
            return Ok(Formula::TerminalSymbol(splitted[0].to_string()));
        }

        return Ok(Formula::Mul {
            formulas: splitted.into_iter().map(|s| Self::parse(s).unwrap()).collect_vec(),
        });
    }

    fn split_by_operator<'a>(s: &'a str, pattern: &'a Regex) -> Vec<&'a str> {
        return pattern
            .split(s)
            .into_iter()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect_vec();
    }
}

impl FromStr for Formula {
    type Err = ParseFormulaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Self::parse(s);
    }
}

impl PartialEq for Formula {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            // l == r
            (Self::TerminalSymbol(l), Self::TerminalSymbol(r)) => l == r,

            // sum(l_formulas) == sum(r_formulas)
            (Self::Add { formulas: l_formulas }, Self::Add { formulas: r_formulas }) => {
                l_formulas.iter().sorted().collect_vec() == r_formulas.iter().sorted().collect_vec()
            }

            // prod(l_formulas) == prod(r_formulas)
            (Self::Mul { formulas: l_formulas }, Self::Mul { formulas: r_formulas }) => {
                l_formulas.iter().sorted().collect_vec() == r_formulas.iter().sorted().collect_vec()
            }

            // o.w.
            _ => false,
        };
    }
}
