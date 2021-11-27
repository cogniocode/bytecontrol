use crate::result::{ValidationError, ValidationResult};
use crate::rule::Rule;

pub struct ConditionalRule<T> {
    pub condition: Box<dyn Fn(&T) -> bool>,
    pub message: String,
}

impl<T> Rule<T> for ConditionalRule<T> {
    fn apply(&self, value: &T) -> ValidationResult {
        if (self.condition)(value) {
            Ok(())
        } else {
            Err(ValidationError::from(self.message.clone()))
        }
    }
}

impl<T> ConditionalRule<T> {
    pub fn new(condition: impl Fn(&T) -> bool + 'static, message: String) -> Self {
        ConditionalRule {
            condition: Box::new(condition),
            message,
        }
    }
}

pub fn condition<T>(condition: impl Fn(&T) -> bool + 'static, message: String) -> ConditionalRule<T> {
    ConditionalRule::new(condition, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        let rule = condition(|val: &String| val.starts_with("test"), String::from("didn't start with 'test'"));

        assert!(rule.apply(&String::from("test")).is_ok());
        assert!(rule.apply(&String::from("fail")).is_err());
    }

    #[test]
    fn test_error_message() {
        let message = "didn't start with 'test'";
        let rule = condition(|val: &String| val.starts_with("test"), String::from(message));

        assert_eq!(
            rule.apply(&String::from("fail"))
                .unwrap_err()
                .messages.first()
                .unwrap_or(&String::from("")),
            message
        );
    }
}