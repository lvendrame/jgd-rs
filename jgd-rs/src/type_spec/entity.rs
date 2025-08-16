use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;
use rand::{rngs::StdRng, SeedableRng};
use serde::Deserialize;
use serde_json::Value;
use crate::{type_spec::{Count, Field, GetCount, JsonGenerator}, JgdGeneratorError, LocalConfig};

/// Creates a fingerprint for uniqueness checking based on specified fields.
///
/// This function extracts values from the specified fields in the JSON object
/// and creates a string fingerprint that can be used to check for duplicates.
///
/// # Arguments
///
/// * `obj` - The JSON object to create a fingerprint for
/// * `unique_fields` - Vector of field names to include in the fingerprint
///
/// # Returns
///
/// A string fingerprint representing the combination of unique field values
fn fingerprint(obj: &Value, unique_fields: &[String]) -> String {
    let mut parts = Vec::new();

    if let Value::Object(map) = obj {
        for field in unique_fields {
            if let Some(value) = map.get(field) {
                // Convert the value to a string representation for fingerprinting
                let value_str = match value {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    Value::Null => "null".to_string(),
                    _ => serde_json::to_string(value).unwrap_or_else(|_| "unknown".to_string()),
                };
                parts.push(format!("{}:{}", field, value_str));
            } else {
                parts.push(format!("{}:missing", field));
            }
        }
    }

    parts.join("|")
}

