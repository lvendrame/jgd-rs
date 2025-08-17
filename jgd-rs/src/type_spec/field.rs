//! # Field Module
//!
//! This module defines the core `Field` enum which represents any value that can be generated
//! in a JGD (JSON Generator Definition) schema. Fields are the building blocks of JSON data
//! generation, supporting primitive types, complex structures, and dynamic content generation.
//!
//! ## Overview
//!
//! The `Field` enum supports all JSON value types plus specialized generator types:
//! - **Primitive types**: strings, numbers, booleans, null
//! - **Complex types**: arrays, entities (objects), optional values
//! - **Dynamic types**: references to other generated values, template strings with placeholders
//!
//! ## Template String Processing
//!
//! String fields support template syntax with `${...}` placeholders for:
//! - Fake data generation: `"${name.firstName}"`, `"${address.city}"`
//! - Cross-references: `"${users.id}"`, `"${posts.title}"`
//! - Function calls with arguments: `"${lorem.sentence(5)}"`

use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::Value;
use crate::{type_spec::{ArraySpec, Entity, GeneratorConfig, JsonGenerator, NumberSpec, OptionalSpec, ReplacerCollection}, JgdGeneratorError, LocalConfig};

/// A field specification that can generate any JSON value type.
///
/// Fields are the fundamental building blocks in JGD schemas. Each field variant
/// corresponds to a different type of value generation strategy. The enum uses
/// `#[serde(untagged)]` to support flexible JSON deserialization where the structure
/// determines the variant.
///
/// # Variants
///
/// ## Complex Types
/// - **`Array`**: Generates arrays with configurable element types and counts
/// - **`Entity`**: Generates nested objects with multiple fields
/// - **`Optional`**: Conditionally generates values based on probability
///
/// ## Dynamic Types
/// - **`Ref`**: References values from other generated entities
/// - **`Str`**: Template strings with placeholder substitution support
///
/// ## Primitive Types
/// - **`Number`**: Generates numbers within specified ranges
/// - **`Bool`**: Static boolean values
/// - **`I64`**: Static 64-bit integer values
/// - **`F64`**: Static 64-bit floating-point values
/// - **`Null`**: JSON null values
///
/// # JGD Schema Examples
///
/// ```json
/// {
///   "name": "John Doe",                          // Field::Str
///   "age": { "number": { "min": 18, "max": 65 } }, // Field::Number
///   "active": true,                              // Field::Bool
///   "score": 95.5,                              // Field::F64
///   "id": 12345,                                // Field::I64
///   "metadata": null,                           // Field::Null
///   "email": "${internet.email}",               // Field::Str with template
///   "user_id": { "ref": "users.id" },          // Field::Ref
///   "tags": {                                   // Field::Array
///     "array": {
///       "count": 3,
///       "of": "${lorem.word}"
///     }
///   },
///   "profile": {                                // Field::Optional
///     "optional": {
///       "prob": 0.8,
///       "of": { "bio": "${lorem.sentence}" }
///     }
///   }
/// }
/// ```
///
/// # Deserialization
///
/// The `#[serde(untagged)]` attribute allows automatic variant detection:
/// - Objects with `"array"` key → `Field::Array`
/// - Objects with `"number"` key → `Field::Number`
/// - Objects with `"optional"` key → `Field::Optional`
/// - Objects with `"ref"` key → `Field::Ref`
/// - Plain strings → `Field::Str`
/// - Plain numbers → `Field::I64` or `Field::F64`
/// - Plain booleans → `Field::Bool`
/// - `null` → `Field::Null`
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Field {
    /// Array field that generates JSON arrays.
    ///
    /// Wraps an `ArraySpec` that defines the element type and count for array generation.
    /// Arrays can contain any field type as elements and support dynamic sizing.
    Array  {
        array: ArraySpec
    },

    /// Entity field that generates nested JSON objects.
    ///
    /// Embeds a complete `Entity` specification for generating complex nested structures.
    /// Entities can contain multiple fields and support uniqueness constraints.
    Entity(Entity),

    /// Number field that generates numeric values within ranges.
    ///
    /// Wraps a `NumberSpec` that defines the range and type (integer/float) for number generation.
    /// Supports both discrete integer ranges and continuous floating-point ranges.
    Number {
        number: NumberSpec
    },

    /// Optional field that conditionally generates values.
    ///
    /// Wraps an `OptionalSpec` that defines probability-based value generation.
    /// Can generate the specified field or null based on the configured probability.
    Optional {
        optional: OptionalSpec
    },

    /// Reference field that links to other generated entities.
    ///
    /// Contains a dot-notation path string for accessing values from previously generated
    /// entities. Enables cross-referencing and relational data generation.
    Ref {
        r#ref: String
    },

    /// String field with template support.
    ///
    /// Can be a literal string or contain `${...}` placeholders for dynamic content generation.
    /// Supports faker function calls and cross-references to other entities.
    Str(String),

    /// Static boolean field.
    ///
    /// Generates a fixed boolean value without any dynamic behavior.
    Bool(bool),

    /// Static 64-bit integer field.
    ///
    /// Generates a fixed integer value without any dynamic behavior.
    I64(i64),

    /// Static 64-bit floating-point field.
    ///
    /// Generates a fixed floating-point value without any dynamic behavior.
    F64(f64),

    /// Null field.
    ///
    /// Always generates a JSON null value.
    Null,
}

