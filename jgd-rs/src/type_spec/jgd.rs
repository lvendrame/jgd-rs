//! # JGD (JSON Generator Definition) Module
//!
//! This module provides the core `Jgd` struct that represents a complete JGD schema definition.
//! JGD is a declarative format for defining JSON data generation rules, supporting both simple
//! root-based schemas and complex multi-entity schemas with cross-references.
//!
//! ## Overview
//!
//! The JGD format enables you to define:
//! - **Schema metadata**: Format version, seed values, and locale preferences
//! - **Entity definitions**: Structured data generators with fields and constraints
//! - **Generation modes**: Root-based (single entity) or entities-based (multiple named entities)
//! - **Cross-references**: Inter-entity relationships and data dependencies
//!
//! ## Schema Modes
//!
//! JGD supports two mutually exclusive schema modes:
//!
//! ### Root Mode
//! Single entity generation where the entire schema defines one entity structure:
//! ```json
//! {
//!   "$format": "jgd/v1",
//!   "version": "1.0.0",
//!   "root": {
//!     "fields": {
//!       "name": "${name.firstName}",
//!       "age": { "number": { "min": 18, "max": 65, "integer": true } }
//!     }
//!   }
//! }
//! ```
//!
//! ### Entities Mode
//! Multiple named entity definitions for complex relational data:
//! ```json
//! {
//!   "$format": "jgd/v1",
//!   "version": "1.0.0",
//!   "entities": {
//!     "users": {
//!       "count": { "fixed": 5 },
//!       "fields": {
//!         "id": { "number": { "min": 1, "max": 1000, "integer": true } },
//!         "name": "${name.fullName}"
//!       }
//!     },
//!     "posts": {
//!       "count": { "range": [1, 3] },
//!       "fields": {
//!         "userId": { "ref": "users.id" },
//!         "title": "${lorem.sentence}"
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! ## Basic Usage
//!
//! ### Loading from File
//! ```rust
//! # use jgd_rs::Jgd;
//! # use std::fs;
//! # use std::path::PathBuf;
//! # let schema_content = r#"{"$format": "jgd/v1", "version": "1.0", "root": {"fields": {"test": "value"}}}"#;
//! # fs::write("/tmp/test_schema.jgd", schema_content).unwrap();
//! let jgd = Jgd::from_file(&PathBuf::from("/tmp/test_schema.jgd"));
//! let data = jgd.generate();
//! # fs::remove_file("/tmp/test_schema.jgd").ok();
//! ```
//!
//! ### Loading from String
//! ```rust
//! # use jgd_rs::Jgd;
//! let schema = r#"{"$format": "jgd/v1", "version": "1.0", "root": {"fields": {"name": "Test"}}}"#;
//! let jgd = Jgd::from(schema);
//! let data = jgd.generate();
//! ```
//!
//! ### Converting Schema to Generation Config
//! ```rust
//! # use jgd_rs::Jgd;
//! let jgd = Jgd::from(r#"{"$format": "jgd/v1", "version": "1.0", "seed": 42, "defaultLocale": "FR", "root": {"fields": {}}}"#);
//! let config = jgd.create_config();
//! // Config now contains locale "FR" and seed 42
//! ```

use std::{fs, path::PathBuf, sync::{LazyLock, Mutex}};

use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::Value;
use crate::{type_spec::{Entity, GeneratorConfig, JsonGenerator}, CustomKeyFunction, JgdGlobalConfig};

/// Default locale for data generation when no locale is specified.
fn default_locale() -> String {
    "EN".to_string()
}