/// Represents an entity specification for generating structured data objects in JGD schemas.
///
/// `Entity` defines a collection of named fields that together form a structured data object.
/// It supports generating single objects or arrays of objects, with optional uniqueness
/// constraints to ensure no duplicate entities based on specified field combinations.
///
/// # JGD Schema Integration
///
/// Entities are the primary building blocks for creating structured data in JGD schemas.
/// They correspond to objects in JSON and can contain multiple fields of different types,
/// creating realistic data structures for testing and development.
///
/// # Features
///
/// - **Multiple Fields**: Define complex objects with various field types
/// - **Count Control**: Generate single objects or arrays of objects
/// - **Uniqueness Constraints**: Ensure entities are unique based on specified fields
/// - **Seeding Support**: Optional seed for deterministic generation
/// - **Cross-References**: Fields can reference other generated entities
///
/// # JSON Schema Representation
///
/// ```json
/// {
///   "entity": {
///     "count": 10,
///     "unique_by": ["id", "email"],
///     "fields": {
///       "id": "${uuid}",
///       "name": "${name.fullName}",
///       "email": "${internet.email}",
///       "age": {
///         "number": {
///           "min": 18,
///           "max": 65,
///           "integer": true
///         }
///       }
///     }
///   }
/// }
/// ```
///
/// # Uniqueness Constraints
///
/// The `unique_by` field allows specifying which field combinations must be unique
/// across all generated entities. This is useful for:
/// - Primary key constraints (IDs, usernames, emails)
/// - Composite uniqueness (user + project combinations)
/// - Realistic data constraints (no duplicate SSNs, etc.)
///
/// # Examples
///
/// ```rust,ignore
/// use jgd_rs::{Entity, Field, Count, JsonGenerator, GeneratorConfig};
/// use indexmap::IndexMap;
///
/// let mut config = GeneratorConfig::new("EN", Some(42));
///
/// // Create user entities with unique emails
/// let mut fields = IndexMap::new();
/// fields.insert("id".to_string(), Field::Str("${uuid}".to_string()));
/// fields.insert("name".to_string(), Field::Str("${name.fullName}".to_string()));
/// fields.insert("email".to_string(), Field::Str("${internet.email}".to_string()));
///
/// let entity = Entity {
///     count: Some(Count::Fixed(5)),
///     seed: None,
///     unique_by: vec!["email".to_string()],
///     fields,
/// };
///
/// let result = entity.generate(&mut config);
/// // Generates an array of 5 user objects with unique emails
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct Entity {
    /// Optional count specification for the number of entities to generate.
    ///
    /// Determines whether to generate a single entity object or an array of entities:
    /// - **Some(count)**: Generates an array with the specified count
    /// - **None**: Generates a single entity object (not wrapped in an array)
    ///
    /// This field works with the `GetCount` trait to provide flexible count specifications
    /// including fixed counts and ranges.
    ///
    /// # JSON Schema Mapping
    ///
    /// ```json
    /// {
    ///   "entity": {
    ///     "count": 5,        // Generate 5 entities
    ///     "fields": { ... }
    ///   }
    /// }
    /// ```
    ///
    /// Or omitted for single entity:
    /// ```json
    /// {
    ///   "entity": {
    ///     "fields": { ... }  // Generate single entity
    ///   }
    /// }
    /// ```
    pub count: Option<Count>,

    /// Optional seed for deterministic entity generation.
    ///
    /// When specified, this seed can be used to ensure reproducible entity generation
    /// for testing and debugging purposes. Currently preserved for future implementation
    /// of per-entity seeding.
    ///
    /// # Future Enhancement
    ///
    /// This field is planned for use in scenarios where specific entities need
    /// their own deterministic generation context, separate from the global
    /// generator configuration seed.
    ///
    /// # JSON Schema Mapping
    ///
    /// ```json
    /// {
    ///   "entity": {
    ///     "seed": 12345,
    ///     "fields": { ... }
    ///   }
    /// }
    /// ```
    #[serde(default)]
    pub seed: Option<u64>,

    /// Fields that must be unique across all generated entities.
    ///
    /// This vector specifies which field combinations must be unique when generating
    /// multiple entities. The uniqueness check creates a fingerprint from the specified
    /// fields and ensures no duplicates are generated.
    ///
    /// # Uniqueness Behavior
    ///
    /// - **Empty vector**: No uniqueness constraints (default)
    /// - **Single field**: Ensures that field is unique across entities
    /// - **Multiple fields**: Ensures the combination of fields is unique
    ///
    /// # Performance Considerations
    ///
    /// Uniqueness checking has a maximum retry limit (1000 attempts) to prevent
    /// infinite loops when constraints are too restrictive relative to the possible
    /// value space.
    ///
    /// # JSON Schema Mapping
    ///
    /// ```json
    /// {
    ///   "entity": {
    ///     "unique_by": ["id"],           // Single field uniqueness
    ///     "fields": { ... }
    ///   }
    /// }
    /// ```
    ///
    /// Or multiple fields:
    /// ```json
    /// {
    ///   "entity": {
    ///     "unique_by": ["user_id", "project_id"], // Composite uniqueness
    ///     "fields": { ... }
    ///   }
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Ensure email uniqueness
    /// unique_by: vec!["email".to_string()]
    ///
    /// // Ensure user+project combination uniqueness
    /// unique_by: vec!["user_id".to_string(), "project_id".to_string()]
    /// ```
    #[serde(default)]
    pub unique_by: Vec<String>,

    /// The collection of fields that make up the entity structure.
    ///
    /// This `IndexMap` defines the schema for the generated entities, mapping field
    /// names to their generation specifications. The use of `IndexMap` preserves
    /// the field ordering defined in the schema.
    ///
    /// # Field Types
    ///
    /// Each field can be any valid `Field` type:
    /// - **Primitive fields**: Strings, numbers, booleans
    /// - **Complex fields**: Nested entities, arrays
    /// - **Template fields**: String templates with placeholder substitution
    /// - **Reference fields**: Cross-references to other generated data
    /// - **Optional fields**: Fields that may or may not be present
    ///
    /// # Field Independence
    ///
    /// Each field is generated independently, allowing for:
    /// - Different fake data for each field
    /// - Independent randomization
    /// - Cross-field references when needed
    ///
    /// # JSON Schema Mapping
    ///
    /// ```json
    /// {
    ///   "entity": {
    ///     "fields": {
    ///       "id": "${uuid}",
    ///       "name": "${name.fullName}",
    ///       "age": {
    ///         "number": {
    ///           "min": 18,
    ///           "max": 65,
    ///           "integer": true
    ///         }
    ///       },
    ///       "address": {
    ///         "entity": {
    ///           "fields": {
    ///             "street": "${address.streetAddress}",
    ///             "city": "${address.cityName}"
    ///           }
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    pub fields: IndexMap<String, Field>,
}

