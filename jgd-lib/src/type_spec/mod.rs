mod array_spec;
mod count;
mod entity;
mod field;
mod jgd;
mod len;
mod number_spec;
mod optional_spec;
mod root_entity;
mod utils;

// Re-export all types
pub use array_spec::ArraySpec;
pub use count::*;
pub use entity::Entity;
pub use field::Field;
pub use jgd::Jgd;
pub use len::*;
pub use number_spec::NumberSpec;
pub use optional_spec::OptionalSpec;
pub use root_entity::RootEntity;
pub use utils::*;

use serde_json::Value;


trait JsonGenerator {
    fn generate(&self, config: &mut GeneratorConfig) -> Value;
}