/// Core JGD schema representation containing all schema metadata and entity definitions.
///
/// The `Jgd` struct represents a complete JSON Generator Definition schema that can be loaded
/// from JSON files or strings. It supports two mutually exclusive generation modes:
/// - **Root mode**: Single entity definition in the `root` field
/// - **Entities mode**: Multiple named entities in the `entities` collection
///
/// # Schema Structure
///
/// A JGD schema requires:
/// - `$format`: Schema format version (e.g., "jgd/v1")
/// - `version`: User-defined schema version string
/// - Either `root` OR `entities` (mutually exclusive)
///
/// Optional fields:
/// - `seed`: Random seed for deterministic generation
/// - `defaultLocale`: Locale code for fake data generation (defaults to "EN")
///
/// # Examples
///
/// ## Root Mode Schema
/// ```rust
/// # use jgd_rs::Jgd;
/// let schema = r#"{
///   "$format": "jgd/v1",
///   "version": "1.0.0",
///   "seed": 42,
///   "root": {
///     "fields": {
///       "name": "John Doe",
///       "age": 30
///     }
///   }
/// }"#;
/// let jgd = Jgd::from(schema);
/// ```
///
/// ## Entities Mode Schema
/// ```rust
/// # use jgd_rs::Jgd;
/// let schema = r#"{
///   "$format": "jgd/v1",
///   "version": "2.0.0",
///   "entities": {
///     "users": {
///       "fields": {
///         "id": 1,
///         "name": "Alice"
///       }
///     }
///   }
/// }"#;
/// let jgd = Jgd::from(schema);
/// ```
#[derive(Debug, Deserialize)]
pub struct Jgd {
    /// Schema format identifier (e.g., "jgd/v1").
    ///
    /// This field identifies the JGD format version and is used for schema validation
    /// and compatibility checking. The `#[serde(rename = "$format")]` attribute maps
    /// it to the JSON `$format` field.
    #[serde(rename = "$format")]
    #[allow(dead_code)]
    pub format: String,

    /// User-defined schema version string.
    ///
    /// This field allows schema authors to version their JGD definitions
    /// for tracking changes and compatibility management.
    #[allow(dead_code)]
    pub version: String,

    /// Optional random seed for deterministic generation.
    ///
    /// When provided, this seed ensures reproducible data generation across multiple
    /// executions. When `None`, generation uses non-deterministic randomness.
    #[serde(default)]
    pub seed: Option<u64>,

    /// Default locale for fake data generation.
    ///
    /// Specifies the locale code (e.g., "EN", "FR", "DE") used for generating
    /// locale-specific fake data. Defaults to "EN" when not specified.
    #[serde(default = "default_locale", rename = "defaultLocale")]
    pub default_locale: String,

    /// Named entity definitions for entities mode (mutually exclusive with `root`).
    ///
    /// When present, the schema operates in entities mode where multiple named
    /// entities are generated. Each key represents an entity name, and the value
    /// contains the entity definition with its fields and generation rules.
    #[serde(default)]
    pub entities: Option<IndexMap<String, Entity>>,

    /// Root entity definition for root mode (mutually exclusive with `entities`).
    ///
    /// When present, the schema operates in root mode where a single entity
    /// structure is generated. The entity definition contains fields and
    /// generation rules applied to the root level.
    #[serde(default)]
    pub root: Option<Entity>,
}

static GLOBAL_CONFIG: LazyLock<Mutex<JgdGlobalConfig>> = LazyLock::new(|| Mutex::new(JgdGlobalConfig::new()));

impl Jgd {

    /// Loads a JGD schema from a file path.
    ///
    /// Reads the specified file and parses its JSON content into a `Jgd` struct.
    /// The file should contain a valid JGD schema in JSON format.
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the JGD schema file to load
    ///
    /// # Returns
    ///
    /// Returns a `Jgd` instance containing the parsed schema.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - The file cannot be read (e.g., file not found, permission denied)
    /// - The file content is not valid UTF-8
    /// - The JSON content cannot be parsed into a valid JGD schema
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use jgd_rs::Jgd;
    /// # use std::fs;
    /// # use std::path::PathBuf;
    /// # let schema_content = r#"{"$format": "jgd/v1", "version": "1.0", "root": {"fields": {"test": "value"}}}"#;
    /// # fs::write("/tmp/test_schema.jgd", schema_content).unwrap();
    /// let jgd = Jgd::from_file(&PathBuf::from("/tmp/test_schema.jgd"));
    /// # fs::remove_file("/tmp/test_schema.jgd").ok();
    /// ```
    pub fn from_file(path: &PathBuf) -> Self {
        let jgd_string = fs::read_to_string(path);

        Self::from(jgd_string.unwrap())
    }

