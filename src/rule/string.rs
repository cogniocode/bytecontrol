use crate::rule::Rule;
use crate::result::{ValidationResult, ValidationError};
use regex::Regex;

pub struct LengthRule {
    pub min: usize,
    pub max: usize
}

pub fn length(min: usize, max: usize) -> LengthRule {
    LengthRule::new(min, max)
}

impl Default for LengthRule {
    fn default() -> Self {
        LengthRule {
            min: usize::MIN,
            max: usize::MAX
        }
    }
}

impl LengthRule {
    pub fn new(min: usize, max: usize) -> Self {
        LengthRule {
            min,
            max
        }
    }
}

impl Rule<String> for LengthRule {
    fn apply(&self, value: &String) -> ValidationResult {
        let length = value.len();
        if length >= self.min && length <= self.max {
            Ok(())
        } else {
            Err(ValidationError::from(self.error_message()))
        }
    }

    fn error_message(&self) -> Option<String> {
        Option::Some(String::from("value length is out of bounds"))
    }
}

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