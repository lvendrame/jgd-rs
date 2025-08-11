use std::collections::HashMap;

use rand::{random_range, rngs::StdRng, SeedableRng};
use serde_json::Value;

use crate::fake::{FakeGenerator, FakeKeys};

pub struct GeneratorConfig {
    pub locale: String,
    pub fake_keys: FakeKeys,
    pub fake_generator: FakeGenerator,
    pub rng: StdRng,
    pub gen_value: serde_json::Map<String, Value>,
}

impl GeneratorConfig {
    pub fn new(locale: &str, seed: Option<u64>) -> Self {
        let fake_keys = FakeKeys::new();
        let fake_generator = FakeGenerator::new(locale);
        let rng = StdRng::seed_from_u64(seed.unwrap_or(rand::random()));
        let locale = locale.to_string();

        Self {
            locale,
            fake_keys,
            fake_generator,
            rng,
            gen_value: serde_json::Map::new(),
        }
    }

    fn get_random_item_from_array<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
        if let Value::Array(items) = value {
            if let Some(Value::Object(obj)) = items.get(random_range(0..items.len())) {
                return obj.get(key);
            }
        }

        None
    }

    pub fn get_value_from_path(&self, path: String) -> Option<&Value> {
        let path = path.split(".");
        let mut current_value: Option<&Value> = None;
        let mut is_first = true;

        for key in path {
            let item = if is_first {
                is_first = false;
                self.gen_value.get(key)
            } else if let Some(value) = current_value {
                match value {
                    Value::Array(_) => Self::get_random_item_from_array(value, key),
                    Value::Object(map) => map.get(key),
                    _ => return None
                }
            } else {
                return None;
            };

            if let Some(part) = item {
                current_value = Some(part);
            } else {
                return None;
            }
        }

        current_value
    }
}