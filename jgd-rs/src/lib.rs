use std::path::PathBuf;

use serde_json::Value;

pub use crate::type_spec::*;

mod type_spec;
mod fake;
mod locales_keys;

pub fn generate_jgd_from_str(value: &str) -> Result<Value, JgdGeneratorError> {
    Jgd::from(value)
        .generate()
}

pub fn generate_jgd_from_file(path: &PathBuf) -> Result<Value, JgdGeneratorError> {
    Jgd::from_file(path)
        .generate()
}

#[cfg(test)]
mod tests {
    #[test]
    fn dummy() {
    }
}
