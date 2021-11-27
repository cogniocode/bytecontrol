use std::rc::Rc;

use crate::result::{ValidationError, ValidationResult};
use crate::rule::Rule;

#[derive(Clone)]
pub enum CompositionKind {
    All,
    Any,
}

impl Default for CompositionKind {
    fn default() -> Self {
        CompositionKind::All
    }
}

pub struct CompositeRule<T> {
    pub kind: CompositionKind,
    pub rules: Vec<Rc<dyn Rule<T>>>,
    pub message: Option<String>,
}

impl<T> Default for CompositeRule<T> {
    fn default() -> Self {
        CompositeRule {
            kind: CompositionKind::default(),
            rules: Vec::default(),
            message: Option::None,
        }
    }
}

impl<T> Clone for CompositeRule<T> {
    fn clone(&self) -> Self {
        CompositeRule {
            kind: self.kind.clone(),
            rules: self.rules.clone(),
            message: self.message.clone(),
        }
    }
}

impl<T> Rule<T> for CompositeRule<T> {
    fn apply(&self, value: &T) -> ValidationResult {
        let errors = self.rules.iter()
            .map(|rule| rule.apply(value))
            .filter(|rule| rule.is_err())
            .map(|rule| rule.unwrap_err())
            .collect::<Vec<ValidationError>>();

        match self.kind {
            CompositionKind::All => {
                if errors.len() > 0 {
                    Err(ValidationError::from(errors))
                } else {
                    Ok(())
                }
            }
            CompositionKind::Any => {
                if (self.rules.len() - errors.len()) > 0 {
                    Ok(())
                } else {
                    Err(ValidationError::from(errors))
                }
            }
        }
    }
}

pub struct RuleComposer<T> {
    rule: CompositeRule<T>,
}

impl<T> Default for RuleComposer<T> {
    fn default() -> Self {
        RuleComposer {
            rule: CompositeRule::default()
        }
    }
}

impl<T> RuleComposer<T> {
    pub fn kind(&mut self, kind: CompositionKind) -> &mut Self {
        self.rule.kind = kind;
        self
    }
    pub fn rule(&mut self, rule: impl Rule<T> + 'static) -> &mut Self {
        self.rule.rules.push(Rc::new(rule));
        self
    }
    pub fn message(&mut self, message: String) -> &mut Self {
        self.rule.message = Option::Some(message);
        self
    }
    pub fn compose(&mut self) -> CompositeRule<T> {
        let rule = self.rule.clone();
        self.rule = CompositeRule::default();
        rule
    }
}

pub fn compose_rules<T>() -> RuleComposer<T> {
    RuleComposer::default()
}

#[cfg(test)]
mod tests {
    use crate::condition::{condition, ConditionalRule};
    use super::*;

    #[test]
    fn test_composition_all() {
        let rule1 = test_rule_contains();
        let rule2 = test_rule_starts_with();

        let composite = compose_rules()
            .kind(CompositionKind::All)
            .rule(rule1)
            .rule(rule2)
            .compose();

        assert!(composite.apply(&String::from("test")).is_ok());
        assert!(composite.apply(&String::from("fail_test")).is_err());
    }

    #[test]
    fn test_composition_any() {
        let rule1 = test_rule_contains();
        let rule2 = test_rule_starts_with();

        let composite = compose_rules()
            .kind(CompositionKind::Any)
            .rule(rule1)
            .rule(rule2)
            .compose();

        assert!(composite.apply(&String::from("test")).is_ok());
        assert!(composite.apply(&String::from("fail_test")).is_ok());
        assert!(composite.apply(&String::from("fail")).is_err());
    }

    fn test_rule_contains() -> ConditionalRule<String> {
        condition(|val: &String| val.contains("test"), String::from("test contains"))
    }

    fn test_rule_starts_with() -> ConditionalRule<String> {
        condition(|val: &String| val.starts_with("test"), String::from("test starts with"))
    }
}