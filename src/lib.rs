pub mod errors;
pub mod formula;
pub mod operator;

use errors::ParseFormulaError;
use formula::Formula;

pub fn exec(lhs: &str, rhs: &str) -> Result<bool, ParseFormulaError> {
    let lhs_formula = Formula::parse(lhs)?;
    let rhs_formula = Formula::parse(rhs)?;
    let result = lhs_formula == rhs_formula;
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use crate::exec;

    #[test]
    fn true_test() {
        assert!(exec("foo", "foo").unwrap());
    }

    #[test]
    fn false_test() {
        assert!(!exec("foo", "bar").unwrap());
    }

    #[test]
    fn add_test() {
        assert!(exec("x + 1", "x + 1").unwrap());
        assert!(exec("x + y + 1", "x + y + 1").unwrap());
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
    }

    #[test]
    fn mul_test() {
        assert!(exec("x * 2", "x * 2").unwrap());
        assert!(exec("x * y * 2", "x * y * 2").unwrap());
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
    }

    #[test]
    fn mul_abbreviate_test() {
        assert!(exec("2 x", "2 * x").unwrap());
        assert!(exec("2 x y", "2 * x * y").unwrap());
        assert!(exec("2x", "2 * x").unwrap());
        assert!(exec("2xy", "2 * x * y").unwrap());
    }
}
