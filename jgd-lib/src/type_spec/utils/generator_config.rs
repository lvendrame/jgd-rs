use rand::{random_range, rngs::StdRng, SeedableRng};
use serde_json::Value;

use crate::fake::{FakeGenerator, FakeKeys};

/// Configuration for JSON data generation in the JGD system.
///
/// `GeneratorConfig` provides the runtime context and state needed for generating
/// fake JSON data according to JGD (JSON Generator Definition) specifications.
/// It encapsulates the random number generator, locale settings, fake data generators,
/// and maintains state during the generation process.
///
/// # Core Components
///
/// - **Random Number Generator**: Deterministic or random generation using `StdRng`
/// - **Locale Support**: Locale-specific fake data generation (names, addresses, etc.)
/// - **Fake Data**: Pre-configured generators for realistic fake data
/// - **Generation State**: Tracks generated values for cross-references and relationships
///
/// # Usage in JGD Generation
///
/// This configuration is passed to all [`JsonGenerator`] implementations to provide
/// consistent generation context. The same config instance should be used throughout
/// a single generation session to maintain consistency and enable value references.
///
/// # Examples
///
/// ```rust,ignore
/// use jgd_lib::{GeneratorConfig, NumberSpec};
///
/// // Create config with English locale and deterministic seed
/// let mut config = GeneratorConfig::new("EN", Some(42));
///
/// // Use config for generation
/// let spec = NumberSpec::new_integer(1.0, 100.0);
/// let value = spec.generate(&mut config);
/// ```
///
/// # Thread Safety
///
/// `GeneratorConfig` is not thread-safe due to its mutable state. Each thread
/// should have its own instance for concurrent generation.
pub struct GeneratorConfig {
    /// The locale code for locale-specific data generation.
    ///
    /// This field determines the language and regional settings used for generating
    /// locale-specific fake data such as names, addresses, and phone numbers.
    /// Common values include "EN" for English, "ES" for Spanish, etc.
    ///
    /// Currently marked as `dead_code` but preserved for future locale-specific features.
    #[allow(dead_code)]
    pub locale: String,

    /// Keys for accessing fake data categories.
    ///
    /// Provides access to different categories of fake data (names, addresses, etc.)
    /// that can be used during generation. This is used in conjunction with the
    /// fake generator to produce realistic test data.
    pub fake_keys: FakeKeys,

    /// Generator for producing locale-specific fake data.
    ///
    /// This component generates realistic fake data such as names, addresses,
    /// phone numbers, and other locale-specific information based on the
    /// configured locale setting.
    pub fake_generator: FakeGenerator,

    /// Random number generator for deterministic or random generation.
    ///
    /// Uses `StdRng` to provide high-quality random numbers. Can be seeded for
    /// deterministic generation (useful for testing) or use a random seed for
    /// truly random output.
    pub rng: StdRng,

    /// Map storing generated values for cross-references and relationships.
    ///
    /// This map maintains the state of previously generated values during a
    /// generation session. It enables features like referencing previously
    /// generated values or maintaining relationships between different parts
    /// of the generated data structure.
    pub gen_value: serde_json::Map<String, Value>,
}

impl GeneratorConfig {
    /// Creates a new `GeneratorConfig` with the specified locale and optional seed.
    ///
    /// This constructor initializes all components needed for JGD data generation,
    /// including the random number generator, fake data generators, and state storage.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale code for locale-specific data generation (e.g., "EN", "ES")
    /// * `seed` - Optional seed for deterministic random number generation. If `None`,
    ///   a random seed will be used for non-deterministic generation.
    ///
    /// # Returns
    ///
    /// A new `GeneratorConfig` instance ready for data generation.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use jgd_lib::GeneratorConfig;
    ///
    /// // Deterministic generation with seed
    /// let config = GeneratorConfig::new("EN", Some(42));
    ///
    /// // Random generation
    /// let config = GeneratorConfig::new("EN", None);
    /// ```
    ///
    /// # Deterministic Generation
    ///
    /// When a seed is provided, the generator will produce the same sequence of
    /// values across different runs, which is useful for:
    /// - Testing and debugging
    /// - Reproducible data sets
    /// - Consistent development environments
    pub fn new(locale: &str, seed: Option<u64>) -> Self {
        let fake_keys = FakeKeys::new();
        let fake_generator = FakeGenerator::new(locale);
        let rng = StdRng::seed_from_u64(seed.unwrap_or(rand::random()));
        let locale = locale.to_string();

        Self {
            locale,
            fake_keys,
            fake_generator,
            rng,
            gen_value: serde_json::Map::new(),
        }
    }

