use std::collections::BTreeMap;
use serde::Deserialize;
use serde_json::Value;
use crate::type_spec::{Count, Field, GetCount, JsonGenerator};

#[derive(Debug, Deserialize)]
pub struct Entity {
    pub count: Count,
    #[serde(default)]
    pub seed: Option<u64>,
    #[serde(default)]
    pub unique_by: Vec<String>,
    pub fields: BTreeMap<String, Field>,
}

impl JsonGenerator for Entity {
    fn generate(&self, config: &mut super::GeneratorConfig) -> Value {
        let count = self.count.count(config);

        let mut items = Vec::with_capacity(count as usize);
        // let mut unique_sets: HashMap<String, HashSet<String>> = HashMap::new();

        for _ in 0..count {
            let obj = self.fields.generate(config);
            // if !ent.unique_by.is_empty() {
            //     let fp = fingerprint(&obj, &ent.unique_by);
            //     let set = unique_sets.entry(ent.unique_by.join("|"))
            //         .or_default();
            //     if set.contains(&fp) {
            //         continue;
            //     } else {
            //         set.insert(fp);
            //     }
            // }
            items.push(obj);
        }

        Value::Array(items)
    }
}

impl JsonGenerator for BTreeMap<String, Entity> {
    fn generate(&self, config: &mut super::GeneratorConfig) -> serde_json::Value {

        let mut map = serde_json::Map::new();
        for (name, entity) in self {
            map.insert(name.clone(), entity.generate(config));
        }

        Value::Object(map)
    }
}
