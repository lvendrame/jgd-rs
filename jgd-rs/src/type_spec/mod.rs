//! Type specifications for JGD (JSON Generator Definition) schema.
//!
//! This module contains all the type specifications that define how different types
//! of data should be generated according to the JGD schema specification. It provides
//! a comprehensive set of building blocks for creating fake JSON data generators.
//!
//! # Overview
//!
//! The type specification system is built around the [`JsonGenerator`] trait, which
//! defines a common interface for all data generators. Each type specification
//! corresponds to a different field type in the JGD schema:
//!
//! - [`NumberSpec`] - Generates random numbers (integers or floats) within a range
//! - [`ArraySpec`] - Generates arrays of elements with specified count and element types
//! - [`Entity`] - Generates complex objects with multiple fields
//! - [`Field`] - Represents individual fields within entities
//! - [`OptionalSpec`] - Wraps other specifications to make them optionally null
//! - [`Count`] - Defines how many items should be generated (fixed or range)
//!
//! # JGD Schema Compliance
//!
//! All type specifications in this module are designed to work with JGD schema
//! definitions, ensuring compatibility with the JSON Generator Definition standard.
//! The generators produce output that conforms to the expected schema structure.
//!
//! # Examples
//!
//! ```rust,ignore
//! use jgd_rs::{NumberSpec, JsonGenerator, GeneratorConfig};
//!
//! let mut config = GeneratorConfig::new("EN", Some(42));
//! let number_spec = NumberSpec::new_integer(1.0, 100.0);
//! let generated_value = number_spec.generate(&mut config);
//! ```

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

/// Core trait for all JSON data generators in the JGD system.
///
/// This trait defines the common interface that all type specifications must implement
/// to generate JSON values according to their specific rules. Each implementation
/// should produce values that conform to the JGD schema specification.
///
/// # Design Philosophy
///
/// The `JsonGenerator` trait provides a unified way to generate JSON data from
/// different type specifications. It takes a mutable reference to a `GeneratorConfig`
/// which contains the random number generator, locale settings, and other configuration
/// needed for data generation.
///
/// # Arguments
///
/// * `config` - A mutable reference to the generator configuration containing:
///   - Random number generator for deterministic or random generation
///   - Locale settings for locale-specific fake data
///   - Other generation context and settings
///
/// # Returns
///
/// A `serde_json::Value` representing the generated data. The specific type of
/// JSON value (number, string, array, object, etc.) depends on the implementing
/// type specification.
///
/// # Examples
///
/// ```rust,ignore
/// use jgd_rs::{JsonGenerator, NumberSpec, GeneratorConfig};
/// use serde_json::Value;
///
/// let mut config = GeneratorConfig::new("EN", Some(42));
/// let spec = NumberSpec::new_integer(1.0, 10.0);
///
/// let generated: Value = spec.generate(&mut config);
/// // generated will be a JSON number between 1 and 10
/// ```
///
/// # Implementation Guidelines
///
/// When implementing this trait:
/// - Ensure generated values conform to the JGD schema specification
/// - Use the provided `GeneratorConfig` for all randomization
/// - Handle edge cases gracefully (e.g., invalid ranges, empty arrays)
/// - Consider performance implications for large data generation
pub trait JsonGenerator {
    /// Generates a JSON value according to the type specification.
    ///
    /// This method should produce a `serde_json::Value` that conforms to the
    /// JGD schema specification for the implementing type. The generation
    /// should use the provided configuration for randomization and locale settings.
    ///
    /// # Arguments
    ///
    /// * `config` - Mutable reference to the generator configuration
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` containing the generated data
    fn generate(&self, config: &mut GeneratorConfig, local_config: Option<&mut LocalConfig>) -> ResultValue;
}

pub type ResultValue = Result<Value, JgdGeneratorError>;