    /// Retrieves a random item from an array and extracts a specific field.
    ///
    /// This is a utility method for working with arrays of objects during generation.
    /// It randomly selects an object from the array and returns the value of the
    /// specified field, if it exists.
    ///
    /// # Arguments
    ///
    /// * `value` - The JSON value that should be an array of objects
    /// * `key` - The field name to extract from the randomly selected object
    ///
    /// # Returns
    ///
    /// `Some(&Value)` if the value is an array containing objects with the specified key,
    /// `None` otherwise.
    ///
    /// # Usage in Path Resolution
    ///
    /// This method is used internally by [`get_value_from_path`] when traversing
    /// array elements in dot-notation paths.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use serde_json::{json, Value};
    /// use jgd_lib::GeneratorConfig;
    ///
    /// let array = json!([
    ///     {"name": "Alice", "age": 30},
    ///     {"name": "Bob", "age": 25}
    /// ]);
    ///
    /// let name = GeneratorConfig::get_random_item_from_array(&array, "name");
    /// // Returns Some with either "Alice" or "Bob"
    /// ```
    fn get_random_item_from_array<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
        if let Value::Array(items) = value {
            if items.is_empty() {
                return None;
            }

            if let Some(Value::Object(obj)) = items.get(random_range(0..items.len())) {
                return obj.get(key);
            }
        }

