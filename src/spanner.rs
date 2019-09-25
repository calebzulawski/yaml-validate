use linked_hash_map::LinkedHashMap;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::hash::{Hash, Hasher};
use yaml_rust::scanner::Marker;

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Span<'a> {
    begin: Marker,
    end: Marker,
    contents: &'a str,
}

impl Hash for Span<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.begin.index().hash(state);
        self.end.index().hash(state);
    }
}

impl Ord for Span<'_> {
    fn cmp(&self, other: &Span) -> Ordering {
        let begin = self
            .begin
            .index()
            .partial_cmp(&other.begin.index())
            .unwrap();
        let end = self.end.index().partial_cmp(&other.end.index()).unwrap();
        begin.then(end)
    }
}

impl PartialOrd for Span<'_> {
    fn partial_cmp(&self, other: &Span) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Eq, Ord, Hash)]
pub struct SpannedYaml<'a> {
    pub span: Span<'a>,
    pub value: Yaml<'a>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Eq, Ord, Hash)]
pub enum Yaml<'a> {
    Real(String),
    Integer(i64),
    String(String),
    Boolean(bool),
    Array(Vec<SpannedYaml<'a>>),
    Hash(LinkedHashMap<SpannedYaml<'a>, SpannedYaml<'a>>),
    Alias(usize),
    Null,
}
