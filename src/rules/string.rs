use regex::Regex;
use crate::condition::{condition, ConditionalRule};

pub fn length(min: usize, max: usize) -> ConditionalRule<String> {
    condition(move |val: &String| val.len() >= min && val.len() <= max, String::from("value length is out of bounds"))
}

pub fn pattern(pattern: Regex) -> ConditionalRule<String> {
    condition(move |val: &String| pattern.is_match(val.as_str()), String::from("value doesn't match pattern"))
}