use std::collections::HashMap;

use rand::{rngs::StdRng, SeedableRng};
use serde_json::Value;

use crate::fake::{FakeGenerator, FakeKeys};

pub struct GeneratorConfig {
    pub locale: String,
    pub fake_keys: FakeKeys,
    pub fake_generator: FakeGenerator,
    pub rng: StdRng,
    pub store: HashMap<String, Vec<Value>>
}

impl GeneratorConfig {
    pub fn new(locale: &str, seed: Option<u64>) -> Self {
        let fake_keys = FakeKeys::new();
        let fake_generator = FakeGenerator::new(locale);
        let rng = StdRng::seed_from_u64(seed.unwrap_or(0));
        let store = HashMap::new();
        let locale = locale.to_string();

        Self {
            locale,
            fake_keys,
            fake_generator,
            rng,
            store,
        }
    }
}