impl JsonGenerator for Entity {
    /// Generates entities according to the entity specification with uniqueness constraints.
    ///
    /// This method creates either a single entity object or an array of entities,
    /// depending on the count specification. When uniqueness constraints are specified,
    /// it ensures that no duplicate entities are generated based on the unique field
    /// combinations.
    ///
    /// # Generation Process
    ///
    /// 1. **Count Determination**: Uses count specification to determine output format
    /// 2. **Uniqueness Setup**: Initializes tracking for unique field combinations
    /// 3. **Entity Generation**: Creates entities with uniqueness validation
    /// 4. **Output Formation**: Returns single object or array based on count
    ///
    /// # Arguments
    ///
    /// * `config` - Mutable reference to the generator configuration containing
    ///   RNG, fake data generators, and cross-reference state
    ///
    /// # Returns
    ///
    /// - **Single Entity**: `Value::Object` when count is None
    /// - **Entity Array**: `Value::Array` when count is specified
    ///
    /// # Uniqueness Algorithm
    ///
    /// When `unique_by` fields are specified:
    /// 1. Generate a candidate entity
    /// 2. Create a fingerprint from the unique fields
    /// 3. Check if fingerprint already exists
    /// 4. If unique, add to results; if duplicate, retry
    /// 5. Maximum 1000 attempts per entity to prevent infinite loops
    ///
    /// # Error Handling
    ///
    /// - **Uniqueness Failures**: Logs warning and stops generation after max attempts
    /// - **Missing Fields**: Includes "missing" in fingerprint for absent unique fields
    /// - **Complex Values**: Serializes complex field values for fingerprinting
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use jgd_rs::{Entity, Field, Count, JsonGenerator, GeneratorConfig};
    /// use indexmap::IndexMap;
    ///
    /// let mut config = GeneratorConfig::new("EN", Some(42));
    ///
    /// // Single entity generation
    /// let mut fields = IndexMap::new();
    /// fields.insert("name".to_string(), Field::Str("${name.fullName}".to_string()));
    /// fields.insert("age".to_string(), Field::I64(25));
    ///
    /// let entity = Entity {
    ///     count: None,  // Single entity
    ///     seed: None,
    ///     unique_by: vec![],
    ///     fields,
    /// };
    ///
    /// let result = entity.generate(&mut config);
    /// // Returns: {"name": "John Doe", "age": 25}
    ///
    /// // Array of unique entities
    /// let entity_array = Entity {
    ///     count: Some(Count::Fixed(3)),
    ///     unique_by: vec!["name".to_string()],
    ///     fields: fields.clone(),
    /// };
    ///
    /// let result = entity_array.generate(&mut config);
    /// // Returns: [{"name": "John", "age": 25}, {"name": "Jane", "age": 25}, ...]
    /// ```
    ///
    /// # Performance Notes
    ///
    /// - **Memory Efficiency**: Pre-allocates result vector with known capacity
    /// - **Uniqueness Overhead**: Fingerprinting and set operations add computational cost
    /// - **Retry Limits**: Maximum attempts prevent infinite loops in constrained scenarios
    /// - **Fingerprint Storage**: Memory usage grows with unique entity count
    ///
    /// # Uniqueness Constraints
    ///
    /// Consider the value space when setting uniqueness constraints:
    /// - **Large Value Space**: Easy to generate unique entities
    /// - **Small Value Space**: May hit retry limits with restrictive constraints
    /// - **Template Variety**: Ensure fake data templates provide sufficient variation
    fn generate(&self, config: &mut super::GeneratorConfig, local_config: Option<&mut LocalConfig>
        ) -> Result<Value, JgdGeneratorError> {
        let count_items = self.count.count(config);

        let mut items = Vec::with_capacity(count_items as usize);
        let mut unique_sets: HashMap<String, HashSet<String>> = HashMap::new();

        let rng = self.seed.map(StdRng::seed_from_u64);

        let mut local_config =
            LocalConfig::from_current_with_config(rng, count_items, local_config);

        let mut _attempts = 0;
        const MAX_ATTEMPTS: usize = 1000; // Prevent infinite loops

        for i in 0..count_items {
            let mut obj = None;
            local_config.set_index(i as usize);

            // Try to generate a unique object
            for _ in 0..MAX_ATTEMPTS {
                _attempts += 1;
                let candidate = self.fields.generate(config, Some(&mut local_config))?;

                if !self.unique_by.is_empty() {
                    let fp = fingerprint(&candidate, &self.unique_by);
                    let set = unique_sets.entry(self.unique_by.join("|"))
                        .or_default();

                    if !set.contains(&fp) {
                        set.insert(fp);
                        obj = Some(candidate);
                        break;
                    }
                    // If fingerprint already exists, try again
                } else {
                    // No uniqueness constraints
                    obj = Some(candidate);
                    break;
                }
            }

            if let Some(generated_obj) = obj {
                if self.count.is_none() {
                    return Ok(generated_obj);
                }
                items.push(generated_obj);
            } else {
                // Failed to generate a unique object after MAX_ATTEMPTS
                // This can happen if the uniqueness constraints are too restrictive
                // relative to the possible value space
                eprintln!("Warning: Failed to generate unique entity after {} attempts. Uniqueness constraints may be too restrictive.", MAX_ATTEMPTS);
                break;
            }
        }

        Ok(Value::Array(items))
    }
}

