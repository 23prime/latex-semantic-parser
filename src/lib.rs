pub mod errors;
pub mod formula;
pub mod logger;
pub mod operator;

use log::debug;

use errors::ParseFormulaError;
use formula::Formula;

pub fn exec(lhs: &str, rhs: &str) -> Result<bool, ParseFormulaError> {
    logger::init();

    let lhs_formula = Formula::parse(lhs)?;
    let rhs_formula = Formula::parse(rhs)?;
    debug!(
        "Parse results => {{ lhs => {:?}, rhs => {:?} }}",
        lhs_formula, rhs_formula
    );

    let result = lhs_formula == rhs_formula;
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use crate::exec;

    #[test]
    fn true_test() {
        assert!(exec("x", "x").unwrap());
        assert!(exec("x", "(x)").unwrap());
    }

    #[test]
    fn false_test() {
        assert!(!exec("x", "y").unwrap());
    }

    #[test]
    fn parse_fail_test() {
        assert!(exec("x", "(x").is_err());
        assert!(exec("x", "x)").is_err());
        assert!(exec("x", ")x(").is_err());
        assert!(exec("x", "(x))(").is_err());
        assert!(exec("x", "(x + 1))").is_err());
        assert!(exec("x", "((x + 1)").is_err());
    }

    #[test]
    fn empty_test() {
        assert!(exec("", "").unwrap());
        assert!(exec("", "    ").unwrap());
        assert!(exec("", "()").unwrap());
        assert!(exec("", " + ").unwrap());
        assert!(exec("", "(()+())").unwrap());
        assert!(exec("", " * ").unwrap());
        assert!(exec("", "(()())").unwrap());
    }

    #[test]
    fn empty_falsy_test() {
        assert!(!exec("", "x").unwrap());
        assert!(!exec("", "x + 1").unwrap());
        assert!(!exec("", "x * 2").unwrap());
    }

    #[test]
    fn add_test() {
        assert!(exec("x + 1", "x + 1").unwrap());
        assert!(exec("x + 1", "(x + 1)").unwrap());
        assert!(exec("x + 1", "(x) + (1)").unwrap());
        assert!(exec("x + 1", "((x) + (1))").unwrap());
        assert!(exec("x + y + 1", "x + y + 1").unwrap());
        assert!(exec("x + y + 1", "(x + y + 1)").unwrap());
        assert!(exec("x + y + 1", "(x) + (y) + (1)").unwrap());
        assert!(exec("x + y + 1", "(x + y) + 1").unwrap());
        assert!(exec("x + y + 1", "x + (y + 1)").unwrap());
        assert!(exec("x + y + 1", "((x + y) + 1)").unwrap());
        assert!(exec("x + y + 1", "(x + (y + 1))").unwrap());
        assert!(exec("x + y + 1", "((x) + (y)) + (1)").unwrap());
    }

    #[test]
    fn sub_test() {
        assert!(exec("x - 1", "x - 1").unwrap());
        assert!(exec("x - 1", "(x - 1)").unwrap());
        assert!(exec("x - 1", "(x) - (1)").unwrap());
        assert!(exec("x - 1", "((x) - (1))").unwrap());
        assert!(exec("x - y - 1", "x - y - 1").unwrap());
        assert!(exec("x - y - 1", "(x - y - 1)").unwrap());
        assert!(exec("x - y - 1", "(x) - (y) - (1)").unwrap());
        assert!(exec("x - y - 1", "(x - y) - 1").unwrap());
        assert!(exec("x - y - 1", "((x - y) - 1)").unwrap());
        assert!(exec("x - y - 1", "((x) - (y)) - (1)").unwrap());
        // TODO:
        // assert!(exec("x - y - 1", "x - (y + 1)").unwrap());
        // assert!(exec("x - y - 1", "(x - (y + 1))").unwrap());
    }

    #[test]
    fn add_falsy_test() {
        assert!(!exec("x + 1", "x + 2").unwrap());
        assert!(!exec("x + y + 1", "x + y + 2").unwrap());
    }

    #[test]
    fn add_commutative_test() {
        assert!(exec("x + 1", "1 + x").unwrap());
        assert!(exec("x + y + 1", "x + 1 + y").unwrap());
        assert!(exec("x + y + 1", "y + x + 1").unwrap());
        assert!(exec("x + y + 1", "y + 1 + x").unwrap());
        assert!(exec("x + y + 1", "1 + x + y").unwrap());
        assert!(exec("x + y + 1", "1 + y + x").unwrap());
    }

    #[test]
    fn add_trim_test() {
        assert!(exec("x + 1", " x  +  1 ").unwrap());
        assert!(exec("x + 1", "x+1").unwrap());
    }

    #[test]
    fn mul_test() {
        assert!(exec("x * 2", "x * 2").unwrap());
        assert!(exec("x * 2", "(x * 2)").unwrap());
        assert!(exec("x * 2", "(x) * (2)").unwrap());
        assert!(exec("x * 2", "((x) * (2))").unwrap());
        assert!(exec("x * y * 2", "x * y * 2").unwrap());
        assert!(exec("x * y * 2", "(x * y * 2)").unwrap());
        assert!(exec("x * y * 2", "(x) * (y) * (2)").unwrap());
        assert!(exec("x * y * 2", "(x * y) * 2").unwrap());
        assert!(exec("x * y * 2", "x * (y * 2)").unwrap());
        assert!(exec("x * y * 2", "((x * y) * 2)").unwrap());
        assert!(exec("x * y * 2", "(x * (y * 2))").unwrap());
        assert!(exec("x * y * 2", "((x) * (y) * 2)").unwrap());
    }

    #[test]
    fn mul_falsy_test() {
        assert!(!exec("x * 2", "x * 3").unwrap());
        assert!(!exec("x * y * 2", "x * y * 3").unwrap());
    }

    #[test]
    fn mul_commutative_test() {
        assert!(exec("x * 2", "2 * x").unwrap());
        assert!(exec("x * y * 2", "x * 2 * y").unwrap());
        assert!(exec("x * y * 2", "y * x * 2").unwrap());
        assert!(exec("x * y * 2", "y * 2 * x").unwrap());
        assert!(exec("x * y * 2", "2 * x * y").unwrap());
        assert!(exec("x * y * 2", "2 * y * x").unwrap());
    }

    #[test]
    fn mul_trim_test() {
        assert!(exec("x * 2", " x  *  2 ").unwrap());
        assert!(exec("x * 2", "x*2").unwrap());
    }

    #[test]
    fn mul_abbreviate_test() {
        assert!(exec("2 * x", "2 x").unwrap());
        assert!(exec("2 * x", "2x").unwrap());
        assert!(exec("2 * x * y", "2 x y").unwrap());
        assert!(exec("2 * x * y", "2xy").unwrap());
    }

    #[test]
    fn mix_add_and_mul_test() {
        assert!(exec("2 * x + 3 * y + 1", "2 * x + 3 * y + 1").unwrap());
        assert!(exec("2 * x + 3 * y + 1", "3 * y + 2 * x + 1").unwrap());
    }
}
