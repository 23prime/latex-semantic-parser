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

    fn is_only_paren(s: &str) -> bool {
        return Regex::new(r"^[\(\)]+$").unwrap().is_match(s);
    }

    pub fn parse(s: &str) -> Result<Self, ParseFormulaError> {
        return Self::parse_by_add(s);
    }

    fn parse_by_add(s: &str) -> Result<Self, ParseFormulaError> {
        if Self::is_only_paren(s) {
            return Ok(Self::Empty);
        }

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
                    println!("Add: close paren before open => {:?}", s);
                    return Err(ParseFormulaError);
                }

                paren_open_count -= 1;
            }

            if c == '(' {
                paren_open_count += 1;
            }

            term.push(c);
            println!(
                "Add: c => {:?}, term => {:?}, paren_open_count => {:?}",
                c, term, paren_open_count
            );
        }

        // push last term
        if !term.is_empty() {
            terms.push(term.clone());
        }

        // some paren has not closed
        if paren_open_count != 0 {
            println!("Add: some paren has not closed => {:?}", s);
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

        let parsed_terms = result_iter.map(|r| r.unwrap()).filter(|f| !f.is_empty()).collect_vec();

        if parsed_terms.is_empty() {
            return Ok(Formula::Empty);
        } else {
            return Ok(Formula::Add(parsed_terms));
        }
    }

    fn parse_by_mul(s: &str) -> Result<Self, ParseFormulaError> {
        if Self::is_only_paren(s) {
            return Ok(Self::Empty);
        }

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
                    println!("Mul: close paren before open => {:?}", s);
                    return Err(ParseFormulaError);
                }

                paren_open_count -= 1;
            }

            if c == '(' {
                paren_open_count += 1;
            }

            term.push(c);
            println!(
                "Mul: c => {:?}, term => {:?}, paren_open_count => {:?}",
                c, term, paren_open_count
            );
        }

        // push last term
        if !term.is_empty() {
            terms.push(term.clone());
        }

        // some paren has not closed
        if paren_open_count != 0 {
            println!("Mul: some paren has not closed => {:?}", s);
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

        let parsed_terms = result_iter.map(|r| r.unwrap()).filter(|f| !f.is_empty()).collect_vec();

        if parsed_terms.is_empty() {
            return Ok(Formula::Empty);
        } else {
            return Ok(Formula::Mul(parsed_terms));
        }
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
            .flat_map(|f| match f {
                Self::Add(formulas) => formulas,
                _ => vec![f],
            })
            .collect_vec();
    }

    fn expand_mul(selfs: Vec<Self>) -> Vec<Self> {
        return selfs
            .into_iter()
            .flat_map(|f| match f {
                Self::Mul(formulas) => formulas,
                _ => vec![f],
            })
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
mod tests {
    use crate::formula::Formula::{self, *};

    // helper
    fn ts(s: &str) -> Formula {
        return TS(s.to_string());
    }

    #[cfg(test)]
    mod parse_tests {
        use super::*;

        #[test]
        fn empty_test() {
            assert!(Formula::eq_without_expand(&Formula::parse("").unwrap(), &Empty));
            assert!(Formula::eq_without_expand(&Formula::parse("    ").unwrap(), &Empty));
        }

        #[test]
        fn ts_test() {
            let input = Formula::parse("x").unwrap();
            let expect = ts("x");
            assert!(Formula::eq_without_expand(&input, &expect));
        }

        #[test]
        fn ts_remove_paren_test() {
            let input = Formula::parse("(x)").unwrap();
            let expect = ts("x");
            assert!(Formula::eq_without_expand(&input, &expect));
        }

        #[test]
        fn add_2_terms_test() {
            let input = Formula::parse("x + 1").unwrap();
            let expect = Add(vec![ts("x"), ts("1")]);
            assert!(Formula::eq_without_expand(&input, &expect));
        }

        #[test]
        fn add_3_terms_test() {
            let input = Formula::parse("x + y + 1").unwrap();
            let expect = Add(vec![ts("x"), ts("y"), ts("1")]);
            assert!(Formula::eq_without_expand(&input, &expect));
        }

        #[test]
        fn add_trim_test() {
            let input_no_spaces = Formula::parse("x+1").unwrap();
            let input_many_spaces = Formula::parse(" x  +  1 ").unwrap();
            let expect = Add(vec![ts("x"), ts("1")]);
            assert!(Formula::eq_without_expand(&input_no_spaces, &expect));
            assert!(Formula::eq_without_expand(&input_many_spaces, &expect));
        }

        #[test]
        fn mul_2_terms_test() {
            let input = Formula::parse("x * 1").unwrap();
            let expect = Mul(vec![ts("x"), ts("1")]);
            assert!(Formula::eq_without_expand(&input, &expect));
        }

        #[test]
        fn mul_3_terms_test() {
            let input = Formula::parse("x * y * 1").unwrap();
            let expect = Mul(vec![ts("x"), ts("y"), ts("1")]);
            assert!(Formula::eq_without_expand(&input, &expect));
        }

        #[test]
        fn mul_2_terms_abbreviate_test() {
            let input = Formula::parse("2 x").unwrap();
            let expect = Mul(vec![ts("2"), ts("x")]);
            assert!(Formula::eq_without_expand(&input, &expect));
        }

        #[test]
        fn mul_3_terms_abbreviate_test() {
            let input = Formula::parse("2 x y").unwrap();
            let expect = Mul(vec![ts("2"), ts("x"), ts("y")]);
            assert!(Formula::eq_without_expand(&input, &expect));
        }

        #[test]
        fn mul_2_terms_abbreviate_no_spaces_test() {
            let input = Formula::parse("2x").unwrap();
            let expect = Mul(vec![ts("2"), ts("x")]);
            assert!(Formula::eq_without_expand(&input, &expect));
        }

        #[test]
        fn mul_3_terms_abbreviate_no_spaces_test() {
            let input = Formula::parse("2xy").unwrap();
            let expect = Mul(vec![ts("2"), ts("x"), ts("y")]);
            assert!(Formula::eq_without_expand(&input, &expect));
        }

        #[test]
        fn mul_trim_test() {
            let input_no_spaces = Formula::parse("x*1").unwrap();
            let input_many_spaces = Formula::parse(" x  *  1 ").unwrap();
            let expect = Mul(vec![ts("x"), ts("1")]);
            assert!(Formula::eq_without_expand(&input_no_spaces, &expect));
            assert!(Formula::eq_without_expand(&input_many_spaces, &expect));
        }

        #[test]
        fn empty_paren_test() {
            assert!(Formula::eq_without_expand(&Formula::parse("()").unwrap(), &Empty));
            assert!(Formula::eq_without_expand(&Formula::parse(" + ").unwrap(), &Empty));
            assert!(Formula::eq_without_expand(&Formula::parse("(()+())").unwrap(), &Empty));
            assert!(Formula::eq_without_expand(&Formula::parse(" * ").unwrap(), &Empty));
            assert!(Formula::eq_without_expand(&Formula::parse("(()())").unwrap(), &Empty));
        }

        #[test]
        fn paren_fail_test() {
            assert!(Formula::parse("(x").is_err());
            assert!(Formula::parse("x)").is_err());
            assert!(Formula::parse(")x(").is_err());
            assert!(Formula::parse("(x))(").is_err());
            assert!(Formula::parse("(x + 1))").is_err());
            assert!(Formula::parse("((x + 1)").is_err());
        }

        #[test]
        fn paren_add_test() {
            let input = Formula::parse("(x + y) + 1").unwrap();
            let expect = Add(vec![Add(vec![ts("x"), ts("y")]), ts("1")]);
            assert!(Formula::eq_without_expand(&input, &expect));
        }

        #[test]
        fn paren_mul_test() {
            let input = Formula::parse("(x * y) * 1").unwrap();
            let expect = Mul(vec![Mul(vec![ts("x"), ts("y")]), ts("1")]);
            assert!(Formula::eq_without_expand(&input, &expect));
        }
    }

    #[cfg(test)]
    mod expand_tests {
        use super::*;

        #[test]
        // x == x
        fn true_test() {
            assert!(Formula::eq_without_expand(&ts("x"), &ts("x")));
        }

        #[test]
        // x != y
        fn false_test() {
            assert!(!Formula::eq_without_expand(&ts("x"), &ts("y")));
        }

        #[test]
        // x => x
        fn no_paren_ts_test() {
            let input = ts("x");
            let expect = input.clone();
            assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
        }

        #[test]
        // x + y + 1 => x + y + 1
        fn no_paren_add_test() {
            let input = Add(vec![ts("x"), ts("y"), ts("1")]);
            let expect = input.clone();
            assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
        }

        #[test]
        // x y 1 => x y 1
        fn no_paren_mul_test() {
            let input = Mul(vec![ts("x"), ts("y"), ts("1")]);
            let expect = input.clone();
            assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
        }

        #[test]
        // 2 x + y => 2 x + y
        fn no_paren_test() {
            let input = Add(vec![Mul(vec![ts("2"), ts("x")]), ts("y")]);
            let expect = input.clone();
            assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
        }

        #[test]
        // 2 (x + y) + z => 2 (x + y) + z
        fn not_expand_paren_test() {
            let input = Add(vec![Mul(vec![ts("2"), Add(vec![ts("x"), ts("y")])]), ts("1")]);
            let expect = input.clone();
            assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
        }

        #[test]
        // (x + y) + (a + b) + 1 => x + y + a + b + 1
        fn expand_add_paren_test() {
            let input = Add(vec![Add(vec![ts("x"), ts("y")]), Add(vec![ts("a"), ts("b")]), ts("1")]);
            let expect = Add(vec![ts("x"), ts("y"), ts("a"), ts("b"), ts("1")]);
            assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
        }

        #[test]
        // ((x + y) + z) + 1 => x + y + z + 1
        fn expand_recursive_add_paren_test() {
            let input = Add(vec![Add(vec![Add(vec![ts("x"), ts("y")]), ts("z")]), ts("1")]);
            let expect = Add(vec![ts("x"), ts("y"), ts("z"), ts("1")]);
            assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
        }

        #[test]
        // (x y) (a b) 1 => x y a b 1
        fn expand_mul_paren_test() {
            let input = Mul(vec![Mul(vec![ts("x"), ts("y")]), Mul(vec![ts("a"), ts("b")]), ts("1")]);
            let expect = Mul(vec![ts("x"), ts("y"), ts("a"), ts("b"), ts("1")]);
            assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
        }

        #[test]
        // ((x y) z) 1 => x y z 1
        fn expand_recursive_mul_paren_test() {
            let input = Mul(vec![Mul(vec![Mul(vec![ts("x"), ts("y")]), ts("z")]), ts("1")]);
            let expect = Mul(vec![ts("x"), ts("y"), ts("z"), ts("1")]);
            assert!(Formula::eq_without_expand(&Formula::expand_paren(input), &expect));
        }
    }
}
