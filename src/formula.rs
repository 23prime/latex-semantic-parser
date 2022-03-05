use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

use crate::errors::ParseFormulaError;

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub enum Formula {
    TS(String), // TerminalSymbol
    Add(Vec<Formula>),
    Mul(Vec<Formula>),
    Empty,
}

impl Formula {
    fn is_empty(&self) -> bool {
        return matches!(self, Self::Empty);
    }

    fn is_single_paren(s: &str) -> bool {
        if !Regex::new(r"^\(.*\)$").unwrap().is_match(s) {
            return false;
        }

        let s_end_paren_removed = &s[0..s.len() - 1];
        let mut paren_open_count = 0;

        for c in s_end_paren_removed.chars() {
            if c == '(' {
                paren_open_count += 1;
            }

            if c == ')' {
                paren_open_count -= 1;
            }

            if paren_open_count == 0 {
                return false;
            }
        }

        return true;
    }

    pub fn parse(s: &str) -> Result<Self, ParseFormulaError> {
        return Self::parse_by_add(s);
    }

    fn parse_by_add(s: &str) -> Result<Self, ParseFormulaError> {
        if Self::is_single_paren(s) {
            return Self::parse(&s[1..s.len() - 1]);
        }

        let mut paren_open_count = 0;
        let mut term = String::new();
        let mut terms = Vec::new();

        for c in s.chars() {
            if c == '+' && paren_open_count == 0 {
                terms.push(term);
                term = String::new();
                continue;
            }

            if c == ')' {
                // close paren before open
                if paren_open_count == 0 {
                    println!("close paren before open => {:?}", s);
                    return Err(ParseFormulaError);
                }

                if paren_open_count != 1 {
                    term.push(c);
                }

                paren_open_count -= 1;
                continue;
            }

            if c == '(' {
                if paren_open_count != 0 {
                    term.push(c);
                }

                paren_open_count += 1;
                continue;
            }

            term.push(c);
        }

        // push last term
        if !term.is_empty() {
            terms.push(term.clone());
        }

        // some paren has not closed
        if paren_open_count != 0 {
            println!("some paren has not closed => {:?}", s);
            return Err(ParseFormulaError);
        }

        terms = terms
            .into_iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect_vec();

        if terms.is_empty() {
            return Ok(Self::Empty);
        }

        if terms.len() == 1 {
            return Self::parse_by_mul(&terms[0]);
        }

        let result_iter = terms.into_iter().map(|s| Self::parse(&s));

        if result_iter.clone().any(|r| r.is_err()) {
            return Err(ParseFormulaError);
        }

        return Ok(Formula::Add(
            result_iter.map(|r| r.unwrap()).filter(|f| !f.is_empty()).collect_vec(),
        ));
    }

    fn parse_by_mul(s: &str) -> Result<Self, ParseFormulaError> {
        if Self::is_single_paren(s) {
            return Self::parse(&s[1..s.len() - 1]);
        }

        let mut paren_open_count = 0;
        let mut term = String::new();
        let mut terms = Vec::new();

        for c in s.chars() {
            if c == '*' && paren_open_count == 0 {
                terms.push(term);
                term = String::new();
                continue;
            }

            if c == ')' {
                // close paren before open
                if paren_open_count == 0 {
                    println!("close paren before open => {:?}", s);
                    return Err(ParseFormulaError);
                }

                if paren_open_count != 1 {
                    term.push(c);
                }

                paren_open_count -= 1;
                continue;
            }

            if c == '(' {
                if paren_open_count != 0 {
                    term.push(c);
                }

                paren_open_count += 1;
                continue;
            }

            term.push(c);
        }

        // push last term
        if !term.is_empty() {
            terms.push(term.clone());
        }

        // some paren has not closed
        if paren_open_count != 0 {
            println!("some paren has not closed => {:?}", s);
            return Err(ParseFormulaError);
        }

        terms = terms
            .into_iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect_vec();

        if terms.is_empty() {
            return Ok(Self::Empty);
        }

        if terms.len() == 1 {
            // for abbreviate
            if terms[0].len() > 1 {
                terms = terms[0].split("").map(|s| s.trim().to_string()).collect_vec();
            } else {
                return Ok(Formula::TS(terms[0].to_string()));
            }
        }

        let result_iter = terms.into_iter().map(|s| Self::parse(&s));

        if result_iter.clone().any(|r| r.is_err()) {
            return Err(ParseFormulaError);
        }

        return Ok(Formula::Mul(
            result_iter.map(|r| r.unwrap()).filter(|f| !f.is_empty()).collect_vec(),
        ));
    }

    fn expand_paren(self) -> Self {
        return match self {
            // Add([Add[x, y], z]) => Add([x, y], z])
            Self::Add(formulas) => {
                return Self::Add(Self::expand_add(
                    formulas.into_iter().map(Self::expand_paren).collect_vec(),
                ))
            }

            // Mul(Mul[x, y], z]) => Mul([x, y], z])
            Self::Mul(formulas) => {
                return Self::Mul(Self::expand_mul(
                    formulas.into_iter().map(Self::expand_paren).collect_vec(),
                ))
            }

            // o.w.
            _ => self,
        };
    }