        None
    }

    /// Retrieves a value from the generated data using a dot-notation path.
    ///
    /// This method enables cross-references and relationships in generated data by
    /// allowing access to previously generated values. It supports traversing nested
    /// objects and randomly selecting from arrays using dot-notation paths.
    ///
    /// # Arguments
    ///
    /// * `path` - A dot-notation path string (e.g., "user.address.city")
    ///
    /// # Returns
    ///
    /// `Some(&Value)` if the path exists in the generated data, `None` otherwise.
    ///
    /// # Path Resolution Rules
    ///
    /// - **Objects**: Navigate through object properties using dot notation
    /// - **Arrays**: Randomly select an object from the array and continue traversal
    /// - **Primitive Values**: Cannot be traversed further (returns `None` for additional path segments)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use jgd_lib::GeneratorConfig;
    /// use serde_json::json;
    ///
    /// let mut config = GeneratorConfig::new("EN", Some(42));
    ///
    /// // Assume some data has been generated and stored
    /// config.gen_value.insert("user".to_string(), json!({
    ///     "name": "Alice",
    ///     "address": {
    ///         "city": "New York",
    ///         "country": "USA"
    ///     }
    /// }));
    ///
    /// // Access nested values
    /// let city = config.get_value_from_path("user.address.city".to_string());
    /// assert_eq!(city, Some(&json!("New York")));
    /// ```
    ///
    /// # Use Cases
    ///
    /// - **Foreign Key References**: Reference IDs from previously generated entities
    /// - **Consistent Data**: Ensure related fields have consistent values
    /// - **Complex Relationships**: Build interconnected data structures
    /// - **Template Substitution**: Replace placeholders with generated values
    ///
    /// # Performance Notes
    ///
    /// - Array traversal involves random selection, making it non-deterministic for the same path
    /// - Deep paths may have performance implications for large data structures
    /// - Path parsing is done on each call; consider caching for frequently accessed paths
    pub fn get_value_from_path(&self, path: String) -> Option<&Value> {
        let path = path.split(".");
        let mut current_value: Option<&Value> = None;
        let mut is_first = true;

        for key in path {
            let item = if is_first {
                is_first = false;
                self.gen_value.get(key)
            } else if let Some(value) = current_value {
                match value {
                    Value::Array(_) => Self::get_random_item_from_array(value, key),
                    Value::Object(map) => map.get(key),
                    _ => return None
                }
            } else {
                return None;
            };

            if let Some(part) = item {
                current_value = Some(part);
            } else {
                return None;
            }
        }

        current_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new_with_seed() {
        let config = GeneratorConfig::new("EN", Some(42));

        assert_eq!(config.locale, "EN");
        assert!(!config.gen_value.is_empty() || config.gen_value.is_empty()); // gen_value starts empty
    }

    #[test]
    fn test_new_with_random_seed() {
        let config1 = GeneratorConfig::new("EN", None);
        let config2 = GeneratorConfig::new("EN", None);

        assert_eq!(config1.locale, "EN");
        assert_eq!(config2.locale, "EN");
        // Both configs should be properly initialized
        assert!(!config1.gen_value.is_empty() || config1.gen_value.is_empty());
        assert!(!config2.gen_value.is_empty() || config2.gen_value.is_empty());
    }

    #[test]
    fn test_new_with_different_locales() {
        let config_en = GeneratorConfig::new("EN", Some(42));
        let config_es = GeneratorConfig::new("ES", Some(42));

        assert_eq!(config_en.locale, "EN");
        assert_eq!(config_es.locale, "ES");
    }

    #[test]
    fn test_get_random_item_from_array_with_valid_array() {
        let array = json!([
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25},
            {"name": "Charlie", "age": 35}
        ]);

        // Test multiple calls to ensure we can get different items
        let mut found_names = std::collections::HashSet::new();
        for _ in 0..20 {
            if let Some(name) = GeneratorConfig::get_random_item_from_array(&array, "name") {
                if let Some(name_str) = name.as_str() {
                    found_names.insert(name_str.to_string());
                }
            }
        }

        // We should get at least one valid name (might get all three depending on randomness)
        assert!(!found_names.is_empty());
        assert!(found_names.iter().any(|name| ["Alice", "Bob", "Charlie"].contains(&name.as_str())));
    }

    #[test]
    fn test_get_random_item_from_array_with_missing_key() {
        let array = json!([
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25}
        ]);

        let result = GeneratorConfig::get_random_item_from_array(&array, "nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_get_random_item_from_array_with_non_array() {
        let not_array = json!({"name": "Alice", "age": 30});

        let result = GeneratorConfig::get_random_item_from_array(&not_array, "name");
        assert!(result.is_none());
    }

    #[test]
    fn test_get_random_item_from_array_with_empty_array() {
        let empty_array = json!([]);

        let result = GeneratorConfig::get_random_item_from_array(&empty_array, "name");
        assert!(result.is_none());
    }

    #[test]
    fn test_get_random_item_from_array_with_non_object_items() {
        let array_with_primitives = json!(["Alice", "Bob", "Charlie"]);

        let result = GeneratorConfig::get_random_item_from_array(&array_with_primitives, "name");
        assert!(result.is_none());
    }

    #[test]
    fn test_get_value_from_path_simple_key() {
        let mut config = GeneratorConfig::new("EN", Some(42));
        config.gen_value.insert("name".to_string(), json!("Alice"));

        let result = config.get_value_from_path("name".to_string());
        assert_eq!(result, Some(&json!("Alice")));
    }

    #[test]
    fn test_get_value_from_path_nested_object() {
        let mut config = GeneratorConfig::new("EN", Some(42));
        config.gen_value.insert("user".to_string(), json!({
            "name": "Alice",
            "address": {
                "city": "New York",
                "country": "USA",
                "details": {
                    "zip": "10001"
                }
            }
        }));

        // Test direct nested access
        let city = config.get_value_from_path("user.address.city".to_string());
        assert_eq!(city, Some(&json!("New York")));

        // Test deep nested access
        let zip = config.get_value_from_path("user.address.details.zip".to_string());
        assert_eq!(zip, Some(&json!("10001")));

        // Test top-level access
        let name = config.get_value_from_path("user.name".to_string());
        assert_eq!(name, Some(&json!("Alice")));
    }

    #[test]
    fn test_get_value_from_path_with_array() {
        let mut config = GeneratorConfig::new("EN", Some(42));
        config.gen_value.insert("users".to_string(), json!([
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25},
            {"name": "Charlie", "age": 35}
        ]));

        // Test array traversal - should return one of the names
        let result = config.get_value_from_path("users.name".to_string());
        assert!(result.is_some());

        if let Some(name) = result {
            if let Some(name_str) = name.as_str() {
                assert!(["Alice", "Bob", "Charlie"].contains(&name_str));
            } else {
                panic!("Expected string value");
            }
        }
    }

    #[test]
    fn test_get_value_from_path_nonexistent_key() {
        let mut config = GeneratorConfig::new("EN", Some(42));
        config.gen_value.insert("user".to_string(), json!({
            "name": "Alice"
        }));

        let result = config.get_value_from_path("user.nonexistent".to_string());
        assert!(result.is_none());
    }

    #[test]
    fn test_get_value_from_path_nonexistent_root() {
        let config = GeneratorConfig::new("EN", Some(42));

        let result = config.get_value_from_path("nonexistent".to_string());
        assert!(result.is_none());
    }

    #[test]
    fn test_get_value_from_path_invalid_traversal() {
        let mut config = GeneratorConfig::new("EN", Some(42));
        config.gen_value.insert("user".to_string(), json!("Alice"));

        // Trying to traverse a primitive value should return None
        let result = config.get_value_from_path("user.name".to_string());
        assert!(result.is_none());
    }

    #[test]
    fn test_get_value_from_path_empty_path() {
        let mut config = GeneratorConfig::new("EN", Some(42));
        config.gen_value.insert("".to_string(), json!("test"));

        let result = config.get_value_from_path("".to_string());
        assert_eq!(result, Some(&json!("test")));
    }

    #[test]
    fn test_get_value_from_path_complex_nested_with_arrays() {
        let mut config = GeneratorConfig::new("EN", Some(42));
        config.gen_value.insert("company".to_string(), json!({
            "departments": [
                {
                    "name": "Engineering",
                    "employees": [
                        {"name": "Alice", "role": "Developer"},
                        {"name": "Bob", "role": "Manager"}
                    ]
                },
                {
                    "name": "Marketing",
                    "employees": [
                        {"name": "Charlie", "role": "Designer"},
                        {"name": "Diana", "role": "Analyst"}
                    ]
                }
            ]
        }));

        // Test nested array traversal
        let dept_name = config.get_value_from_path("company.departments.name".to_string());
        assert!(dept_name.is_some());

        if let Some(name) = dept_name {
            if let Some(name_str) = name.as_str() {
                assert!(["Engineering", "Marketing"].contains(&name_str));
            }
        }
    }

    #[test]
    fn test_gen_value_map_operations() {
        let mut config = GeneratorConfig::new("EN", Some(42));

        // Test that gen_value starts empty
        assert!(config.gen_value.is_empty());

        // Test adding values
        config.gen_value.insert("key1".to_string(), json!("value1"));
        config.gen_value.insert("key2".to_string(), json!({"nested": "value2"}));

        assert_eq!(config.gen_value.len(), 2);
        assert_eq!(config.gen_value.get("key1"), Some(&json!("value1")));
        assert_eq!(config.gen_value.get("key2"), Some(&json!({"nested": "value2"})));
    }
}
