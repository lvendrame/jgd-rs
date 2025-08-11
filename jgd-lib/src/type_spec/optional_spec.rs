use rand::Rng;
use serde::Deserialize;
use serde_json::Value;
use crate::type_spec::{Field, JsonGenerator};

fn default_prob() -> f64 {
    0.5
}

#[derive(Debug, Deserialize, Clone)]
pub struct OptionalSpec {
    pub of: Box<Field>,
    #[serde(default = "default_prob")]
    pub prob: f64
}

impl JsonGenerator for OptionalSpec {
    fn generate(&self, config: &mut super::GeneratorConfig) -> Value {
        if config.rng.random::<f64>() < self.prob {
            self.of.generate(config)
        } else {
            Value::Null
        }
    }
}
