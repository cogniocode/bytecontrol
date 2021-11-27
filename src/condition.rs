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