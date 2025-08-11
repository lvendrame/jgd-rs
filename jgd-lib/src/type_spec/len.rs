use rand::Rng;
use serde::Deserialize;

use crate::type_spec::GeneratorConfig;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Len {
    Fixed(u64),
    Range((u64,u64))
}

pub trait GetLength {
    fn len(&self, config: &mut GeneratorConfig) -> u64;
}

impl GetLength for Len {
    fn len(&self, config: &mut GeneratorConfig) -> u64 {
        match self {
            Len::Fixed(n) => *n,
            Len::Range((a, b)) => config.rng.random_range(*a..=*b),
        }
    }
}

impl GetLength for Option<Len> {
    fn len(&self, config: &mut GeneratorConfig) -> u64 {
        self.clone().unwrap_or(Len::Fixed(1)).len(config)
    }
}
