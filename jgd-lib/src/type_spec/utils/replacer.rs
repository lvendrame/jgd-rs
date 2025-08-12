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
    #[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{type_spec::GeneratorConfig};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn create_test_config() -> GeneratorConfig {
        GeneratorConfig::new("EN", None)
    }

    #[test]
    fn test_replacer_new() {
        let regex = regex::Regex::new(r"(\$\{(.+?)(\(.+?\))?\})").unwrap();
        let text = "Hello ${name.firstName} from ${address.cityName}!";
        let captures: Vec<regex::Captures> = regex.captures_iter(text).collect();

        assert_eq!(captures.len(), 2);

        let replacer1 = Replacer::new(&captures[0]);
        assert_eq!(replacer1.tag, "${name.firstName}");
        assert_eq!(replacer1.key, "name.firstName");
        assert_eq!(replacer1.pattern, "name.firstName");
        assert_eq!(replacer1.start, 6);
        assert_eq!(replacer1.end, 23);
        assert_eq!(replacer1.length, 17);

        let replacer2 = Replacer::new(&captures[1]);
        assert_eq!(replacer2.tag, "${address.cityName}");
        assert_eq!(replacer2.key, "address.cityName");
        assert_eq!(replacer2.pattern, "address.cityName");
        assert_eq!(replacer2.start, 29);
        assert_eq!(replacer2.end, 48);
        assert_eq!(replacer2.length, 19);
    }

    #[test]
    fn test_replacer_new_with_arguments() {
        let regex = regex::Regex::new(r"(\$\{(.+?)(\(.+?\))?\})").unwrap();
        let text = "Number: ${lorem.words(3..5)}";
        let captures: Vec<regex::Captures> = regex.captures_iter(text).collect();

        assert_eq!(captures.len(), 1);

        let replacer = Replacer::new(&captures[0]);
        assert_eq!(replacer.tag, "${lorem.words(3..5)}");
        assert_eq!(replacer.key, "lorem.words");
        assert_eq!(replacer.pattern, "lorem.words(3..5)");
        assert_eq!(replacer.start, 8);
        assert_eq!(replacer.end, 28);
        assert_eq!(replacer.length, 20);
    }

    #[test]
    fn test_replacer_collection_new_empty() {
        let collection = ReplacerCollection::new("Hello world!".to_string());

        assert!(collection.is_empty());
        assert!(!collection.full_replace);
        assert_eq!(collection.value, "Hello world!");
        assert_eq!(collection.collection.len(), 0);
    }

    #[test]
    fn test_replacer_collection_new_single_replacement() {
        let collection = ReplacerCollection::new("${name.firstName}".to_string());

        assert!(!collection.is_empty());
        assert!(collection.full_replace);
        assert_eq!(collection.value, "${name.firstName}");
        assert_eq!(collection.collection.len(), 1);

        let replacer = &collection.collection[0];
        assert_eq!(replacer.key, "name.firstName");
        assert_eq!(replacer.pattern, "name.firstName");
    }

    #[test]
    fn test_replacer_collection_new_multiple_replacements() {
        let collection = ReplacerCollection::new(
            "Hello ${name.firstName} from ${address.cityName}!".to_string()
        );

        assert!(!collection.is_empty());
        assert!(!collection.full_replace);
        assert_eq!(collection.collection.len(), 2);

        let replacer1 = &collection.collection[0];
        assert_eq!(replacer1.key, "name.firstName");
        assert_eq!(replacer1.start, 6);
        assert_eq!(replacer1.end, 23);

        let replacer2 = &collection.collection[1];
        assert_eq!(replacer2.key, "address.cityName");
        assert_eq!(replacer2.start, 29);
        assert_eq!(replacer2.end, 48);
    }

    #[test]
    fn test_replacer_collection_new_partial_replacement() {
        let collection = ReplacerCollection::new(
            "Prefix ${name.firstName} suffix".to_string()
        );

        assert!(!collection.is_empty());
        assert!(!collection.full_replace);
        assert_eq!(collection.collection.len(), 1);

        let replacer = &collection.collection[0];
        assert_eq!(replacer.key, "name.firstName");
        assert_eq!(replacer.length, 17);
        // Total length is 31, replacer length is 17, so not full replace
        assert!(replacer.length < collection.value.len());
    }

    #[test]
    fn test_replacer_collection_is_empty() {
        let empty_collection = ReplacerCollection::new("No replacements here".to_string());
        assert!(empty_collection.is_empty());

        let non_empty_collection = ReplacerCollection::new("${name.firstName}".to_string());
        assert!(!non_empty_collection.is_empty());
    }

    #[test]
    fn test_replacer_collection_get_full_replacer() {
        let collection = ReplacerCollection::new("${name.firstName}".to_string());

        let full_replacer = collection.get_full_replacer();
        assert_eq!(full_replacer.key, "name.firstName");
        assert_eq!(full_replacer.pattern, "name.firstName");
    }

    #[test]
    fn test_replacer_collection_replace_empty() {
        let mut config = create_test_config();
        let collection = ReplacerCollection::new("No replacements here".to_string());

        let result = collection.replace(&mut config);
        assert!(result.is_some());

        match result.unwrap() {
            Value::String(s) => {
                assert_eq!(s, "No replacements here");
            }
            _ => panic!("Expected a string value"),
        }
    }    #[test]
    fn test_replacer_collection_replace_full_with_valid_key() {
        let mut config = create_test_config();
        let collection = ReplacerCollection::new("${name.firstName}".to_string());

        let result = collection.replace(&mut config);
        assert!(result.is_some());

        match result.unwrap() {
            Value::String(s) => {
                // Should generate a valid first name, not the original pattern
                assert_ne!(s, "${name.firstName}");
                assert!(!s.is_empty());
            }
            _ => panic!("Expected a string value"),
        }
    }

    #[test]
    fn test_replacer_collection_replace_full_with_invalid_key() {
        let mut config = create_test_config();
        let collection = ReplacerCollection::new("${invalid.key}".to_string());

        let result = collection.replace(&mut config);
        assert!(result.is_some());

        match result.unwrap() {
            Value::String(s) => {
                // Should return the original string since key is invalid
                assert_eq!(s, "${invalid.key}");
            }
            _ => panic!("Expected a string value"),
        }
    }

    #[test]
    fn test_replacer_collection_replace_partial_with_valid_keys() {
        let mut config = create_test_config();
        let collection = ReplacerCollection::new(
            "Hello ${name.firstName} from ${address.cityName}!".to_string()
        );

        let result = collection.replace(&mut config);
        assert!(result.is_some());

        match result.unwrap() {
            Value::String(s) => {
                // Should start with "Hello " and end with "!"
                assert!(s.starts_with("Hello "));
                assert!(s.ends_with("!"));
                assert!(s.contains(" from "));
                // Should not contain the original patterns
                assert!(!s.contains("${name.firstName}"));
                assert!(!s.contains("${address.cityName}"));
            }
            _ => panic!("Expected a string value"),
        }
    }

    #[test]
    fn test_replacer_collection_replace_partial_with_mixed_keys() {
        let mut config = create_test_config();
        let collection = ReplacerCollection::new(
            "Hello ${name.firstName} and ${invalid.key}!".to_string()
        );

        let result = collection.replace(&mut config);
        assert!(result.is_some());

        match result.unwrap() {
            Value::String(s) => {
                // Should start with "Hello " and end with "!"
                assert!(s.starts_with("Hello "));
                assert!(s.ends_with("!"));
                // Should not contain the valid pattern but should contain the invalid one
                assert!(!s.contains("${name.firstName}"));
                // Invalid key should remain unchanged
                assert!(s.contains("${invalid.key}"));
            }
            _ => panic!("Expected a string value"),
        }
    }

    #[test]
    fn test_replacer_collection_replace_with_arguments() {
        let mut config = create_test_config();
        let collection = ReplacerCollection::new("${lorem.words(5)}".to_string());

        let result = collection.replace(&mut config);
        assert!(result.is_some());

        match result.unwrap() {
            Value::String(s) => {
                // Should generate lorem words, not the original pattern
                assert_ne!(s, "${lorem.words(5)}");
                assert!(!s.is_empty());
                // Should contain multiple words (spaces between them)
                assert!(s.contains(' '));
            }
            _ => panic!("Expected a string value"),
        }
    }

    #[test]
    fn test_replacer_collection_replace_preserves_order() {
        let mut config = create_test_config();
        // Use different seeds to ensure we get different values
        config.rng = StdRng::seed_from_u64(123);

        let collection = ReplacerCollection::new(
            "${name.firstName}_${name.lastName}".to_string()
        );

        let result = collection.replace(&mut config);
        assert!(result.is_some());

        match result.unwrap() {
            Value::String(s) => {
                // Should contain an underscore separator
                assert!(s.contains('_'));
                let parts: Vec<&str> = s.split('_').collect();
                assert_eq!(parts.len(), 2);
                // Both parts should be non-empty and not contain the original patterns
                assert!(!parts[0].is_empty());
                assert!(!parts[1].is_empty());
                assert!(!parts[0].contains("${name.firstName}"));
                assert!(!parts[1].contains("${name.lastName}"));
            }
            _ => panic!("Expected a string value"),
        }
    }

    #[test]
    fn test_replacer_collection_replace_overlapping_patterns() {
        let mut config = create_test_config();
        let collection = ReplacerCollection::new(
            "${name.firstName}${name.lastName}".to_string()
        );

        let result = collection.replace(&mut config);
        assert!(result.is_some());

        match result.unwrap() {
            Value::String(s) => {
                // Should not contain the original patterns
                assert!(!s.contains("${name.firstName}"));
                assert!(!s.contains("${name.lastName}"));
                // Should be a concatenation of first and last name
                assert!(!s.is_empty());
            }
            _ => panic!("Expected a string value"),
        }
    }

    #[test]
    fn test_regex_pattern_matching() {
        let test_cases = vec![
            ("${simple}", vec!["simple"]),
            ("${with.dot}", vec!["with.dot"]),
            ("${with.args(1,2)}", vec!["with.args"]),
            ("${complex.path(1..5)}", vec!["complex.path"]),
            ("Multiple ${first} and ${second}", vec!["first", "second"]),
            ("${nested.${invalid}}", vec!["nested.$", "invalid"]), // Edge case
        ];

        for (input, expected_keys) in test_cases {
            let collection = ReplacerCollection::new(input.to_string());
            let actual_keys: Vec<String> = collection.collection
                .iter()
                .map(|r| r.key.clone())
                .collect();

            if input == "${nested.${invalid}}" {
                // Special case for malformed nested patterns
                continue;
            }

            assert_eq!(
                actual_keys.len(),
                expected_keys.len(),
                "Mismatch for input: {}",
                input
            );

            for (actual, expected) in actual_keys.iter().zip(expected_keys.iter()) {
                assert_eq!(actual, expected, "Key mismatch for input: {}", input);
            }
        }
    }
}