    fn expand_add(selfs: Vec<Self>) -> Vec<Self> {
        return selfs
            .into_iter()
            .map(|f| match f {
                Self::Add(formulas) => formulas,
                _ => vec![f],
            })
            .flatten()
            .collect_vec();
    }

    fn expand_mul(selfs: Vec<Self>) -> Vec<Self> {
        return selfs
            .into_iter()
            .map(|f| match f {
                Self::Mul(formulas) => formulas,
                _ => vec![f],
            })
            .flatten()
            .collect_vec();
    }

    fn eq_without_expand(&self, other: &Self) -> bool {
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

            // Empty
            (Self::Empty, Self::Empty) => true,

            // o.w.
            _ => false,
        };
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
        return Self::eq_without_expand(&Self::expand_paren(self.clone()), &Self::expand_paren(other.clone()));
    }
}

#[cfg(test)]
mod expand_tests {
    use crate::formula::Formula::{self, *};

    #[test]
    // x == x
    fn true_test() {
        assert!(Formula::eq_without_expand(&TS("x".to_string()), &TS("x".to_string())));
    }

    #[test]
    // x != y
    fn false_test() {
        assert!(!Formula::eq_without_expand(&TS("x".to_string()), &TS("y".to_string())));
    }

    #[test]
    // x => x
    fn no_paren_ts_test() {
        let input = TS("x".to_string());
        let expect = input.clone();
        assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
    }

    #[test]
    // x + y + 1 => x + y + 1
    fn no_paren_add_test() {
        let input = Add(vec![TS("x".to_string()), TS("y".to_string()), TS("1".to_string())]);
        let expect = input.clone();
        assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
    }

    #[test]
    // x y 1 => x y 1
    fn no_paren_mul_test() {
        let input = Mul(vec![TS("x".to_string()), TS("y".to_string()), TS("1".to_string())]);
        let expect = input.clone();
        assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
    }

    #[test]
    // 2 x + y => 2 x + y
    fn no_paren_test() {
        let input = Add(vec![
            Mul(vec![TS("2".to_string()), TS("x".to_string())]),
            TS("y".to_string()),
        ]);
        let expect = input.clone();
        assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
    }

    #[test]
    // 2 (x + y) + z => 2 (x + y) + z
    fn not_expand_paren_test() {
        let input = Add(vec![
            Mul(vec![
                TS("2".to_string()),
                Add(vec![TS("x".to_string()), TS("y".to_string())]),
            ]),
            TS("1".to_string()),
        ]);
        let expect = input.clone();
        assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
    }

    #[test]
    // (x + y) + (a + b) + 1 => x + y + a + b + 1
    fn expand_add_paren_test() {
        let input = Add(vec![
            Add(vec![TS("x".to_string()), TS("y".to_string())]),
            Add(vec![TS("a".to_string()), TS("b".to_string())]),
            TS("1".to_string()),
        ]);
        let expect = Add(vec![
            TS("x".to_string()),
            TS("y".to_string()),
            TS("a".to_string()),
            TS("b".to_string()),
            TS("1".to_string()),
        ]);
        assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
    }

    #[test]
    // ((x + y) + z) + 1 => x + y + z + 1
    fn expand_recursive_add_paren_test() {
        let input = Add(vec![
            Add(vec![
                Add(vec![TS("x".to_string()), TS("y".to_string())]),
                TS("z".to_string()),
            ]),
            TS("1".to_string()),
        ]);
        let expect = Add(vec![
            TS("x".to_string()),
            TS("y".to_string()),
            TS("z".to_string()),
            TS("1".to_string()),
        ]);
        assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
    }

    #[test]
    // (x y) (a b) 1 => x y a b 1
    fn expand_mul_paren_test() {
        let input = Mul(vec![
            Mul(vec![TS("x".to_string()), TS("y".to_string())]),
            Mul(vec![TS("a".to_string()), TS("b".to_string())]),
            TS("1".to_string()),
        ]);
        let expect = Mul(vec![
            TS("x".to_string()),
            TS("y".to_string()),
            TS("a".to_string()),
            TS("b".to_string()),
            TS("1".to_string()),
        ]);
        assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
    }

    #[test]
    // ((x y) z) 1 => x y z 1
    fn expand_recursive_mul_paren_test() {
        let input = Mul(vec![
            Mul(vec![
                Mul(vec![TS("x".to_string()), TS("y".to_string())]),
                TS("z".to_string()),
            ]),
            TS("1".to_string()),
        ]);
        let expect = Mul(vec![
            TS("x".to_string()),
            TS("y".to_string()),
            TS("z".to_string()),
            TS("1".to_string()),
        ]);
        assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
    }
}
