use rand::Rng;
use serde::Deserialize;

use crate::type_spec::GeneratorConfig;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Count {
    Fixed(u64),
    Range((u64,u64))
}

pub trait GetCount {
    fn count(&self, config: &mut GeneratorConfig) -> u64;
}

impl GetCount for Count {
    fn count(&self, config: &mut GeneratorConfig) -> u64 {
        match self {
            Count::Fixed(n) => *n,
            Count::Range((a, b)) => config.rng.random_range(*a..=*b),
        }
    }
}

impl GetCount for Option<Count> {
    fn count(&self, config: &mut GeneratorConfig) -> u64 {
        self.clone().unwrap_or(Count::Fixed(1)).count(config)
    }
}
