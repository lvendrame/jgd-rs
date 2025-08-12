use serde::Deserialize;
use serde_json::Value;
use crate::type_spec::{Count, Field, GetCount, JsonGenerator};

/// Specification for generating JSON arrays in JGD (JSON Generator Definition) schemas.
///
/// `ArraySpec` defines how to generate arrays of elements, including the type of
/// elements to generate and how many elements the array should contain. It provides
/// a flexible way to create arrays with consistent element types and configurable
/// cardinality.
///
/// # JGD Schema Integration
///
/// Arrays are a fundamental data structure in JGD schemas, used to represent
/// collections of similar items. The `ArraySpec` corresponds to the `array`
/// field type in JGD schema specifications.
///
/// # Components
///
/// - **Element Specification**: Defines what type of elements to generate (`of` field)
/// - **Count Specification**: Defines how many elements to generate (`count` field)
///
/// # JSON Schema Representation
///
/// In a JGD schema, an array specification is represented as:
///
/// ```json
/// {
///   "array": {
///     "of": "${name.firstName}",
///     "count": 5
///   }
/// }
/// ```
///
/// Or with a range count:
///
/// ```json
/// {
///   "array": {
///     "of": {
///       "number": {
///         "min": 1,
///         "max": 100,
///         "integer": true
///       }
///     },
///     "count": [3, 8]
///   }
/// }
/// ```
///
/// # Element Types
///
/// The `of` field can specify any valid JGD field type:
/// - Primitive types (strings, numbers, booleans)
/// - Complex types (objects, nested arrays)
/// - Fake data specifications
/// - Optional types
///
/// # Count Behavior
///
/// - **Specified Count**: Uses the provided count specification
/// - **Default Count**: When omitted, defaults to generating 1 element
/// - **Range Counts**: Randomly selects count within the specified range
/// - **Fixed Counts**: Always generates exactly the specified number
///
/// # Examples
///
/// ```rust,ignore
/// use jgd_lib::{ArraySpec, Field, Count, JsonGenerator, GeneratorConfig};
/// use serde_json::json;
///
/// let mut config = GeneratorConfig::new("EN", Some(42));
///
/// // Array of 5 strings with fake data template
/// let string_array = ArraySpec {
///     of: Box::new(Field::Str("${name.firstName}".to_string())),
///     count: Some(Count::Fixed(5)),
/// };
///
/// let result = string_array.generate(&mut config);
/// // Generates: ["John", "Jane", "Bob", "Alice", "Charlie"]
///
/// // Array with variable count using numbers
/// let number_array = ArraySpec {
///     of: Box::new(Field::Number {
///         number: NumberSpec::new_integer(1.0, 100.0)
///     }),
///     count: Some(Count::Range((2, 6))),
/// };
///
/// let result = number_array.generate(&mut config);
/// // Generates: [42, 17, 89] (length between 2-6)
/// ```
///
/// # Performance Considerations
///
/// - Pre-allocates the vector with the known capacity for efficiency
/// - Generates elements sequentially to maintain deterministic ordering
/// - Memory usage scales linearly with the generated count
///
/// # Use Cases
///
/// - **User Lists**: Generate arrays of user profiles or contact information
/// - **Product Catalogs**: Create collections of product specifications
/// - **Transaction Records**: Generate sequences of financial transactions
/// - **Test Data**: Create realistic datasets for application testing
/// - **Mock APIs**: Provide dynamic array responses for API development
#[derive(Debug, Deserialize, Clone)]
pub struct ArraySpec {
    /// The specification for elements that will populate the array.
    ///
    /// This field defines the type and generation rules for each element in the
    /// generated array. All elements in the array will be generated according
    /// to this specification, ensuring type consistency throughout the array.
    ///
    /// The field is boxed to allow for recursive type definitions (arrays of arrays)
    /// and to keep the `ArraySpec` struct size manageable.
    ///
    /// # Element Generation
    ///
    /// Each element is generated independently using this specification, which means:
    /// - Fake data will produce different values for each element
    /// - Random numbers will vary across elements (with proper seeding)
    /// - Optional fields may be present or absent independently
    ///
    /// # Supported Types
    ///
    /// Any valid `Field` type can be used as the element specification:
    /// - `Field::String` for text arrays
    /// - `Field::Number` for numeric arrays
    /// - `Field::Entity` for object arrays
    /// - `Field::Array` for nested array structures
    /// - `Field::Optional` for arrays with nullable elements
    ///
    /// # JSON Schema Mapping
    ///
    /// Maps to the `of` property in JGD array specifications:
    /// ```json
    /// {
    ///   "array": {
    ///     "of": { ... element specification ... }
    ///   }
    /// }
    /// ```
    pub of: Box<Field>,

