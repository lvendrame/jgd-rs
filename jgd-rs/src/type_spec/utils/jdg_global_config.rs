use std::{collections::HashMap, sync::Arc};

use serde_json::Value;

use crate::{Arguments};

pub type CustomKeyFunction = Arc<dyn (Fn(Arguments) -> Result<Value, String>) + Send + Sync + 'static>;

#[derive(Default)]
pub struct JgdGlobalConfig {
    pub custom_keys: HashMap<&'static str, CustomKeyFunction>,
}

impl std::fmt::Debug for JgdGlobalConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JgdGlobalConfig")
            .field("custom_keys", &format!("HashMap with {} entries", self.custom_keys.len()))
            .finish()
    }
}

impl JgdGlobalConfig {
    pub fn new() -> Self {
        Self { custom_keys: HashMap::new() }
    }
}
