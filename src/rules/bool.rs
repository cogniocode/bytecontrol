use crate::condition::{ConditionalRule, condition};

pub fn is_true() -> ConditionalRule<bool> {
    condition(|val: &bool| *val == true, String::from("value is not true"))
}

pub fn is_false() -> ConditionalRule<bool> {
    condition(|val: &bool| *val == false, String::from("value is not false"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::Rule;

    #[test]
    fn test_is_true() {
        assert!(is_true().apply(&true).is_ok());
        assert!(is_true().apply(&false).is_err());
    }

    fn test_is_false() {
        assert!(is_false().apply(&false).is_ok());
        assert!(!is_false().apply(&true).is_err());
    }
}