    /// Creates a generation configuration from this JGD schema.
    ///
    /// Builds a `GeneratorConfig` instance using the schema's locale and seed settings.
    /// This configuration is used internally for data generation and can be customized
    /// with the schema's `defaultLocale` and `seed` values.
    ///
    /// # Returns
    ///
    /// Returns a `GeneratorConfig` configured with:
    /// - The schema's `defaultLocale` (or "EN" if not specified)
    /// - The schema's `seed` value (or `None` for non-deterministic generation)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use jgd_rs::Jgd;
    /// let jgd = Jgd::from(r#"{
    ///   "$format": "jgd/v1",
    ///   "version": "1.0",
    ///   "seed": 42,
    ///   "defaultLocale": "FR",
    ///   "root": {"fields": {}}
    /// }"#);
    /// let config = jgd.create_config();
    /// // Config now uses French locale and seed 42
    /// ```
    pub fn create_config(&self) -> GeneratorConfig {
        GeneratorConfig::new(&self.default_locale, self.seed)
    }

    /// Generates JSON data according to the schema definition.
    ///
    /// Executes the schema's generation rules and produces JSON data. The generation
    /// behavior depends on the schema mode:
    ///
    /// - **Root mode**: Generates a single JSON object or array based on the root entity
    /// - **Entities mode**: Generates a JSON object with named entity collections
    /// - **Empty schema**: Returns `Value::Null` when neither root nor entities are defined
    ///
    /// # Returns
    ///
    /// Returns a `serde_json::Value` containing the generated data:
    /// - Root mode: Object or array depending on count specification
    /// - Entities mode: Object with entity names as keys and generated data as values
    /// - Empty schema: `Value::Null`
    ///
    /// # Examples
    ///
    /// ## Root Mode Generation
    /// ```rust
    /// # use jgd_rs::Jgd;
    /// # use serde_json::Value;
    /// let jgd = Jgd::from(r#"{
    ///   "$format": "jgd/v1",
    ///   "version": "1.0",
    ///   "root": {
    ///     "fields": {
    ///       "name": "Test",
    ///       "value": 42
    ///     }
    ///   }
    /// }"#);
    /// let result = jgd.generate();
    /// // Returns: {"name": "Test", "value": 42}
    /// ```
    ///
    /// ## Entities Mode Generation
    /// ```rust
    /// # use jgd_rs::Jgd;
    /// # use serde_json::Value;
    /// let jgd = Jgd::from(r#"{
    ///   "$format": "jgd/v1",
    ///   "version": "1.0",
    ///   "entities": {
    ///     "users": {
    ///       "fields": {"name": "Alice"}
    ///     },
    ///     "posts": {
    ///       "fields": {"title": "Post"}
    ///     }
    ///   }
    /// }"#);
    /// let result = jgd.generate();
    /// // Returns: {"users": {"name": "Alice"}, "posts": {"title": "Post"}}
    /// ```
    pub fn generate(&self) -> Value {
        let mut config = self.create_config();

        if let Some(root) = &self.root {
            return root.generate(&mut config);
        }

        if let Some(entities) = &self.entities {
            return entities.generate(&mut config);
        }

        Value::Null
    }

    /// Adds a custom key function to the global configuration.
    ///
    /// This method allows you to register custom faker patterns that can be used
    /// in JGD schemas. The function will be available globally across all JGD
    /// instances and is thread-safe.
    ///
    /// # Parameters
    ///
    /// * `key` - A static string reference that identifies the custom pattern
    /// * `func` - A function that takes `Arguments` and returns a `Result<Value, JgdGeneratorError>`
    ///
    /// # Thread Safety
    ///
    /// This method is thread-safe and uses a mutex to ensure that custom keys
    /// can be safely added from multiple threads. If the mutex is poisoned,
    /// the method will silently fail to add the key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use jgd_rs::{Jgd, Arguments};
    /// # use serde_json::Value;
    /// # use jgd_rs::JgdGeneratorError;
    /// Jgd::add_custom_key("custom.greeting", Box::new(|args| {
    ///     let name = args.get_string("World");
    ///     Ok(Value::String(format!("Hello, {}!", name)))
    /// }));
    /// ```
    pub fn add_custom_key(key: &'static str, func: CustomKeyFunction) {
        if let Ok(mut config) = GLOBAL_CONFIG.lock() {
            config.custom_keys.insert(key, func);
        }
    }

    pub fn get_custom_key(key: &'static str) -> Option<CustomKeyFunction> {
        if let Ok(config) = GLOBAL_CONFIG.lock() {
            if let Some(func) = config.custom_keys.get(key) {
                return Some(func.clone());
            }
        }
        None
    }
}

