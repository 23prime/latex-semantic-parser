use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

use crate::errors::ParseFormulaError;

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub enum Formula {
    TS(String), // TerminalSymbol
    Add(Vec<Formula>),
    Mul(Vec<Formula>),
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

        return Ok(Formula::Add(
            splitted.into_iter().map(|s| Self::parse(s).unwrap()).collect_vec(),
        ));
    }

    fn parse_by_mul(s: &str) -> Result<Self, ParseFormulaError> {
        let pattern = Regex::new(r"\*| |").unwrap();
        let splitted = Self::split_by_operator(s, &pattern);

        if splitted.len() == 1 {
            return Ok(Formula::TS(splitted[0].to_string()));
        }

        return Ok(Formula::Mul(
            splitted.into_iter().map(|s| Self::parse(s).unwrap()).collect_vec(),
        ));
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
            (Self::TS(l), Self::TS(r)) => l == r,

            // sum(l_formulas) == sum(r_formulas)
            (Self::Add(l_formulas), Self::Add(r_formulas)) => {
                l_formulas.iter().sorted().collect_vec() == r_formulas.iter().sorted().collect_vec()
            }

            // prod(l_formulas) == prod(r_formulas)
            (Self::Mul(l_formulas), Self::Mul(r_formulas)) => {
                l_formulas.iter().sorted().collect_vec() == r_formulas.iter().sorted().collect_vec()
            }

            // o.w.
            _ => false,
        };
    }
}
