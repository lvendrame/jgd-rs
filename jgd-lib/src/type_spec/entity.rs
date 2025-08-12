use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::Value;
use crate::type_spec::{Count, Field, GetCount, JsonGenerator};

#[derive(Debug, Deserialize, Clone)]
pub struct Entity {
    pub count: Option<Count>,
    #[serde(default)]
    pub seed: Option<u64>,
    #[serde(default)]
    pub unique_by: Vec<String>,
    pub fields: IndexMap<String, Field>,
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

            if self.count.is_none() {
                return obj;
            }

            items.push(obj);
        }

        Value::Array(items)
    }
}

impl JsonGenerator for IndexMap<String, Entity> {
    fn generate(&self, config: &mut super::GeneratorConfig) -> serde_json::Value {

        let mut map = serde_json::Map::new();
        for (name, entity) in self {
            let generated = entity.generate(config);
            map.insert(name.clone(), generated.clone());

            config.gen_value.insert(name.clone(), generated);
        }

        Value::Object(map)
    }
}