    /// Optional count specification for the number of elements to generate.
    ///
    /// This field determines how many elements will be generated for the array.
    /// When not specified (defaults to `None`), the array will contain exactly
    /// one element.
    ///
    /// # Default Behavior
    ///
    /// The `#[serde(default)]` attribute ensures that if the `count` field is
    /// omitted from the JGD schema, it defaults to `None`, which through the
    /// `GetCount` implementation for `Option<Count>` results in a count of 1.
    ///
    /// # Count Types
    ///
    /// - **Fixed Count**: `Some(Count::Fixed(n))` generates exactly `n` elements
    /// - **Range Count**: `Some(Count::Range((min, max)))` generates between `min` and `max` elements
    /// - **Default Count**: `None` generates exactly 1 element
    ///
    /// # JSON Schema Mapping
    ///
    /// Maps to the optional `count` property in JGD array specifications:
    /// ```json
    /// {
    ///   "array": {
    ///     "of": { ... },
    ///     "count": 5           // Fixed count
    ///   }
    /// }
    /// ```
    ///
    /// Or omitted for default behavior:
    /// ```json
    /// {
    ///   "array": {
    ///     "of": { ... }
    ///     // count omitted - defaults to 1
    ///   }
    /// }
    /// ```
    #[serde(default)]
    pub count: Option<Count>
}