/// Implements conversion from string slice to `Jgd`.
///
/// Parses a JSON string containing a JGD schema into a `Jgd` struct.
/// This is the primary method for loading JGD schemas from JSON text.
///
/// # Parameters
///
/// * `value` - JSON string slice containing the JGD schema
///
/// # Returns
///
/// Returns a `Jgd` instance containing the parsed schema.
///
/// # Panics
///
/// Panics if the JSON string cannot be parsed into a valid JGD schema.
/// This includes invalid JSON syntax, missing required fields, or invalid field types.
///
/// # Examples
///
/// ```rust
/// # use jgd_rs::Jgd;
/// let schema_str = r#"{
///   "$format": "jgd/v1",
///   "version": "1.0.0",
///   "root": {
///     "fields": {
///       "message": "Hello, World!"
///     }
///   }
/// }"#;
/// let jgd = Jgd::from(schema_str);
/// ```
impl From<&str> for Jgd {
    fn from(value: &str) -> Self {
        serde_json::from_str(value).unwrap()
    }
}

/// Implements conversion from owned String to `Jgd`.
///
/// Parses a JSON string containing a JGD schema into a `Jgd` struct.
/// This method consumes the input string and is useful when working with
/// owned string data.
///
/// # Parameters
///
/// * `value` - Owned JSON string containing the JGD schema
///
/// # Returns
///
/// Returns a `Jgd` instance containing the parsed schema.
///
/// # Panics
///
/// Panics if the JSON string cannot be parsed into a valid JGD schema.
/// This includes invalid JSON syntax, missing required fields, or invalid field types.
///
/// # Examples
///
/// ```rust
/// # use jgd_rs::Jgd;
/// let schema_string = String::from(r#"{
///   "$format": "jgd/v1",
///   "version": "1.0.0",
///   "root": {
///     "fields": {
///       "id": 123,
///       "active": true
///     }
///   }
/// }"#);
/// let jgd = Jgd::from(schema_string);
/// ```
impl From<String> for Jgd {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap()
    }
}

/// Implements conversion from `serde_json::Value` to `Jgd`.
///
/// Converts a parsed JSON value into a `Jgd` struct. This is useful when
/// working with pre-parsed JSON data or when the schema is constructed
/// programmatically using serde_json macros.
///
/// # Parameters
///
/// * `value` - `serde_json::Value` containing the JGD schema structure
///
/// # Returns
///
/// Returns a `Jgd` instance containing the parsed schema.
///
/// # Panics
///
/// Panics if the JSON value structure cannot be converted into a valid JGD schema.
/// This includes missing required fields, invalid field types, or structural mismatches.
///
/// # Examples
///
/// ```rust
/// # use jgd_rs::Jgd;
/// # use serde_json::{json, Value};
/// let schema_value = json!({
///   "$format": "jgd/v1",
///   "version": "1.0.0",
///   "seed": 42,
///   "defaultLocale": "EN",
///   "root": {
///     "fields": {
///       "name": "Sample",
///       "count": 10
///     }
///   }
/// });
/// let jgd = Jgd::from(schema_value);
/// ```
impl From<Value> for Jgd {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::Arguments;

    use super::*;
    use serde::de::value;
    use serde_json::json;

    #[test]
    fn test_default_locale_function() {
        assert_eq!(default_locale(), "EN");
    }

    #[test]
    fn test_jgd_from_str_root_mode() {
        let schema_json = r#"{
            "$format": "jgd/v1",
            "version": "1.0.0",
            "seed": 42,
            "defaultLocale": "EN",
            "root": {
                "fields": {
                    "name": "John Doe",
                    "age": 30
                }
            }
        }"#;

        let jgd = Jgd::from(schema_json);

