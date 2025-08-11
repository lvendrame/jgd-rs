use std::sync::LazyLock;

use regex::Regex;
use serde_json::Value;

use crate::type_spec::GeneratorConfig;

static RE_FAKES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\$\{(.+?)(\(.+?\))?\})").unwrap());

#[derive(Debug, Clone)]
pub struct Replacer {
    pub start: usize,
    pub end: usize,
    pub length: usize,
    pub key: String,
    pub pattern: String,
    pub tag: String,
}

impl Replacer {
    fn new(captures: &regex::Captures<'_>) -> Self {
        let tag = captures.get(1).unwrap();

        let range = tag.range();
        let start = range.start;
        let end = range.end;
        let length = range.len();

        let tag = tag.as_str().to_string();
        let key = captures.get(2).unwrap().as_str().to_string();

        let arguments = captures.get(3).map_or("".to_string(), |m| m.as_str().to_string());

        let pattern = format!("{}{}", key, arguments);

        Self { start, end, length, key, pattern, tag }
    }
}

pub struct ReplacerCollection {
    pub value: String,
    pub collection: Vec<Replacer>,
    pub full_replace: bool,
}

impl ReplacerCollection {
    pub fn new(value: String) -> Self {
        let collection: Vec<Replacer> = RE_FAKES
            .captures_iter(&value)
            .map(|captures| Replacer::new(&captures))
            .collect();

        let full_replace = if let Some(replacer) = collection.first() {
            replacer.length == value.len()
        } else {
            false
        };

        Self {
            value,
            collection,
            full_replace,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }

    fn get_full_replacer(&self) -> &Replacer {
        self.collection.first().unwrap()
    }

    pub fn replace(&self, config: &mut GeneratorConfig) -> Option<Value> {
        let mut value = self.value.clone();
        if self.full_replace {
            let replacer = self.get_full_replacer();
            if config.fake_keys.contains_key(&replacer.key) {
                return Some(config.fake_generator.generate_by_key(&replacer.pattern, &mut config.rng));
            }
            return Some(Value::String(value));
        }

        for replacer in self.collection.iter().rev() {
            if config.fake_keys.contains_key(&replacer.key) {
                let new_value = config.fake_generator.generate_by_key(&replacer.key, &mut config.rng);
                let new_value = if let Value::String(value) = new_value {
                    value
                } else {
                    new_value.to_string()
                };
                value.replace_range(replacer.start..replacer.end, &new_value);
            }
        }

        Some(Value::String(value))
    }
}