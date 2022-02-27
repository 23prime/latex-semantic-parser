use std::str::FromStr;

use crate::errors::ParseFormulaError;

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub enum Formula {
    TerminalSymbol(String),
    Add { formulas: Vec<Formula> },
}

impl Formula {
    pub fn parse(s: &str) -> Result<Self, ParseFormulaError> {
        let trimmed = s.trim();
        let operator = "+";
        let mut splitted = trimmed
            .split(operator)
            .into_iter()
            .map(|s| s.trim())
            .collect::<Vec<_>>();
        splitted.sort_unstable();

        if splitted.len() == 1 {
            return Ok(Formula::TerminalSymbol(trimmed.to_string()));
        }

        return Ok(Formula::Add {
            formulas: splitted
                .into_iter()
                .map(|s| Self::parse(s).unwrap())
                .collect::<Vec<_>>(),
        });
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

            // sum(l_formulas) == sum (r_formulas)
            (Self::Add { formulas: l_formulas }, Self::Add { formulas: r_formulas }) => l_formulas == r_formulas,

            // o.w.
            _ => false,
        };
    }
}