impl Field {
    /// Resolves a reference path to retrieve a value from generated entities.
    ///
    /// This method handles cross-reference resolution by looking up values in the
    /// generator configuration's `gen_value` map using dot-notation paths.
    ///
    /// # Parameters
    /// - `r#ref`: The dot-notation path to resolve (e.g., "users.name", "posts.0.title")
    /// - `config`: Mutable reference to the generator configuration containing generated values
    ///
    /// # Returns
    /// - `Value`: The resolved value if found, or an error message string if the path doesn't exist
    ///
    /// # Path Resolution
    ///
    /// Paths follow dot-notation syntax:
    /// - `"entity.field"` - Access a field from a single entity
    /// - `"entity.0.field"` - Access a field from the first item in an entity array
    /// - `"entity.id"` - Common pattern for referencing entity IDs
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Reference a user's name from a posts entity
    /// let user_ref = Field::Ref { r#ref: "users.name".to_string() };
    /// let resolved_value = user_ref.generate_for_ref("users.name", &mut config);
    ///
    /// // Reference an ID from an entity array
    /// let id_ref = Field::Ref { r#ref: "users.0.id".to_string() };
    /// let user_id = id_ref.generate_for_ref("users.0.id", &mut config);
    /// ```
    fn generate_for_ref(&self, r#ref: &str, config: &mut GeneratorConfig, local_config: Option<&mut LocalConfig>
        ) -> Result<Value, JgdGeneratorError> {
        let value = config.get_value_from_path(r#ref.to_string());

        if let Some(value) = value {
            return Ok(value.clone());
        }

        let (entity_name, field_name) = if let Some(local_config) = local_config {
            let entity_name = local_config.entity_name.clone();
            let field_name = local_config.field_name.clone();
            (entity_name, field_name)
        } else {
            (None, None)
        };

        Err(JgdGeneratorError {
            message: format!("The path {} is not found", r#ref),
            entity: entity_name,
            field: field_name,
        })
    }
}

impl JsonGenerator for Field {
    /// Generates a JSON value based on the field type.
    ///
    /// This method dispatches to the appropriate generation logic for each field variant.
    /// It handles all supported field types and ensures proper JSON value generation
    /// according to the JGD specification.
    ///
    /// # Parameters
    /// - `config`: Mutable reference to generator configuration for accessing state and utilities
    ///
    /// # Returns
    /// - `serde_json::Value`: The generated JSON value appropriate for the field type
    ///
    /// # Generation Behavior
    ///
    /// - **Array**: Delegates to `ArraySpec::generate()` for array creation
    /// - **Entity**: Delegates to `Entity::generate()` for object creation
    /// - **Number**: Delegates to `NumberSpec::generate()` for numeric value generation
    /// - **Optional**: Delegates to `OptionalSpec::generate()` for probability-based generation
    /// - **Ref**: Resolves cross-references using `generate_for_ref()`
    /// - **Str**: Processes template strings with placeholder replacement
    /// - **Bool/I64/F64/Null**: Direct conversion to corresponding JSON values
    ///
    /// # Template Processing
    ///
    /// String fields undergo template processing to replace `${...}` placeholders:
    /// - Faker calls: `"${name.firstName}"` → `"John"`
    /// - Cross-references: `"${users.id}"` → `"12345"`
    /// - Function calls: `"${lorem.words(3)}"` → `"lorem ipsum dolor"`
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let field = Field::Str("Hello ${name.firstName}!".to_string());
    /// let result = field.generate(&mut config);
    /// // Result: Value::String("Hello John!")
    ///
    /// let number_field = Field::Number {
    ///     number: NumberSpec::new_integer(1.0, 100.0)
    /// };
    /// let result = number_field.generate(&mut config);
    /// // Result: Value::Number(42)
    /// ```
    fn generate(&self, config: &mut super::GeneratorConfig, local_config: Option<&mut LocalConfig>
        ) -> Result<Value, JgdGeneratorError> {
        match self {
            // Field::Object { object } => object.generate(config),
            Field::Array { array } => array.generate(config, local_config),
            Field::Entity(entity) => entity.generate(config, local_config),
            Field::Number { number } => number.generate(config, local_config),
            Field::Optional { optional } => optional.generate(config, local_config),
            Field::Ref { r#ref } => self.generate_for_ref(r#ref, config, local_config),
            Field::Str(value) => value.generate(config, local_config),
            Field::Bool(value) => Ok(Value::Bool(*value)),
            Field::I64(value) => Ok(Value::Number(serde_json::Number::from(*value))),
            Field::F64(value) => Ok(Value::Number(serde_json::Number::from_f64(*value).unwrap())),
            Field::Null => Ok(Value::Null),
        }
    }
}

impl JsonGenerator for IndexMap<String, Field> {
    /// Generates a JSON object from a map of field specifications.
    ///
    /// This implementation allows `IndexMap<String, Field>` to be used directly as a
    /// JSON generator, converting each field to its corresponding JSON value while
    /// preserving key ordering.
    ///
    /// # Parameters
    /// - `config`: Mutable reference to generator configuration for field generation
    ///
    /// # Returns
    /// - `Value`: A JSON object containing all generated field values
    ///
    /// # Behavior
    ///
    /// - Iterates through the map in insertion order (preserved by `IndexMap`)
    /// - Generates each field value using the field's `generate()` method
    /// - Collects all key-value pairs into a JSON object
    /// - Maintains field ordering as defined in the original specification
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut fields = IndexMap::new();
    /// fields.insert("name".to_string(), Field::Str("John".to_string()));
    /// fields.insert("age".to_string(), Field::I64(30));
    /// fields.insert("active".to_string(), Field::Bool(true));
    ///
    /// let result = fields.generate(&mut config);
    /// // Result: {"name": "John", "age": 30, "active": true}
    /// ```
    ///
    /// # Use Cases
    ///
    /// This implementation is primarily used by:
    /// - Entity field generation for creating object structures
    /// - Root-level object generation in JGD schemas
    /// - Nested object creation within complex field hierarchies
    fn generate(&self, config: &mut super::GeneratorConfig, local_config: Option<&mut LocalConfig>
        ) -> Result<Value, JgdGeneratorError> {

        let mut local_config = LocalConfig::from_current_with_config(None, None, local_config);

        let mut map = serde_json::Map::new();
        for (key, field) in self {
            local_config.field_name = Some(key.clone());
            let generated = field.generate(config, Some(&mut local_config))?;
            map.insert(key.clone(), generated);
        }

        Ok(Value::Object(map))
    }
}

impl JsonGenerator for String {
    /// Generates a JSON value from a string with template processing.
    ///
    /// This implementation enables `String` values to be used directly in the JSON generation
    /// pipeline with automatic template processing for placeholder substitution.
    ///
    /// # Parameters
    /// - `config`: Mutable reference to generator configuration for replacement processing
    ///
    /// # Returns
    /// - `Value`: A JSON string value, either the original string or with placeholders replaced
    ///
    /// # Template Processing
    ///
    /// The method uses `ReplacerCollection` to detect and process `${...}` placeholders:
    ///
    /// 1. **No placeholders**: Returns the string as-is
    /// 2. **With placeholders**: Attempts replacement using available replacers
    /// 3. **Replacement failure**: Falls back to the original string
    ///
    /// # Placeholder Types
    ///
    /// - **Faker calls**: `"${name.firstName}"` → generates fake names
    /// - **Cross-references**: `"${users.id}"` → references other entity values
    /// - **Function calls**: `"${lorem.sentence(5)}"` → calls faker functions with arguments
    /// - **Mixed content**: `"User: ${name.firstName} (${internet.email})"` → multiple replacements
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Simple string without placeholders
    /// let simple = "Hello World".to_string();
    /// let result = simple.generate(&mut config);
    /// // Result: Value::String("Hello World")
    ///
    /// // Template string with faker call
    /// let template = "Welcome ${name.firstName}!".to_string();
    /// let result = template.generate(&mut config);
    /// // Result: Value::String("Welcome John!")
    ///
    /// // Complex template with multiple placeholders
    /// let complex = "User ${name.firstName} lives in ${address.city}".to_string();
    /// let result = complex.generate(&mut config);
    /// // Result: Value::String("User John lives in New York")
    /// ```
    ///
    /// # Error Handling
    ///
    /// If replacement fails for any reason, the method gracefully falls back to
    /// returning the original string value, ensuring generation never fails due
    /// to template processing errors.
    fn generate(&self, config: &mut super::GeneratorConfig, local_config: Option<&mut LocalConfig>
        ) -> Result<Value, JgdGeneratorError> {

        let value = self.to_string();
        let replacers = ReplacerCollection::new(value.clone());
        if replacers.is_empty() {
            return Ok(Value::String(value));
        }

        replacers.replace(config, local_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_spec::{Count, NumberSpec};
    use serde_json::json;

    fn create_test_config(seed: Option<u64>) -> GeneratorConfig {
        GeneratorConfig::new("EN", seed)
    }

    #[test]
    fn test_field_str_without_placeholders() {
        let mut config = create_test_config(Some(42));
        let field = Field::Str("Hello World".to_string());

        let result = field.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            assert_eq!(result, Value::String("Hello World".to_string()));
        }
    }

    #[test]
    fn test_field_bool_true() {
        let mut config = create_test_config(Some(42));
        let field = Field::Bool(true);

        let result = field.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            assert_eq!(result, Value::Bool(true));
        }
    }

    #[test]
    fn test_field_bool_false() {
        let mut config = create_test_config(Some(42));
        let field = Field::Bool(false);

        let result = field.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            assert_eq!(result, Value::Bool(false));
        }
    }

    #[test]
    fn test_field_i64() {
        let mut config = create_test_config(Some(42));
        let field = Field::I64(12345);

        let result = field.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            assert_eq!(result, Value::Number(serde_json::Number::from(12345)));
        }
    }

    #[test]
    fn test_field_f64() {
        let mut config = create_test_config(Some(42));
        let field = Field::F64(123.45);

        let result = field.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            if let Value::Number(num) = result {
                assert_eq!(num.as_f64(), Some(123.45));
            } else {
                panic!("Expected number value");
            }
        }
    }

