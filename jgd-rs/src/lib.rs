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
    use std::{path::PathBuf, str::FromStr, sync::Arc};

    use serde_json::Value;

    use crate::{Arguments, Jgd};

    #[test]
    fn all_keys() {
        Jgd::add_custom_key("custom.key", Arc::new(|args: Arguments| {
            let value = match args {
                Arguments::None => "Empty Custom Key".to_string(),
                Arguments::Fixed(_) => format!("Fixed Custom Key with value {}", args.get_string("")),
                Arguments::Range(_, _) => format!("Range Custom Key with values {:?}", args.get_string_tuple("0", "1")),
            };

            Ok(Value::String(value))
        }));

        let result = Jgd::from_file(&PathBuf::from_str("../examples/user-post-entities-custom-keys.jgd").unwrap())
            .generate();

        assert!(result.is_ok());
    }
}
