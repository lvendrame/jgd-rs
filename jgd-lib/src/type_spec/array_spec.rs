use serde::Deserialize;
use serde_json::Value;
use crate::type_spec::{Count, Field, GetCount, JsonGenerator};

#[derive(Debug, Deserialize, Clone)]
pub struct ArraySpec {
    pub of: Box<Field>,
    #[serde(default)]
    pub count: Option<Count>
}

impl JsonGenerator for ArraySpec {
    fn generate(&self, config: &mut super::GeneratorConfig) -> Value {
        let len = self.count.count(config);
        let mut arr = Vec::with_capacity(len as usize);
        for _ in 0..len {
            arr.push(self.of.generate(config));
        }

        Value::Array(arr)
    }
}
