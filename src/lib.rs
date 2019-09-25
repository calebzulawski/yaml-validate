mod spanner;
use crate::spanner::{Span, SpannedYaml, Yaml};

pub struct ValidationFailure<'a> {
    message: String,
    span: &'a Span<'a>,
}

impl<'a> ValidationFailure<'a> {
    pub fn new(message: String, span: &'a Span) -> Self {
        Self {
            message: message,
            span: span,
        }
    }
}

pub trait Validator<'a> {
    fn class(&self) -> &'a str;
    fn name(&self) -> Option<&'a str>;
    fn validate<'b>(&self, value: &'b SpannedYaml) -> Option<ValidationFailure<'b>>;
}

pub struct Map<'a> {
    name: &'a str,
    map: Vec<(&'a dyn Validator<'a>, &'a dyn Validator<'a>)>,
}

impl<'a> Map<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name: name,
            map: Vec::new(),
        }
    }

    fn key_value(&mut self, key: &'a dyn Validator<'a>, value: &'a dyn Validator<'a>) -> &Self {
        self.map.push((key, value));
        self
    }
}

impl<'a> Validator<'a> for Map<'a> {
    fn class(&self) -> &'a str {
        "map"
    }

    fn name(&self) -> Option<&'a str> {
        Some(self.name)
    }

    fn validate<'b>(&self, spanned_value: &'b SpannedYaml) -> Option<ValidationFailure<'b>> {
        if let Yaml::Hash(map) = &spanned_value.value {
            None
        } else {
            Some(ValidationFailure::new(
                "expected map".to_string(),
                &spanned_value.span,
            ))
        }
    }
}