        assert_eq!(jgd.format, "jgd/v1");
        assert_eq!(jgd.version, "1.0.0");
        assert_eq!(jgd.seed, Some(42));
        assert_eq!(jgd.default_locale, "EN");
        assert!(jgd.root.is_some());
        assert!(jgd.entities.is_none());
    }

    #[test]
    fn test_jgd_from_str_entities_mode() {
        let schema_json = r#"{
            "$format": "jgd/v1",
            "version": "2.0.0",
            "entities": {
                "users": {
                    "fields": {
                        "name": "Alice"
                    }
                },
                "posts": {
                    "fields": {
                        "title": "Test Post"
                    }
                }
            }
        }"#;

        let jgd = Jgd::from(schema_json);

        assert_eq!(jgd.format, "jgd/v1");
        assert_eq!(jgd.version, "2.0.0");
        assert_eq!(jgd.seed, None);
        assert_eq!(jgd.default_locale, "EN"); // Default value
        assert!(jgd.root.is_none());
        assert!(jgd.entities.is_some());

        let entities = jgd.entities.unwrap();
        assert!(entities.contains_key("users"));
        assert!(entities.contains_key("posts"));
    }

    #[test]
    fn test_jgd_from_string() {
        let schema_json = String::from(r#"{
            "$format": "jgd/v1",
            "version": "1.0.0",
            "root": {
                "fields": {
                    "message": "Hello"
                }
            }
        }"#);

        let jgd = Jgd::from(schema_json);
        assert_eq!(jgd.format, "jgd/v1");
        assert!(jgd.root.is_some());
    }

    #[test]
    fn test_jgd_from_value() {
        let schema_value = json!({
            "$format": "jgd/v1",
            "version": "1.0.0",
            "seed": 123,
            "defaultLocale": "FR",
            "root": {
                "fields": {
                    "greeting": "Bonjour"
                }
            }
        });

        let jgd = Jgd::from(schema_value);
        assert_eq!(jgd.format, "jgd/v1");
        assert_eq!(jgd.seed, Some(123));
        assert_eq!(jgd.default_locale, "FR");
        assert!(jgd.root.is_some());
    }

    #[test]
    fn test_jgd_default_values() {
        let schema_json = r#"{
            "$format": "jgd/v1",
            "version": "1.0",
            "root": {
                "fields": {
                    "test": "value"
                }
            }
        }"#;

        let jgd = Jgd::from(schema_json);

        // Test default values
        assert_eq!(jgd.seed, None);
        assert_eq!(jgd.default_locale, "EN");
        assert!(jgd.entities.is_none());
    }

    #[test]
    fn test_create_config() {
        let jgd = Jgd::from(r#"{
            "$format": "jgd/v1",
            "version": "1.0.0",
            "seed": 42,
            "defaultLocale": "ES",
            "root": {
                "fields": {
                    "test": "value"
                }
            }
        }"#);

        let _config = jgd.create_config();

        // We can't directly test the internal values of GeneratorConfig,
        // but we can test that it's created successfully
        // The actual locale and seed testing would be done in GeneratorConfig tests
    }

    #[test]
    fn test_generate_root_mode() {
        let jgd = Jgd::from(r#"{
            "$format": "jgd/v1",
            "version": "1.0.0",
            "seed": 42,
            "root": {
                "fields": {
                    "name": "John Doe",
                    "age": 30,
                    "active": true
                }
            }
        }"#);

        let result = jgd.generate();

        assert!(result.is_object());
        if let Value::Object(obj) = result {
            assert_eq!(obj.get("name"), Some(&Value::String("John Doe".to_string())));
            assert_eq!(obj.get("age"), Some(&Value::Number(serde_json::Number::from(30))));
            assert_eq!(obj.get("active"), Some(&Value::Bool(true)));
        }
    }

    #[test]
    fn test_generate_entities_mode() {
        let jgd = Jgd::from(r#"{
            "$format": "jgd/v1",
            "version": "1.0.0",
            "seed": 42,
            "entities": {
                "users": {
                    "fields": {
                        "name": "Alice"
                    }
                },
                "products": {
                    "fields": {
                        "title": "Product A"
                    }
                }
            }
        }"#);

        let result = jgd.generate();

        assert!(result.is_object());
        if let Value::Object(obj) = result {
            assert!(obj.contains_key("users"));
            assert!(obj.contains_key("products"));

            // Check users entity
            if let Some(Value::Object(users)) = obj.get("users") {
                assert_eq!(users.get("name"), Some(&Value::String("Alice".to_string())));
            }

            // Check products entity
            if let Some(Value::Object(products)) = obj.get("products") {
                assert_eq!(products.get("title"), Some(&Value::String("Product A".to_string())));
            }
        }
    }

    #[test]
    fn test_generate_empty_schema() {
        let jgd = Jgd::from(r#"{
            "$format": "jgd/v1",
            "version": "1.0"
        }"#);

        let result = jgd.generate();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_generate_deterministic_with_seed() {
        let schema = r#"{
            "$format": "jgd/v1",
            "version": "1.0.0",
            "seed": 42,
            "root": {
                "fields": {
                    "random_number": {
                        "number": {
                            "min": 1,
                            "max": 100,
                            "integer": true
                        }
                    }
                }
            }
        }"#;

        let jgd1 = Jgd::from(schema);
        let jgd2 = Jgd::from(schema);

        let result1 = jgd1.generate();
        let result2 = jgd2.generate();

        // With same seed, results should be identical
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_generate_different_seeds_different_results() {
        let schema_template = |seed: u64| format!(r#"{{
            "$format": "jgd/v1",
            "version": "1.0.0",
            "seed": {},
            "root": {{
                "fields": {{
                    "random_number": {{
                        "number": {{
                            "min": 1,
                            "max": 1000,
                            "integer": true
                        }}
                    }}
                }}
            }}
        }}"#, seed);

        let jgd1 = Jgd::from(schema_template(42).as_str());
        let jgd2 = Jgd::from(schema_template(24).as_str());

        let result1 = jgd1.generate();
        let result2 = jgd2.generate();

        // Different seeds should produce different results (with high probability)
        assert_ne!(result1, result2);
    }

    #[test]
    fn test_jgd_debug() {
        let jgd = Jgd::from(r#"{
            "$format": "jgd/v1",
            "version": "test",
            "root": {
                "fields": {
                    "test": "value"
                }
            }
        }"#);

        let debug_str = format!("{:?}", jgd);
        assert!(debug_str.contains("Jgd"));
        assert!(debug_str.contains("jgd/v1"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_complex_root_mode_with_count() {
        let jgd = Jgd::from(r#"{
            "$format": "jgd/v1",
            "version": "1.0.0",
            "seed": 42,
            "root": {
                "count": 2,
                "fields": {
                    "id": {
                        "number": {
                            "min": 1,
                            "max": 10,
                            "integer": true
                        }
                    },
                    "name": "Test Item"
                }
            }
        }"#);

        let result = jgd.generate();

        assert!(result.is_array());
        if let Value::Array(arr) = result {
            assert_eq!(arr.len(), 2);

            for item in &arr {
                assert!(item.is_object());
                if let Value::Object(obj) = item {
                    assert!(obj.contains_key("id"));
                    assert!(obj.contains_key("name"));
                    assert_eq!(obj.get("name"), Some(&Value::String("Test Item".to_string())));
                }
            }
        }
    }

    #[test]
    fn test_entities_with_cross_references() {
        let jgd = Jgd::from(r#"{
            "$format": "jgd/v1",
            "version": "1.0.0",
            "seed": 42,
            "entities": {
                "users": {
                    "fields": {
                        "id": 1,
                        "name": "Alice"
                    }
                },
                "posts": {
                    "fields": {
                        "author_id": {
                            "ref": "users.id"
                        },
                        "title": "My Post"
                    }
                }
            }
        }"#);

        let result = jgd.generate();

        assert!(result.is_object());
        if let Value::Object(obj) = result {
            assert!(obj.contains_key("users"));
            assert!(obj.contains_key("posts"));

            // Verify cross-reference works
            if let Some(Value::Object(posts)) = obj.get("posts") {
                assert_eq!(posts.get("author_id"), Some(&Value::Number(serde_json::Number::from(1))));
                assert_eq!(posts.get("title"), Some(&Value::String("My Post".to_string())));
            }
        }
    }

    #[test]
    fn test_custom_locale() {
        let jgd = Jgd::from(r#"{
            "$format": "jgd/v1",
            "version": "1.0.0",
            "defaultLocale": "DE",
            "root": {
                "fields": {
                    "message": "Hallo Welt"
                }
            }
        }"#);

        assert_eq!(jgd.default_locale, "DE");

        let result = jgd.generate();
        assert!(result.is_object());
    }

    #[test]
    fn test_minimal_valid_schema() {
        let jgd = Jgd::from(r#"{
            "$format": "jgd/v1",
            "version": "",
            "root": {
                "fields": {}
            }
        }"#);

        let result = jgd.generate();
        assert!(result.is_object());

        if let Value::Object(obj) = result {
            assert!(obj.is_empty());
        }
    }

    #[test]
    fn test_custom_key() {
        let key = "custom";
        Jgd::add_custom_key(key, Arc::new(|_| Ok(Value::String("worked".to_string()))));

        if let Some(func) = Jgd::get_custom_key(key) {
            if let Ok(Value::String(value)) = func(Arguments::None) {
                assert_eq!("worked", value)
            }
        }
    }
}