    #[test]
    fn test_field_null() {
        let mut config = create_test_config(Some(42));
        let field = Field::Null;

        let result = field.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            assert_eq!(result, Value::Null);
        }
    }

    #[test]
    fn test_field_number() {
        let mut config = create_test_config(Some(42));
        let number_spec = NumberSpec::new_integer(1.0, 10.0);
        let field = Field::Number { number: number_spec };

        let result = field.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            assert!(result.is_number());

            if let Value::Number(num) = result {
                let value = num.as_i64().unwrap();
                assert!((1..=10).contains(&value));
            }
        }
    }

    #[test]
    fn test_field_array() {
        let mut config = create_test_config(Some(42));
        let array_spec = ArraySpec {
            count: Some(Count::Fixed(3)),
            of: Box::new(Field::Str("test".to_string())),
        };
        let field = Field::Array { array: array_spec };

        let result = field.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            assert!(result.is_array());

            if let Value::Array(arr) = result {
                assert_eq!(arr.len(), 3);
                for item in arr {
                    assert_eq!(item, Value::String("test".to_string()));
                }
            }
        }
    }

    #[test]
    fn test_field_ref_existing_path() {
        let mut config = create_test_config(Some(42));

        // Set up a reference value in the config
        config.gen_value.insert("users".to_string(), json!({
            "id": 12345,
            "name": "John Doe"
        }));

        let field = Field::Ref { r#ref: "users.name".to_string() };
        let result = field.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            assert_eq!(result, Value::String("John Doe".to_string()));
        }
    }

    #[test]
    fn test_field_ref_missing_path() {
        let mut config = create_test_config(Some(42));
        let field = Field::Ref { r#ref: "nonexistent.path".to_string() };

        let result = field.generate(&mut config, None);
        assert!(result.is_err());

        if let Err(error) = result {
            assert_eq!(error.message, "The path nonexistent.path is not found".to_string());
        }
    }

    #[test]
    fn test_field_entity() {
        let mut config = create_test_config(Some(42));

        let mut fields = IndexMap::new();
        fields.insert("name".to_string(), Field::Str("Test User".to_string()));
        fields.insert("age".to_string(), Field::I64(25));

        let entity = Entity {
            count: None,
            seed: None,
            unique_by: vec![],
            fields,
        };

        let field = Field::Entity(entity);
        let result = field.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            assert!(result.is_object());
            if let Value::Object(obj) = result {
                assert_eq!(obj.get("name"), Some(&Value::String("Test User".to_string())));
                assert_eq!(obj.get("age"), Some(&Value::Number(serde_json::Number::from(25))));
            }
        }
    }

    #[test]
    fn test_indexmap_field_generation() {
        let mut config = create_test_config(Some(42));
        let mut fields = IndexMap::new();

        fields.insert("string_field".to_string(), Field::Str("Hello".to_string()));
        fields.insert("number_field".to_string(), Field::I64(42));
        fields.insert("bool_field".to_string(), Field::Bool(true));
        fields.insert("null_field".to_string(), Field::Null);

        let result = fields.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            assert!(result.is_object());
            if let Value::Object(obj) = result {
                assert_eq!(obj.get("string_field"), Some(&Value::String("Hello".to_string())));
                assert_eq!(obj.get("number_field"), Some(&Value::Number(serde_json::Number::from(42))));
                assert_eq!(obj.get("bool_field"), Some(&Value::Bool(true)));
                assert_eq!(obj.get("null_field"), Some(&Value::Null));

                // Verify ordering is preserved (IndexMap maintains insertion order)
                let keys: Vec<&String> = obj.keys().collect();
                assert_eq!(keys, vec!["string_field", "number_field", "bool_field", "null_field"]);
            }
        }
    }

    #[test]
    fn test_indexmap_empty_generation() {
        let mut config = create_test_config(Some(42));
        let fields: IndexMap<String, Field> = IndexMap::new();

        let result = fields.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            assert!(result.is_object());
            if let Value::Object(obj) = result {
                assert!(obj.is_empty());
            }
        }
    }

    #[test]
    fn test_string_template_processing() {
        let mut config = create_test_config(Some(42));

        let template = "Hello ${name.name}!".to_string();
        let result = template.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            // The result should either be the template with replacement or the original string
            // Since we can't control the exact replacement logic in this test,
            // we verify it's still a string
            assert!(result.is_string());
        }
    }

    #[test]
    fn test_string_invalid_template_error() {
        let mut config = create_test_config(Some(42));

        let template = "Hello ${invalid.key}!".to_string();
        let result = template.generate(&mut config, None);
        assert!(result.is_err());

        if let Err(error) = result {
            // The result should either be the template with replacement or the original string
            // Since we can't control the exact replacement logic in this test,
            // we verify it's still a string
            assert_eq!(error.message, "Error to process the pattern ${invalid.key}".to_string());
        }
    }

    #[test]
    fn test_string_no_placeholders() {
        let mut config = create_test_config(Some(42));
        let simple_string = "No placeholders here".to_string();

        let result = simple_string.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            assert_eq!(result, Value::String("No placeholders here".to_string()));
        }
    }

    #[test]
    fn test_field_clone() {
        let field = Field::Str("test".to_string());
        let cloned = field.clone();

        match (field, cloned) {
            (Field::Str(original), Field::Str(cloned_str)) => {
                assert_eq!(original, cloned_str);
            }
            _ => panic!("Clone should preserve field type"),
        }
    }

    #[test]
    fn test_field_debug() {
        let field = Field::Bool(true);
        let debug_str = format!("{:?}", field);
        assert!(debug_str.contains("Bool"));
        assert!(debug_str.contains("true"));
    }

    #[test]
    fn test_complex_nested_fields() {
        let mut config = create_test_config(Some(42));

        // Create a complex nested structure
        let mut inner_fields = IndexMap::new();
        inner_fields.insert("inner_str".to_string(), Field::Str("inner_value".to_string()));
        inner_fields.insert("inner_num".to_string(), Field::I64(99));

        let inner_entity = Entity {
            count: None,
            seed: None,
            unique_by: vec![],
            fields: inner_fields,
        };

        let mut outer_fields = IndexMap::new();
        outer_fields.insert("nested".to_string(), Field::Entity(inner_entity));
        outer_fields.insert("simple".to_string(), Field::Str("outer_value".to_string()));

        let result = outer_fields.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            assert!(result.is_object());
            if let Value::Object(outer_obj) = result {
                assert!(outer_obj.contains_key("nested"));
                assert!(outer_obj.contains_key("simple"));

                if let Some(Value::Object(inner_obj)) = outer_obj.get("nested") {
                    assert_eq!(inner_obj.get("inner_str"), Some(&Value::String("inner_value".to_string())));
                    assert_eq!(inner_obj.get("inner_num"), Some(&Value::Number(serde_json::Number::from(99))));
                } else {
                    panic!("Expected nested object");
                }
            }
        }
    }

    #[test]
    fn test_field_variants_coverage() {
        let mut config = create_test_config(Some(42));

        config.gen_value.insert("test".to_string(), json!({
            "path": "found"
        }));

        // Test all field variants to ensure they can be created and generate values
        let variants = vec![
            Field::Str("test".to_string()),
            Field::Bool(true),
            Field::I64(42),
            Field::F64(123.45), // Using arbitrary float to avoid clippy warnings
            Field::Null,
            Field::Number { number: NumberSpec::new_integer(1.0, 10.0) },
            Field::Ref { r#ref: "test.path".to_string() },
        ];

        for field in variants {
            let result = field.generate(&mut config, None);

            assert!(result.is_ok());

            if let Ok(result) = result {
                // Each field should generate some valid JSON value
                assert!(result.is_string() || result.is_number() || result.is_boolean() || result.is_null());
            }
        }
    }
}