impl JsonGenerator for IndexMap<String, Entity> {
    /// Generates a collection of named entities and manages cross-references.
    ///
    /// This implementation generates multiple entities defined in an `IndexMap`,
    /// where each entity has a name and specification. It handles both the generation
    /// of individual entities and the management of cross-references between them.
    ///
    /// # Cross-Reference Management
    ///
    /// Each generated entity is stored in the global `gen_value` map using its name
    /// as the key. This enables:
    /// - References between entities using path notation
    /// - Consistent data relationships across the generated dataset
    /// - Template substitution with previously generated values
    ///
    /// # Generation Process
    ///
    /// 1. **Entity Generation**: Generate each named entity independently
    /// 2. **Cross-Reference Storage**: Store each entity in the global reference map
    /// 3. **Object Construction**: Build the final JSON object with all entities
    ///
    /// # Arguments
    ///
    /// * `config` - Mutable reference to the generator configuration containing
    ///   the global state for cross-references and generation context
    ///
    /// # Returns
    ///
    /// A `Value::Object` containing all generated entities, where keys are entity
    /// names and values are the generated entity data (objects or arrays).
    ///
    /// # Cross-Reference Usage
    ///
    /// Once entities are generated and stored, they can be referenced in templates:
    /// - `"author": "${users.name}"` - Reference user entity's name
    /// - `"post_author": "${posts.author}"` - Reference post entity's author
    ///
    /// # JGD Schema Example
    /// ```json
    /// {
    ///   "root": {
    ///     "users": {
    ///       "count": { "fixed": 2 },
    ///       "fields": {
    ///         "id": { "number": { "from": 1, "to": 1000, "type": "integer" } },
    ///         "name": { "fake": "name.firstName" }
    ///       }
    ///     },
    ///     "posts": {
    ///       "count": { "fixed": 3 },
    ///       "fields": {
    ///         "user_id": "${users.id}",
    ///         "title": { "fake": "lorem.sentence(3)" }
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use jgd_rs::{Entity, Field, JsonGenerator, GeneratorConfig};
    /// use indexmap::IndexMap;
    ///
    /// let mut config = GeneratorConfig::new("EN", Some(42));
    /// let mut entities = IndexMap::new();
    ///
    /// // Define user entity
    /// let mut user_fields = IndexMap::new();
    /// user_fields.insert("id".to_string(), Field::Str("${uuid}".to_string()));
    /// user_fields.insert("name".to_string(), Field::Str("${name.fullName}".to_string()));
    ///
    /// entities.insert("users".to_string(), Entity {
    ///     count: Some(Count::Fixed(2)),
    ///     seed: None,
    ///     unique_by: vec!["id".to_string()],
    ///     fields: user_fields,
    /// });
    ///
    /// // Define post entity that references users
    /// let mut post_fields = IndexMap::new();
    /// post_fields.insert("title".to_string(), Field::Str("${lorem.sentence}".to_string()));
    /// post_fields.insert("author_id".to_string(), Field::Ref {
    ///     r#ref: "users.id".to_string()
    /// });
    ///
    /// entities.insert("posts".to_string(), Entity {
    ///     count: Some(Count::Fixed(3)),
    ///     seed: None,
    ///     unique_by: vec![],
    ///     fields: post_fields,
    /// });
    ///
    /// let result = entities.generate(&mut config);
    /// // Returns: {
    /// //   "users": [{"id": "...", "name": "..."}, ...],
    /// //   "posts": [{"title": "...", "author_id": "..."}, ...]
    /// // }
    /// ```
    ///
    /// # Performance Notes
    ///
    /// - **Sequential Generation**: Entities are generated in insertion order
    /// - **Reference Storage**: Each entity is cloned for storage in gen_value
    /// - **Memory Usage**: Stores both final result and reference copies
    /// - **Order Dependency**: Earlier entities can be referenced by later ones
    fn generate(&self, config: &mut super::GeneratorConfig, local_config: Option<&mut LocalConfig>
        ) -> Result<Value, JgdGeneratorError> {
        let mut local_config =
            LocalConfig::from_current_with_config(None, 0, local_config);

        let mut map = serde_json::Map::new();
        for (name, entity) in self {
            local_config.entity_name = Some(name.clone());
            let generated = entity.generate(config, Some(&mut local_config))?;
            map.insert(name.clone(), generated.clone());

            config.gen_value.insert(name.clone(), generated);
        }

        Ok(Value::Object(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_spec::{GeneratorConfig, NumberSpec};

    fn create_test_config(seed: Option<u64>) -> GeneratorConfig {
        GeneratorConfig::new("EN", seed)
    }

    #[test]
    fn test_fingerprint_single_field() {
        let obj = serde_json::json!({
            "id": "12345",
            "name": "John Doe"
        });

        let fp = fingerprint(&obj, &["id".to_string()]);
        assert_eq!(fp, "id:12345");
    }

    #[test]
    fn test_fingerprint_multiple_fields() {
        let obj = serde_json::json!({
            "id": "12345",
            "name": "John Doe",
            "email": "john@example.com"
        });

        let fp = fingerprint(&obj, &["id".to_string(), "email".to_string()]);
        assert_eq!(fp, "id:12345|email:john@example.com");
    }

    #[test]
    fn test_fingerprint_missing_field() {
        let obj = serde_json::json!({
            "id": "12345",
            "name": "John Doe"
        });

        let fp = fingerprint(&obj, &["id".to_string(), "missing".to_string()]);
        assert_eq!(fp, "id:12345|missing:missing");
    }

    #[test]
    fn test_fingerprint_different_types() {
        let obj = serde_json::json!({
            "id": 12345,
            "active": true,
            "score": 98.5,
            "data": null,
            "tags": ["rust", "json"]
        });

        let fp = fingerprint(&obj, &[
            "id".to_string(),
            "active".to_string(),
            "score".to_string(),
            "data".to_string(),
            "tags".to_string()
        ]);

        assert!(fp.contains("id:12345"));
        assert!(fp.contains("active:true"));
        assert!(fp.contains("score:98.5"));
        assert!(fp.contains("data:null"));
        assert!(fp.contains("tags:"));
    }

    #[test]
    fn test_entity_single_generation() {
        let mut config = create_test_config(Some(42));
        let mut fields = IndexMap::new();
        fields.insert("name".to_string(), Field::Str("John".to_string()));
        fields.insert("age".to_string(), Field::I64(30));

        let entity = Entity {
            count: None,
            seed: None,
            unique_by: vec![],
            fields,
        };

        let result = entity.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            match result {
                Value::Object(obj) => {
                    assert_eq!(obj.get("name"), Some(&Value::String("John".to_string())));
                    assert_eq!(obj.get("age"), Some(&Value::Number(serde_json::Number::from(30))));
                }
                _ => panic!("Expected object for single entity"),
            }
        }
    }

    #[test]
    fn test_entity_array_generation() {
        let mut config = create_test_config(Some(42));
        let mut fields = IndexMap::new();
        fields.insert("id".to_string(), Field::Number {
            number: NumberSpec::new_integer(1.0, 1000.0)
        });

        let entity = Entity {
            count: Some(Count::Fixed(3)),
            seed: None,
            unique_by: vec![],
            fields,
        };

        let result = entity.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            match result {
                Value::Array(arr) => {
                    assert_eq!(arr.len(), 3);
                    for item in arr {
                        assert!(item.is_object());
                        if let Value::Object(obj) = item {
                            assert!(obj.contains_key("id"));
                            assert!(obj.get("id").unwrap().is_number());
                        }
                    }
                }
                _ => panic!("Expected array for entity with count"),
            }
        }
    }

    #[test]
    fn test_entity_uniqueness_constraint() {
        let mut config = create_test_config(Some(42));
        let mut fields = IndexMap::new();
        fields.insert("id".to_string(), Field::Number {
            number: NumberSpec::new_integer(1.0, 3.0) // Small range to force uniqueness testing
        });
        fields.insert("name".to_string(), Field::Str("Test".to_string()));

        let entity = Entity {
            count: Some(Count::Fixed(3)),
            seed: None,
            unique_by: vec!["id".to_string()],
            fields,
        };

        let result = entity.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            match result {
                Value::Array(arr) => {
                    // Should generate up to 3 unique entities (might be less due to small range)
                    assert!(arr.len() <= 3);

                    // Verify uniqueness
                    let mut seen_ids = std::collections::HashSet::new();
                    for item in arr {
                        if let Value::Object(obj) = item {
                            if let Some(Value::Number(id)) = obj.get("id") {
                                let id_value = id.as_i64().unwrap();
                                assert!(!seen_ids.contains(&id_value), "Duplicate ID found: {}", id_value);
                                seen_ids.insert(id_value);
                            }
                        }
                    }
                }
                _ => panic!("Expected array for entity with count"),
            }
        }
    }

