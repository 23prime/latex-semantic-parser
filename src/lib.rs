use std::str::FromStr;

mod errors;
mod formula;

use errors::ParseFormulaError;
use formula::Formula;

pub fn exec(lhs: &str, rhs: &str) -> Result<bool, ParseFormulaError> {
    let lhs_formula = Formula::from_str(lhs)?;
    let rhs_formula = Formula::from_str(rhs)?;
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
}
