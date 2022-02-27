use std::str::FromStr;

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
        let splitted = Self::split_by_operator(s, "+");

        if splitted.len() == 1 {
            return Self::parse_by_mul(splitted[0]);
        }

        return Ok(Formula::Add {
            formulas: splitted
                .into_iter()
                .map(|s| Self::parse(s).unwrap())
                .collect::<Vec<_>>(),
        });
    }

    fn parse_by_mul(s: &str) -> Result<Self, ParseFormulaError> {
        let splitted = Self::split_by_operator(s, "*");

        if splitted.len() == 1 {
            return Ok(Formula::TerminalSymbol(splitted[0].to_string()));
        }

        return Ok(Formula::Mul {
            formulas: splitted
                .into_iter()
                .map(|s| Self::parse(s).unwrap())
                .collect::<Vec<_>>(),
        });
    }

    fn split_by_operator<'a>(s: &'a str, operator: &'a str) -> Vec<&'a str> {
        let mut result = s.split(operator).into_iter().map(|s| s.trim()).collect::<Vec<_>>();
        result.sort_unstable();
        return result;
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
            (Self::Add { formulas: l_formulas }, Self::Add { formulas: r_formulas }) => l_formulas == r_formulas,

            // prod(l_formulas) == prod(r_formulas)
            (Self::Mul { formulas: l_formulas }, Self::Mul { formulas: r_formulas }) => l_formulas == r_formulas,

            // o.w.
            _ => false,
        };
    }
}
