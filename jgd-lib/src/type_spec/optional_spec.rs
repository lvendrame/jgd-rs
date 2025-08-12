//! # Optional Specification Module
//!
//! This module provides probability-based field generation through the `OptionalSpec` struct.
//! It enables conditional generation of JSON values where fields may or may not be present
//! based on configurable probability thresholds.
//!
//! ## Overview
//!
//! The `OptionalSpec` allows you to define fields that are generated conditionally:
//! - Values are generated with a specified probability (0.0 to 1.0)
//! - When the probability condition is not met, `null` is generated instead
//! - Useful for modeling real-world data where some fields are optional or missing
//!
//! ## Use Cases
//!
//! - **User profiles**: Optional bio, avatar, or social media links
//! - **Product data**: Optional descriptions, images, or categories
//! - **Configuration objects**: Optional settings with default behaviors
//! - **API responses**: Fields that may be present based on user permissions or data availability

use rand::Rng;
use serde::Deserialize;
use serde_json::Value;
use crate::type_spec::{Field, JsonGenerator};

/// Default probability value when not specified in the JSON schema.
///
/// This function provides a 50% probability (0.5) as a reasonable default
/// for optional field generation, ensuring balanced presence/absence in generated data.
fn default_prob() -> f64 {
    0.5
}

/// Specification for conditional field generation based on probability.
///
/// `OptionalSpec` wraps any field type and generates its value conditionally based on
/// a probability threshold. This enables modeling of optional or sparse data patterns
/// commonly found in real-world JSON datasets.
///
/// # Fields
///
/// - **`of`**: The wrapped field specification that will be generated when the probability condition is met
/// - **`prob`**: Probability value (0.0 to 1.0) determining how often the field is generated
///
/// # Probability Behavior
///
/// - **0.0**: Field is never generated (always `null`)
/// - **0.5**: Field is generated 50% of the time (default)
/// - **1.0**: Field is always generated (never `null`)
/// - **Other values**: Proportional probability of generation
///
/// # JGD Schema Examples
///
/// ## Basic Optional String
/// ```json
/// {
///   "bio": {
///     "optional": {
///       "of": "${lorem.paragraph}",
///       "prob": 0.7
///     }
///   }
/// }
/// ```
///
/// ## Optional Nested Object
/// ```json
/// {
///   "profile": {
///     "optional": {
///       "prob": 0.8,
///       "of": {
///         "fields": {
///           "avatar": "${internet.avatar}",
///           "social": {
///             "fields": {
///               "twitter": "${internet.userName}",
///               "github": "${internet.userName}"
///             }
///           }
///         }
///       }
///     }
///   }
/// }
/// ```
///
/// ## Optional Array
/// ```json
/// {
///   "tags": {
///     "optional": {
///       "prob": 0.6,
///       "of": {
///         "array": {
///           "count": [1, 5],
///           "of": "${lorem.word}"
///         }
///       }
///     }
///   }
/// }
/// ```
///
/// # Default Probability
///
/// When `prob` is not specified in the JSON schema, it defaults to `0.5` (50% probability).
/// This provides balanced optional field generation for realistic data patterns.
///
/// # Deserialization
///
/// The struct uses Serde's `#[serde(default)]` attribute with a custom default function
/// to provide the 0.5 probability when not explicitly specified in the input JSON.
#[derive(Debug, Deserialize, Clone)]
pub struct OptionalSpec {
    /// The field specification to generate when the probability condition is met.
    ///
    /// This boxed field can be any valid `Field` type, including primitives, complex objects,
    /// arrays, or even nested optional specifications for multi-level conditional generation.
    pub of: Box<Field>,

    /// Probability threshold for field generation (0.0 to 1.0).
    ///
    /// - Values closer to 0.0 make the field less likely to be generated
    /// - Values closer to 1.0 make the field more likely to be generated
    /// - Defaults to 0.5 when not specified in the JSON schema
    ///
    /// # Valid Range
    /// While the type allows any `f64` value, probabilities should typically be
    /// between 0.0 and 1.0 for meaningful probability behavior.
    #[serde(default = "default_prob")]
    pub prob: f64
}

