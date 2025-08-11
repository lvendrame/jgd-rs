use std::collections::BTreeMap;
use serde::Deserialize;
use serde_json::Value;
use crate::type_spec::{Entity, GeneratorConfig, JsonGenerator, RootEntity};

fn default_locale() -> String {
    "EN".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Jgd {
    #[serde(rename = "$format")]
    #[allow(dead_code)]
    pub format: String,
    #[allow(dead_code)]
    pub version: String,
    #[serde(default)]
    pub seed: Option<u64>,
    #[serde(default = "default_locale")]
    pub default_locale: String,
    #[serde(default)]
    pub entities: Option<BTreeMap<String, Entity>>, // collection mode
    #[serde(default)]
    pub root: Option<RootEntity>,                    // root mode
}

impl Jgd {
    pub fn generate(&self) -> Value {
        let mut config = GeneratorConfig::new(&self.default_locale, self.seed);

        if let Some(root) = &self.root {
            return root.generate(&mut config);
        }

        if let Some(entities) = &self.entities {
            return entities.generate(&mut config);
        }

        Value::Null
    }
}