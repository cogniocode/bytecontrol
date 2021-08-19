pub mod length;
pub mod pattern;

use crate::result::{ValidationResult, ValidationError};

pub trait Rule<T> {
    fn apply(&self, value: &T) -> ValidationResult;
    fn error_message(&self) -> Option<String>;
}

impl <T, F> Rule<T> for F
where
    F: Fn(&T) -> bool + 'static
{
    fn apply(&self, value: &T) -> ValidationResult {
        if (self)(value) {
            Ok(())
        } else {
            Err(ValidationError::from(self.error_message()))
        }
    }

    fn error_message(&self) -> Option<String> {
        Option::None
    }
}
