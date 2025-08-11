use rand::Rng;
use serde::Deserialize;
use serde_json::Value;

use crate::type_spec::JsonGenerator;

#[derive(Debug, Deserialize, Clone)]
pub struct NumberSpec {
    pub min: f64,
    pub max: f64,
    #[serde(default)]
    pub integer: bool
}

impl JsonGenerator for NumberSpec {
    fn generate(&self, config: &mut super::GeneratorConfig) -> Value {
        if self.integer {
            Value::from(config.rng.random_range(self.min as i64 ..= self.max as i64))
        } else {
            Value::from(config.rng.random_range(self.min..=self.max))
        }
    }
}