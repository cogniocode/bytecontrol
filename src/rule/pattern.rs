use regex::Regex;
use crate::rule::Rule;
use crate::result::{ValidationResult, ValidationError};

pub struct PatternRule {
    pub pattern: Regex
}

impl Rule<String> for PatternRule {
    fn apply(&self, value: &String) -> ValidationResult {
        if self.pattern.is_match(value.as_str()) {
            Ok(())
        } else {
            Err(ValidationError::from(self.error_message()))
        }
    }

    fn error_message(&self) -> Option<String> {
        Option::Some(String::from("value doesn't match pattern"))
    }
}