impl JsonGenerator for ArraySpec {
    /// Generates a JSON array according to the array specification.
    ///
    /// This method creates a JSON array by generating the specified number of
    /// elements, each conforming to the element specification. The generation
    /// process is deterministic when using a seeded generator configuration.
    ///
    /// # Generation Process
    ///
    /// 1. **Count Determination**: Uses the count specification to determine array length
    /// 2. **Memory Allocation**: Pre-allocates a vector with the exact capacity needed
    /// 3. **Element Generation**: Generates each element independently using the element spec
    /// 4. **Array Construction**: Constructs and returns a `Value::Array` containing all elements
    ///
    /// # Arguments
    ///
    /// * `config` - Mutable reference to the generator configuration containing
    ///   the random number generator, fake data generators, and generation state
    ///
    /// # Returns
    ///
    /// A `serde_json::Value::Array` containing the generated elements. The array
    /// length will be determined by the count specification, and each element
    /// will be generated according to the element specification.
    ///
    /// # Count Resolution
    ///
    /// The array length is determined by the count specification:
    /// - **Fixed Count**: Always generates exactly the specified number
    /// - **Range Count**: Randomly selects a count within the range
    /// - **Default (None)**: Generates exactly 1 element
    ///
    /// # Element Independence
    ///
    /// Each element is generated independently, which means:
    /// - Random values will differ between elements (with proper RNG state)
    /// - Fake data will produce varied realistic values
    /// - Optional fields may be present or absent independently
    /// - Cross-references within elements work independently
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use jgd_lib::{ArraySpec, Field, Count, NumberSpec, JsonGenerator, GeneratorConfig};
    /// use serde_json::{json, Value};
    ///
    /// let mut config = GeneratorConfig::new("EN", Some(42));
    ///
    /// // Generate array of 3 random integers
    /// let spec = ArraySpec {
    ///     of: Box::new(Field::Number(NumberSpec::new_integer(1.0, 100.0))),
    ///     count: Some(Count::Fixed(3)),
    /// };
    ///
    /// let result = spec.generate(&mut config);
    /// match result {
    ///     Value::Array(arr) => {
    ///         assert_eq!(arr.len(), 3);
    ///         for element in arr {
    ///             assert!(element.is_number());
    ///             let num = element.as_i64().unwrap();
    ///             assert!((1..=100).contains(&num));
    ///         }
    ///     }
    ///     _ => panic!("Expected array"),
    /// }
    /// ```
    ///
    /// # Performance Notes
    ///
    /// - **Memory Efficiency**: Pre-allocates vector to avoid reallocations
    /// - **Sequential Generation**: Elements are generated in order for predictable results
    /// - **Scalability**: Performance scales linearly with array size
    /// - **Memory Usage**: Total memory usage is proportional to element count and element size
    ///
    /// # Error Handling
    ///
    /// This method assumes that:
    /// - The element specification is valid and can generate values
    /// - The count specification produces valid non-negative counts
    /// - The generator configuration is properly initialized
    ///
    /// Invalid configurations may result in panics or unexpected behavior.
    fn generate(&self, config: &mut super::GeneratorConfig) -> Value {
        let len = self.count.count(config);
        let mut arr = Vec::with_capacity(len as usize);
        for _ in 0..len {
            arr.push(self.of.generate(config));
        }

        Value::Array(arr)
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
    fn test_array_spec_with_fixed_count() {
        let mut config = create_test_config(Some(42));

        let spec = ArraySpec {
            of: Box::new(Field::Number {
                number: NumberSpec::new_integer(1.0, 10.0)
            }),
            count: Some(Count::Fixed(3)),
        };

        let result = spec.generate(&mut config);

        match result {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 3);
                for element in arr {
                    assert!(element.is_number());
                }
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_array_spec_with_range_count() {
        let mut config = create_test_config(Some(42));

        let spec = ArraySpec {
            of: Box::new(Field::Number {
                number: NumberSpec::new_integer(1.0, 100.0)
            }),
            count: Some(Count::Range((2, 5))),
        };

        let result = spec.generate(&mut config);

        match result {
            Value::Array(arr) => {
                assert!((2..=5).contains(&arr.len()));
                for element in arr {
                    assert!(element.is_number());
                }
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_array_spec_with_default_count() {
        let mut config = create_test_config(Some(42));

        let spec = ArraySpec {
            of: Box::new(Field::Number {
                number: NumberSpec::new_integer(1.0, 100.0)
            }),
            count: None, // Should default to 1
        };

        let result = spec.generate(&mut config);

        match result {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 1);
                assert!(arr[0].is_number());
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_array_spec_with_zero_count() {
        let mut config = create_test_config(Some(42));

        let spec = ArraySpec {
            of: Box::new(Field::Number {
                number: NumberSpec::new_integer(1.0, 100.0)
            }),
            count: Some(Count::Fixed(0)),
        };

        let result = spec.generate(&mut config);

        match result {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 0);
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_array_spec_deterministic_with_seed() {
        let spec = ArraySpec {
            of: Box::new(Field::Number {
                number: NumberSpec::new_integer(1.0, 100.0)
            }),
            count: Some(Count::Fixed(3)),
        };

        let mut config1 = create_test_config(Some(42));
        let mut config2 = create_test_config(Some(42));

        let result1 = spec.generate(&mut config1);
        let result2 = spec.generate(&mut config2);

        // Same seed should produce identical arrays
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_array_spec_different_seeds_different_results() {
        let spec = ArraySpec {
            of: Box::new(Field::Number {
                number: NumberSpec::new_integer(1.0, 100.0)
            }),
            count: Some(Count::Range((3, 5))),
        };

        let mut config1 = create_test_config(Some(42));
        let mut config2 = create_test_config(Some(123));

        let result1 = spec.generate(&mut config1);
        let result2 = spec.generate(&mut config2);

        // Different seeds might produce different results
        // At minimum, both should be valid arrays
        assert!(result1.is_array());
        assert!(result2.is_array());
    }

    #[test]
    fn test_array_spec_clone() {
        let spec = ArraySpec {
            of: Box::new(Field::Number {
                number: NumberSpec::new_integer(1.0, 100.0)
            }),
            count: Some(Count::Fixed(2)),
        };

        let cloned_spec = spec.clone();
        let mut config = create_test_config(Some(42));

        let result1 = spec.generate(&mut config);
        let result2 = cloned_spec.generate(&mut config);

        // Both should generate arrays (though content may differ due to RNG state)
        assert!(result1.is_array());
        assert!(result2.is_array());
    }

    #[test]
    fn test_array_spec_debug() {
        let spec = ArraySpec {
            of: Box::new(Field::Number {
                number: NumberSpec::new_integer(1.0, 100.0)
            }),
            count: Some(Count::Fixed(3)),
        };

        // Test that Debug is implemented (should not panic)
        let debug_output = format!("{:?}", spec);
        assert!(debug_output.contains("ArraySpec"));
    }

    #[test]
    fn test_array_spec_large_count() {
        let mut config = create_test_config(Some(42));

        let spec = ArraySpec {
            of: Box::new(Field::Number {
                number: NumberSpec::new_integer(1.0, 10.0)
            }),
            count: Some(Count::Fixed(100)),
        };

        let result = spec.generate(&mut config);

        match result {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 100);
                for element in arr {
                    assert!(element.is_number());
                    let num = element.as_i64().unwrap();
                    assert!((1..=10).contains(&num));
                }
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_array_spec_element_independence() {
        let mut config = create_test_config(Some(42));

        let spec = ArraySpec {
            of: Box::new(Field::Number {
                number: NumberSpec::new_integer(1.0, 1000.0)
            }),
            count: Some(Count::Fixed(10)),
        };

        let result = spec.generate(&mut config);

        match result {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 10);

                // Convert to numbers for comparison
                let numbers: Vec<i64> = arr.iter()
                    .map(|v| v.as_i64().unwrap())
                    .collect();

                // With a large range and good RNG, we should get some variety
                // (This test might occasionally fail with very bad luck, but is highly unlikely)
                let unique_count = numbers.iter().collect::<std::collections::HashSet<_>>().len();
                assert!(unique_count > 1, "Expected some variety in generated numbers");
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_array_spec_with_string_elements() {
        let mut config = create_test_config(Some(42));

        let spec = ArraySpec {
            of: Box::new(Field::Str("test_value".to_string())),
            count: Some(Count::Fixed(2)),
        };

        let result = spec.generate(&mut config);

        match result {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 2);
                for element in arr {
                    assert_eq!(element, Value::String("test_value".to_string()));
                }
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_array_spec_with_mixed_primitive_types() {
        let mut config = create_test_config(Some(42));

        // Test with boolean
        let bool_spec = ArraySpec {
            of: Box::new(Field::Bool(true)),
            count: Some(Count::Fixed(1)),
        };

        let result = bool_spec.generate(&mut config);
        match result {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 1);
                assert_eq!(arr[0], Value::Bool(true));
            }
            _ => panic!("Expected array"),
        }

        // Test with null
        let null_spec = ArraySpec {
            of: Box::new(Field::Null),
            count: Some(Count::Fixed(1)),
        };

        let result = null_spec.generate(&mut config);
        match result {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 1);
                assert_eq!(arr[0], Value::Null);
            }
            _ => panic!("Expected array"),
        }
    }
}
