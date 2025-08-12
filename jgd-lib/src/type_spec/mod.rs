mod array_spec;
mod count;
mod entity;
mod field;
mod jgd;
mod number_spec;
mod optional_spec;
mod utils;

// Re-export all types
pub use array_spec::ArraySpec;
pub use count::*;
pub use entity::Entity;
pub use field::Field;
pub use jgd::Jgd;
pub use number_spec::NumberSpec;
pub use optional_spec::OptionalSpec;
pub use utils::*;

use serde_json::Value;


pub trait JsonGenerator {
    fn generate(&self, config: &mut GeneratorConfig) -> Value;
}
