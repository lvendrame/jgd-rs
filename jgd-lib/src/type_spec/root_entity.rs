use std::collections::{BTreeMap, HashMap, HashSet};
use serde::Deserialize;
use serde_json::Value;
use crate::type_spec::{Count, Field, GetCount, JsonGenerator};

#[derive(Debug, Deserialize)]
pub struct RootEntity {
    #[serde(default)]
    pub count: Option<Count>,
    #[serde(default)]
    pub seed: Option<u64>,
    #[serde(default)]
    pub unique_by: Vec<String>,
    pub fields: BTreeMap<String, Field>,
}

impl JsonGenerator for RootEntity {
    fn generate(&self, config: &mut super::GeneratorConfig) -> Value {
        let mut items = Vec::new();
        let count = self.count.count(config);

        //let mut unique_sets: HashMap<String, HashSet<String>> = HashMap::new();
        for _ in 0..count {
            let obj = self.fields.generate(config);
            // if !self.unique_by.is_empty() {
            //     let fp = fingerprint(&obj, &root.unique_by);
            //     let set = unique_sets.entry(root.unique_by.join("|"))
            //         .or_default();
            //     if set.contains(&fp) {
            //         continue;
            //     } else {
            //         set.insert(fp);
            //     }
            // }
            if self.count.is_none() {
                return obj;
            }
            items.push(obj);
        }

        Value::Array(items)
    }
}