use rand::Rng;
use serde::Deserialize;
use serde_json::Value;
use crate::type_spec::{len::GetLength, Field, JsonGenerator, Len};

#[derive(Debug, Deserialize, Clone)]
pub struct ArraySpec {
    pub of: Box<Field>,
    #[serde(default)]
    pub length: Option<Len>
}

impl JsonGenerator for ArraySpec {
    fn generate(&self, config: &mut super::GeneratorConfig) -> Value {
        let len = self.length.len(config);
        let mut arr = Vec::with_capacity(len as usize);
        for _ in 0..len {
            arr.push(self.of.generate(config));
        }

        Value::Array(arr)
    }
}