impl JsonGenerator for OptionalSpec {
    /// Generates a JSON value conditionally based on the configured probability.
    ///
    /// This method implements the core probability-based generation logic for optional fields.
    /// It uses the random number generator from the configuration to determine whether
    /// to generate the wrapped field or return `null`.
    ///
    /// # Parameters
    /// - `config`: Mutable reference to generator configuration containing the RNG state
    ///
    /// # Returns
    /// - `Value`: Either the generated field value or `Value::Null` based on probability
    ///
    /// # Algorithm
    ///
    /// 1. Generate a random floating-point number between 0.0 and 1.0
    /// 2. Compare it against the configured probability threshold
    /// 3. If random value < probability: generate the wrapped field
    /// 4. Otherwise: return `Value::Null`
    ///
    /// # Probability Distribution
    ///
    /// The method uses uniform random distribution, ensuring that over many generations,
    /// the actual presence rate will converge to the specified probability value.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use jgd_lib::{OptionalSpec, Field, GeneratorConfig};
    ///
    /// // 80% chance of generating a string
    /// let optional_field = OptionalSpec {
    ///     of: Box::new(Field::Str("Hello World".to_string())),
    ///     prob: 0.8,
    /// };
    ///
    /// let mut config = GeneratorConfig::new("EN", Some(42));
    /// let result = optional_field.generate(&mut config);
    /// // Result: ~80% chance of Value::String("Hello World"), ~20% chance of Value::Null
    /// ```
    ///
    /// # Deterministic Behavior
    ///
    /// When using a seeded random number generator, the probability outcomes become
    /// deterministic and reproducible, which is useful for testing and consistent
    /// data generation across runs.
    fn generate(&self, config: &mut super::GeneratorConfig) -> Value {
        if config.rng.random::<f64>() < self.prob {
            self.of.generate(config)
        } else {
            Value::Null
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_spec::GeneratorConfig;

    fn create_test_config(seed: Option<u64>) -> GeneratorConfig {
        GeneratorConfig::new("EN", seed)
    }

    #[test]
    fn test_default_prob_function() {
        assert_eq!(default_prob(), 0.5);
    }

    #[test]
    fn test_optional_spec_always_generate() {
        let mut config = create_test_config(Some(42));

        let optional = OptionalSpec {
            of: Box::new(Field::Str("test".to_string())),
            prob: 1.0, // Always generate
        };

        // Test multiple times to ensure it always generates
        for _ in 0..10 {
            let result = optional.generate(&mut config);
            assert_eq!(result, Value::String("test".to_string()));
        }
    }

    #[test]
    fn test_optional_spec_never_generate() {
        let mut config = create_test_config(Some(42));

        let optional = OptionalSpec {
            of: Box::new(Field::Str("test".to_string())),
            prob: 0.0, // Never generate
        };

        // Test multiple times to ensure it never generates
        for _ in 0..10 {
            let result = optional.generate(&mut config);
            assert_eq!(result, Value::Null);
        }
    }

    #[test]
    fn test_optional_spec_probability_distribution() {
        let mut config = create_test_config(Some(42));

        let optional = OptionalSpec {
            of: Box::new(Field::Str("test".to_string())),
            prob: 0.7, // 70% chance
        };

        let mut generated_count = 0;
        let total_attempts = 1000;

        for _ in 0..total_attempts {
            let result = optional.generate(&mut config);
            if result != Value::Null {
                generated_count += 1;
            }
        }

        // With a large sample, the actual percentage should be close to 70%
        // Allow for some variance due to randomness (±10%)
        let actual_probability = generated_count as f64 / total_attempts as f64;
        assert!((0.6..=0.8).contains(&actual_probability),
               "Expected probability around 0.7, got {}", actual_probability);
    }

    #[test]
    fn test_optional_spec_with_different_field_types() {
        let mut config = create_test_config(Some(42));

        // Test with boolean field
        let bool_optional = OptionalSpec {
            of: Box::new(Field::Bool(true)),
            prob: 1.0,
        };

        let result = bool_optional.generate(&mut config);
        assert_eq!(result, Value::Bool(true));

        // Test with integer field
        let int_optional = OptionalSpec {
            of: Box::new(Field::I64(42)),
            prob: 1.0,
        };

        let result = int_optional.generate(&mut config);
        assert_eq!(result, Value::Number(serde_json::Number::from(42)));

        // Test with null field
        let null_optional = OptionalSpec {
            of: Box::new(Field::Null),
            prob: 1.0,
        };

        let result = null_optional.generate(&mut config);
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_optional_spec_deterministic_with_seed() {
        let optional = OptionalSpec {
            of: Box::new(Field::Str("test".to_string())),
            prob: 0.5,
        };

        // Generate with same seed multiple times
        let results1: Vec<Value> = (0..10).map(|_| {
            let mut config = create_test_config(Some(42));
            optional.generate(&mut config)
        }).collect();

        let results2: Vec<Value> = (0..10).map(|_| {
            let mut config = create_test_config(Some(42));
            optional.generate(&mut config)
        }).collect();

        // Results should be identical with same seed
        assert_eq!(results1, results2);
    }

    #[test]
    fn test_optional_spec_different_seeds_different_results() {
        let optional = OptionalSpec {
            of: Box::new(Field::Str("test".to_string())),
            prob: 0.5,
        };

        let mut config1 = create_test_config(Some(42));
        let mut config2 = create_test_config(Some(24));

        let results1: Vec<Value> = (0..20).map(|_| optional.generate(&mut config1)).collect();
        let results2: Vec<Value> = (0..20).map(|_| optional.generate(&mut config2)).collect();

        // Different seeds should produce different results
        assert_ne!(results1, results2);
    }

    #[test]
    fn test_optional_spec_clone() {
        let original = OptionalSpec {
            of: Box::new(Field::Str("test".to_string())),
            prob: 0.7,
        };

        let cloned = original.clone();

        assert_eq!(cloned.prob, original.prob);
        // Test that the cloned field generates the same type
        let mut config = create_test_config(Some(42));
        let original_result = original.generate(&mut config);
        let mut config2 = create_test_config(Some(42)); // Same seed
        let cloned_result = cloned.generate(&mut config2);

        // With same seed, should get same result
        assert_eq!(original_result, cloned_result);
    }

    #[test]
    fn test_optional_spec_debug() {
        let optional = OptionalSpec {
            of: Box::new(Field::Bool(true)),
            prob: 0.8,
        };

        let debug_str = format!("{:?}", optional);
        assert!(debug_str.contains("OptionalSpec"));
        assert!(debug_str.contains("0.8"));
    }

    #[test]
    fn test_optional_spec_edge_case_probabilities() {
        let mut config = create_test_config(Some(42));

        // Test with very small positive probability
        let tiny_prob = OptionalSpec {
            of: Box::new(Field::Str("test".to_string())),
            prob: 0.001,
        };

        // Should mostly generate null, but might occasionally generate value
        let mut null_count = 0;
        for _ in 0..100 {
            let result = tiny_prob.generate(&mut config);
            if result == Value::Null {
                null_count += 1;
            }
        }

        // With 0.001 probability, expect mostly nulls
        assert!(null_count > 90);

        // Test with very high probability
        let high_prob = OptionalSpec {
            of: Box::new(Field::Str("test".to_string())),
            prob: 0.999,
        };

        let mut value_count = 0;
        for _ in 0..100 {
            let result = high_prob.generate(&mut config);
            if result != Value::Null {
                value_count += 1;
            }
        }

        // With 0.999 probability, expect mostly values
        assert!(value_count > 90);
    }

    #[test]
    fn test_optional_spec_nested_optionals() {
        let mut config = create_test_config(Some(42));

        // Create nested optional: optional of optional
        let inner_optional = OptionalSpec {
            of: Box::new(Field::Str("inner".to_string())),
            prob: 0.8,
        };

        let outer_optional = OptionalSpec {
            of: Box::new(Field::Optional { optional: inner_optional }),
            prob: 0.8,
        };

        // Generate several times to test all possible outcomes
        let mut outcomes = std::collections::HashMap::new();
        for _ in 0..100 {
            let result = outer_optional.generate(&mut config);
            let outcome = match result {
                Value::Null => "outer_null",
                Value::String(_) => "inner_string",
                _ => "unknown",
            };
            *outcomes.entry(outcome).or_insert(0) += 1;
        }

        // Should have both null outcomes and string outcomes
        // With 0.8 * 0.8 = 0.64 chance of string, rest null
        assert!(outcomes.contains_key("outer_null"));
        // Note: inner nulls would also appear as outer nulls in this case
    }

    #[test]
    fn test_optional_spec_with_complex_field() {
        let mut config = create_test_config(Some(42));

        // Test with a more complex field type (though we can't test Entity directly here,
        // we can test with other complex field types when available)
        use crate::type_spec::{ArraySpec, Count};

        let array_spec = ArraySpec {
            count: Some(Count::Fixed(3)),
            of: Box::new(Field::Str("item".to_string())),
        };

        let optional = OptionalSpec {
            of: Box::new(Field::Array { array: array_spec }),
            prob: 1.0,
        };

        let result = optional.generate(&mut config);

        // Should generate an array when prob is 1.0
        assert!(result.is_array());
        if let Value::Array(arr) = result {
            assert_eq!(arr.len(), 3);
            for item in arr {
                assert_eq!(item, Value::String("item".to_string()));
            }
        }
    }

    #[test]
    fn test_optional_spec_probability_boundaries() {
        let mut config = create_test_config(Some(42));

        // Test exactly at boundary values
        let boundary_cases = vec![0.0, 0.25, 0.5, 0.75, 1.0];

        for prob in boundary_cases {
            let optional = OptionalSpec {
                of: Box::new(Field::I64(1)),
                prob,
            };

            let result = optional.generate(&mut config);

            // All probabilities should produce valid results (either value or null)
            assert!(result == Value::Number(serde_json::Number::from(1)) || result == Value::Null);
        }
    }

    #[test]
    fn test_optional_spec_consistent_rng_usage() {
        // Test that the optional spec properly uses the RNG from config
        let optional = OptionalSpec {
            of: Box::new(Field::Str("test".to_string())),
            prob: 0.5,
        };

        // Create config and advance RNG state
        let mut config = create_test_config(Some(42));
        let _ = config.rng.random::<f64>(); // Advance RNG state

        let result1 = optional.generate(&mut config);

        // Create fresh config with same seed
        let mut config2 = create_test_config(Some(42));
        let _ = config2.rng.random::<f64>(); // Advance RNG state same way

        let result2 = optional.generate(&mut config2);

        // Should get same result when RNG is in same state
        assert_eq!(result1, result2);
    }
}