    #[test]
    fn test_entity_composite_uniqueness() {
        let mut config = create_test_config(Some(42));
        let mut fields = IndexMap::new();
        fields.insert("category".to_string(), Field::Number {
            number: NumberSpec::new_integer(1.0, 2.0)
        });
        fields.insert("subcategory".to_string(), Field::Number {
            number: NumberSpec::new_integer(1.0, 2.0)
        });

        let entity = Entity {
            count: Some(Count::Fixed(5)),
            seed: None,
            unique_by: vec!["category".to_string(), "subcategory".to_string()],
            fields,
        };

        let result = entity.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            match result {
                Value::Array(arr) => {
                    // With 2x2 combinations, maximum should be 4 unique entities
                    assert!(arr.len() <= 4);

                    // Verify composite uniqueness
                    let mut seen_combinations = std::collections::HashSet::new();
                    for item in arr {
                        if let Value::Object(obj) = item {
                            let cat = obj.get("category").unwrap().as_i64().unwrap();
                            let subcat = obj.get("subcategory").unwrap().as_i64().unwrap();
                            let combination = (cat, subcat);

                            assert!(!seen_combinations.contains(&combination),
                                "Duplicate combination found: {:?}", combination);
                            seen_combinations.insert(combination);
                        }
                    }
                }
                _ => panic!("Expected array for entity with count"),
            }
        }
    }

    #[test]
    fn test_entity_map_generation() {
        let mut config = create_test_config(Some(42));
        let mut entities = IndexMap::new();

        // First entity
        let mut user_fields = IndexMap::new();
        user_fields.insert("id".to_string(), Field::I64(1));
        user_fields.insert("name".to_string(), Field::Str("User".to_string()));

        entities.insert("users".to_string(), Entity {
            count: Some(Count::Fixed(1)),
            seed: None,
            unique_by: vec![],
            fields: user_fields,
        });

        // Second entity
        let mut post_fields = IndexMap::new();
        post_fields.insert("title".to_string(), Field::Str("Post".to_string()));

        entities.insert("posts".to_string(), Entity {
            count: None,
            seed: None,
            unique_by: vec![],
            fields: post_fields,
        });

        let result = entities.generate(&mut config, None);
        assert!(result.is_ok());

        if let Ok(result) = result {
            match result {
                Value::Object(obj) => {
                    assert_eq!(obj.len(), 2);
                    assert!(obj.contains_key("users"));
                    assert!(obj.contains_key("posts"));

                    // Users should be an array
                    assert!(obj.get("users").unwrap().is_array());

                    // Posts should be an object (no count specified)
                    assert!(obj.get("posts").unwrap().is_object());
                }
                _ => panic!("Expected object for entity map"),
            }
        }
    }

    #[test]
    fn test_entity_cross_reference_storage() {
        let mut config = create_test_config(Some(42));
        let mut entities = IndexMap::new();

        let mut user_fields = IndexMap::new();
        user_fields.insert("name".to_string(), Field::Str("TestUser".to_string()));

        entities.insert("users".to_string(), Entity {
            count: None,
            seed: None,
            unique_by: vec![],
            fields: user_fields,
        });

        let _ = entities.generate(&mut config, None);

        // Verify that the entity was stored in gen_value for cross-references
        assert!(config.gen_value.contains_key("users"));

        let stored_user = config.gen_value.get("users").unwrap();
        match stored_user {
            Value::Object(obj) => {
                assert_eq!(obj.get("name"), Some(&Value::String("TestUser".to_string())));
            }
            _ => panic!("Expected stored user to be an object"),
        }
    }
}
