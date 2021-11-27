use crate::result::ValidationResult;

pub trait Rule<T> {
    fn apply(&self, value: &T) -> ValidationResult;
}
