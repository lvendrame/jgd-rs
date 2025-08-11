use std::collections::BTreeMap;
use serde::Deserialize;
use serde_json::Value;
use crate::type_spec::{ArraySpec, GeneratorConfig, JsonGenerator, NumberSpec, OptionalSpec, ReplacerCollection};

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Field {
    Object {
        object: BTreeMap<String, Field>
    },
    Array  {
        array: ArraySpec
    },
    Number {
        number: NumberSpec
    },
    Optional {
        optional: OptionalSpec
    },
    Ref {
        r#ref: String
    },
    Str(String),
    Bool(bool),
    I64(i64),
    F64(f64),
    Null,
}

impl Field {

    fn generate_for_ref(&self, r#ref: &str, config: &mut GeneratorConfig) -> Value {
        let value = config.get_value_from_path(r#ref.to_string());

        if let Some(value) = value {
            return value.clone();
        }

        Value::String(format!("The path {} is not found", r#ref))
    }
}

impl JsonGenerator for Field {
    fn generate(&self, config: &mut GeneratorConfig) -> serde_json::Value {
        match self {
            Field::Object { object } => object.generate(config),
            Field::Array { array } => array.generate(config),
            Field::Number { number } => number.generate(config),
            Field::Optional { optional } => optional.generate(config),
            Field::Ref { r#ref } => self.generate_for_ref(r#ref, config),
            Field::Str(value) => value.generate(config),
            Field::Bool(value) => Value::Bool(*value),
            Field::I64(value) => Value::Number(serde_json::Number::from(*value)),
            Field::F64(value) => Value::Number(serde_json::Number::from_f64(*value).unwrap()),
            Field::Null => Value::Null,
        }
    }
}

impl JsonGenerator for BTreeMap<String, Field> {
    fn generate(&self, config: &mut GeneratorConfig) -> Value {
       let mut map = serde_json::Map::new();
        for (key, field) in self {
            map.insert(key.clone(), field.generate(config));
        }

        Value::Object(map)
    }
}

impl JsonGenerator for String {
    fn generate(&self, config: &mut GeneratorConfig) -> Value {
        let value = self.to_string();
        let replacers = ReplacerCollection::new(value.clone());
        if replacers.is_empty() {
            return Value::String(value);
        }

        replacers.replace(config).unwrap_or(
            Value::String(value)
        )
    }
}