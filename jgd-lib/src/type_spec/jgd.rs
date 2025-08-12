use std::{fs, path::PathBuf};

use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::Value;
use crate::type_spec::{Entity, GeneratorConfig, JsonGenerator};

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
    pub entities: Option<IndexMap<String, Entity>>, // collection mode
    #[serde(default)]
    pub root: Option<Entity>, // root mode
}

impl Jgd {
    pub fn from_file(path: &PathBuf) -> Self {
        let jgd_string = fs::read_to_string(path);

        Self::from(jgd_string.unwrap())
    }

    pub fn create_config(&self) -> GeneratorConfig {
        GeneratorConfig::new(&self.default_locale, self.seed)
    }

    pub fn generate(&self) -> Value {
        let mut config = self.create_config();

        if let Some(root) = &self.root {
            return root.generate(&mut config);
        }

        if let Some(entities) = &self.entities {
            return entities.generate(&mut config);
        }

        Value::Null
    }
}

impl From<&str> for Jgd {
    fn from(value: &str) -> Self {
        serde_json::from_str(value).unwrap()
    }
}

impl From<String> for Jgd {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap()
    }
}

impl From<Value> for Jgd {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}
