use crate::rule::Rule;
use crate::result::{ValidationResult, ValidationError};
use std::rc::Rc;

#[derive(Clone)]
pub enum CompositionKind {
    All,
    Any
}

impl Default for CompositionKind {
    fn default() -> Self {
        CompositionKind::All
    }
}

pub struct CompositeRule<T> {
    pub kind: CompositionKind,
    pub rules: Vec<Rc<dyn Rule<T>>>
}

impl <T> Default for CompositeRule<T> {
    fn default() -> Self {
        CompositeRule {
            kind: CompositionKind::default(),
            rules: Vec::default()
        }
    }
}

impl <T> Clone for CompositeRule<T> {
    fn clone(&self) -> Self {
        CompositeRule {
            kind: self.kind.clone(),
            rules: self.rules.clone()
        }
    }
}

impl <T> Rule<T> for CompositeRule<T> {
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

    fn error_message(&self) -> Option<String> {
        Option::None
    }
}

pub struct CompositeRuleBuilder<T> {
    rule: CompositeRule<T>
}

impl <T> Default for CompositeRuleBuilder<T> {
    fn default() -> Self {
        CompositeRuleBuilder {
            rule: CompositeRule::default()
        }
    }
}

impl <T> CompositeRuleBuilder<T> {
    pub fn kind(&mut self, kind: CompositionKind) -> &mut Self {
        self.rule.kind = kind;
        self
    }
    pub fn rule(&mut self, rule: impl Rule<T> + 'static) -> &mut Self {
        self.rule.rules.push(Rc::new(rule));
        self
    }
    pub fn compose(&mut self) -> CompositeRule<T> {
        let rule = self.rule.clone();
        self.rule = CompositeRule::default();
        rule
    }
}

pub fn compose_rules<T>() -> CompositeRuleBuilder<T> {
    CompositeRuleBuilder::default()
}