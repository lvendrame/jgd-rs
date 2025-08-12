use std::path::PathBuf;

use serde_json::Value;

pub use crate::type_spec::Jgd;

mod type_spec;
mod fake;
mod locales_keys;

pub fn generate_jgd_from_str(value: &str) -> Value {
    Jgd::from(value)
        .generate()
}

pub fn generate_jgd_from_file(path: &PathBuf) -> Value {
    Jgd::from_file(path)
        .generate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}
