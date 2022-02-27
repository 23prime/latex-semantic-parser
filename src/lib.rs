pub fn exec(lhs: &str, rhs: &str) -> bool {
    return lhs == rhs;
}

#[cfg(test)]
mod tests {
    use crate::exec;

    #[test]
    fn true_test() {
        assert!(exec("foo", "foo"));
    }

    #[test]
    fn false_test() {
        assert!(!exec("foo", "bar"));
    }
}
