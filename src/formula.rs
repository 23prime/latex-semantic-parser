use std::str::FromStr;

use crate::errors::ParseFormulaError;
use crate::operator::Operator;

#[derive(Debug, Clone)]
pub enum Formula {
    TerminalSymbol(String),
    Operation {
        operator: Operator,
        lhs: Box<Formula>,
        rhs: Box<Formula>,
    },
}

impl Formula {
    pub fn parse(s: &str) -> Result<Self, ParseFormulaError> {
        let trimmed = s.trim();
        let operator = "+";
        let splitted = trimmed.rsplitn(2, operator).collect::<Vec<_>>();

        if splitted.len() == 1 {
            return Ok(Formula::TerminalSymbol(trimmed.to_string()));
        }

        if splitted.len() == 2 {
            return Ok(Formula::Operation {
                operator: Operator::from_str(operator)?,
                lhs: Box::new(Self::parse(splitted[1])?),
                rhs: Box::new(Self::parse(splitted[0])?),
            });
        }

        return Err(ParseFormulaError);
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

            // l_lhs `l_operator` l_rhs == r_lhs `r_operator` r_rhs
            (
                Self::Operation {
                    operator: l_operator,
                    lhs: l_lhs,
                    rhs: l_rhs,
                },
                Self::Operation {
                    operator: r_operator,
                    lhs: r_lhs,
                    rhs: r_rhs,
                },
            ) => l_operator == r_operator && ((l_lhs == r_lhs && l_rhs == r_rhs) || (l_lhs == r_rhs && l_rhs == r_lhs)),

            // o.w.
            _ => false,
        };
    }
}
