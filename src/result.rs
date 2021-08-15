use std::result::Result;
use std::fmt::{Display, Formatter};

pub type ValidationResult = Result<(), ValidationError>;

pub struct ValidationError {
    pub messages: Vec<String>
}

impl ValidationError {
    pub fn new() -> Self {
        ValidationError {
            messages: Vec::new()
        }
    }
}

impl From<String> for ValidationError {
    fn from(message: String) -> Self {
        ValidationError {
            messages: vec![message]
        }
    }
}

impl From<Vec<String>> for ValidationError {
    fn from(messages: Vec<String>) -> Self {
        ValidationError {
            messages
        }
    }
}

impl From<Option<String>> for ValidationError {
    fn from(opt_str: Option<String>) -> Self {
        opt_str.map_or(ValidationError::new(), |str| ValidationError::from(str))
    }
}

impl From<Vec<ValidationError>> for ValidationError {
    fn from(errors: Vec<ValidationError>) -> Self {
        ValidationError {
            messages: errors.iter().map(|err| err.messages.clone()).flatten().collect()
        }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.messages.join(";"))
    }
}