use rand::Rng;
use serde::Deserialize;

use crate::type_spec::GeneratorConfig;

/// Represents count specifications for JGD (JSON Generator Definition) entities.
///
/// `Count` defines how many items should be generated for arrays, collections,
/// or repeated elements in JGD schemas. It supports both fixed counts and
/// dynamic ranges, allowing for flexible data generation scenarios.
///
/// # JGD Schema Usage
///
/// In JGD schemas, count specifications control the cardinality of generated
/// data structures. They are commonly used with:
/// - Array generation (how many array elements to create)
/// - Entity repetition (how many instances of an entity to generate)
/// - Collection sizing (dynamic sizing of data collections)
///
/// # Variants
///
/// - **Fixed(u64)**: Generates exactly the specified number of items
/// - **Range((u64, u64))**: Generates a random number of items within the range (inclusive)
///
/// # Serialization Format
///
/// The enum uses `#[serde(untagged)]` for natural JSON representation:
/// - Fixed count: `42` (just a number)
/// - Range count: `[5, 10]` (array with min and max values)
///
/// # Examples
///
/// ```rust,ignore
/// use jgd_lib::{Count, GetCount, GeneratorConfig};
///
/// let mut config = GeneratorConfig::new("EN", Some(42));
///
/// // Fixed count - always generates exactly 5 items
/// let fixed = Count::Fixed(5);
/// assert_eq!(fixed.count(&mut config), 5);
///
/// // Range count - generates between 1 and 10 items
/// let range = Count::Range((1, 10));
/// let result = range.count(&mut config);
/// assert!((1..=10).contains(&result));
/// ```
///
/// # JSON Schema Examples
///
/// ```json
/// {
///   "array": {
///     "count": 5,           // Fixed count
///     "element": { ... }
///   }
/// }
/// ```
///
/// ```json
/// {
///   "array": {
///     "count": [1, 10],     // Range count
///     "element": { ... }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Count {
    /// A fixed count that always generates exactly the specified number of items.
    ///
    /// This variant is used when you need a consistent, predictable number of
    /// generated items. The value represents the exact count to generate.
    ///
    /// # JSON Representation
    /// ```json
    /// 42
    /// ```
    ///
    /// # Use Cases
    /// - Testing scenarios requiring consistent data sizes
    /// - Schema definitions with fixed requirements
    /// - Performance testing with known data volumes
    Fixed(u64),

    /// A range count that generates a random number of items within the specified bounds.
    ///
    /// The tuple contains `(min, max)` values where both bounds are inclusive.
    /// The actual count is randomly determined each time `count()` is called.
    ///
    /// # JSON Representation
    /// ```json
    /// [5, 15]
    /// ```
    ///
    /// # Use Cases
    /// - Realistic data generation with natural variation
    /// - Stress testing with variable load sizes
    /// - Simulating real-world data patterns
    Range((u64,u64))
}

/// Trait for extracting count values from count specifications.
///
/// This trait provides a unified interface for obtaining count values from
/// different types that can specify counts in JGD generation. It abstracts
/// over the different ways counts can be represented and ensures consistent
/// behavior across the generation system.
///
/// # Design Philosophy
///
/// The trait allows for extensible count specifications while maintaining
/// a simple interface. It takes a mutable reference to `GeneratorConfig`
/// to access the random number generator for range-based counts.
///
/// # Implementations
///
/// - `Count`: Direct count specification (fixed or range)
/// - `Option<Count>`: Optional count with default fallback
///
/// # Examples
///
/// ```rust,ignore
/// use jgd_lib::{Count, GetCount, GeneratorConfig};
///
/// let mut config = GeneratorConfig::new("EN", Some(42));
///
/// // Using Count directly
/// let count = Count::Range((1, 5));
/// let result = count.count(&mut config);
///
/// // Using Option<Count> with Some
/// let opt_count = Some(Count::Fixed(3));
/// let result = opt_count.count(&mut config);
///
/// // Using Option<Count> with None (defaults to 1)
/// let none_count: Option<Count> = None;
/// let result = none_count.count(&mut config); // Returns 1
/// ```
pub trait GetCount {
    /// Generates a count value using the provided generator configuration.
    ///
    /// This method produces a `u64` count value according to the implementing
    /// type's specification. For fixed counts, it returns the constant value.
    /// For range counts, it uses the random number generator to produce a
    /// value within the specified bounds.
    ///
    /// # Arguments
    ///
    /// * `config` - Mutable reference to the generator configuration containing
    ///   the random number generator and other generation context
    ///
    /// # Returns
    ///
    /// A `u64` representing the number of items to generate
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use jgd_lib::{Count, GetCount, GeneratorConfig};
    ///
    /// let mut config = GeneratorConfig::new("EN", Some(42));
    ///
    /// let fixed = Count::Fixed(5);
    /// assert_eq!(fixed.count(&mut config), 5);
    ///
    /// let range = Count::Range((1, 10));
    /// let result = range.count(&mut config);
    /// assert!((1..=10).contains(&result));
    /// ```
    fn count(&self, config: &mut GeneratorConfig) -> u64;
}

impl GetCount for Count {
    /// Generates a count value based on the Count variant.
    ///
    /// This implementation handles both fixed and range count specifications:
    ///
    /// - **Fixed**: Returns the constant value immediately
    /// - **Range**: Uses the RNG to generate a random value within the inclusive range
    ///
    /// # Deterministic Behavior
    ///
    /// When the `GeneratorConfig` is created with a seed, range-based counts
    /// will produce deterministic results, which is useful for:
    /// - Testing with reproducible data
    /// - Debugging generation issues
    /// - Creating consistent development datasets
    ///
    /// # Range Semantics
    ///
    /// Range bounds are inclusive on both ends. A range of `(5, 10)` can
    /// generate any value from 5 to 10, including both 5 and 10.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use jgd_lib::{Count, GetCount, GeneratorConfig};
    ///
    /// let mut config = GeneratorConfig::new("EN", Some(42));
    ///
    /// // Fixed count always returns the same value
    /// let fixed = Count::Fixed(7);
    /// assert_eq!(fixed.count(&mut config), 7);
    /// assert_eq!(fixed.count(&mut config), 7); // Always 7
    ///
    /// // Range count varies within bounds
    /// let range = Count::Range((3, 8));
    /// for _ in 0..10 {
    ///     let result = range.count(&mut config);
    ///     assert!((3..=8).contains(&result));
    /// }
    /// ```
    fn count(&self, config: &mut GeneratorConfig) -> u64 {
        match self {
            Count::Fixed(n) => *n,
            Count::Range((a, b)) => config.rng.random_range(*a..=*b),
        }
    }
}

impl GetCount for Option<Count> {
    /// Generates a count value from an optional Count specification.
    ///
    /// This implementation provides convenient default behavior for optional
    /// count specifications commonly used in JGD schemas where count may be
    /// omitted.
    ///
    /// # Behavior
    ///
    /// - **Some(count)**: Delegates to the wrapped `Count`'s implementation
    /// - **None**: Defaults to `Count::Fixed(1)`, generating exactly 1 item
    ///
    /// # Default Semantics
    ///
    /// The default count of 1 is chosen because:
    /// - It's the most common case for single item generation
    /// - It avoids empty collections which might break downstream processing
    /// - It provides sensible behavior for optional array sizes
    ///
    /// # JGD Schema Integration
    ///
    /// This implementation allows JGD schemas to omit count specifications
    /// when single-item generation is desired, making schemas more concise:
    ///
    /// ```json
    /// {
    ///   "array": {
    ///     // count omitted - defaults to 1
    ///     "element": { "type": "string" }
    ///   }
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use jgd_lib::{Count, GetCount, GeneratorConfig};
    ///
    /// let mut config = GeneratorConfig::new("EN", Some(42));
    ///
    /// // Some count uses the wrapped specification
    /// let some_count = Some(Count::Range((2, 5)));
    /// let result = some_count.count(&mut config);
    /// assert!((2..=5).contains(&result));
    ///
    /// // None count defaults to 1
    /// let none_count: Option<Count> = None;
    /// assert_eq!(none_count.count(&mut config), 1);
    /// ```
    fn count(&self, config: &mut GeneratorConfig) -> u64 {
        self.clone().unwrap_or(Count::Fixed(1)).count(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_spec::GeneratorConfig;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn create_test_config(seed: Option<u64>) -> GeneratorConfig {
        GeneratorConfig::new("EN", seed)
    }

    #[test]
    fn test_count_fixed() {
        let mut config = create_test_config(None);
        let count = Count::Fixed(5);

        assert_eq!(count.count(&mut config), 5);
        // Should always return the same value
        assert_eq!(count.count(&mut config), 5);
        assert_eq!(count.count(&mut config), 5);
    }

    #[test]
    fn test_count_fixed_zero() {
        let mut config = create_test_config(None);
        let count = Count::Fixed(0);

        assert_eq!(count.count(&mut config), 0);
    }

    #[test]
    fn test_count_fixed_large_number() {
        let mut config = create_test_config(None);
        let count = Count::Fixed(1000);

        assert_eq!(count.count(&mut config), 1000);
    }

    #[test]
    fn test_count_range_single_value() {
        let mut config = create_test_config(None);
        let count = Count::Range((5, 5));

        // Range with same min and max should always return that value
        assert_eq!(count.count(&mut config), 5);
        assert_eq!(count.count(&mut config), 5);
    }

    #[test]
    fn test_count_range_within_bounds() {
        let mut config = create_test_config(None);
        let count = Count::Range((1, 10));

        // Test multiple times to ensure values are within range
        for _ in 0..20 {
            let result = count.count(&mut config);
            assert!((1..=10).contains(&result), "Value {} not in range [1, 10]", result);
        }
    }

    #[test]
    fn test_count_range_zero_to_value() {
        let mut config = create_test_config(None);
        let count = Count::Range((0, 3));

        for _ in 0..20 {
            let result = count.count(&mut config);
            assert!(result <= 3, "Value {} exceeds maximum 3", result);
        }
    }

    #[test]
    fn test_count_range_large_range() {
        let mut config = create_test_config(None);
        let count = Count::Range((100, 200));

        for _ in 0..10 {
            let result = count.count(&mut config);
            assert!((100..=200).contains(&result), "Value {} not in range [100, 200]", result);
        }
    }

    #[test]
    fn test_count_range_deterministic_with_seed() {
        // Test that the same seed produces the same sequence
        let count = Count::Range((1, 100));

        let mut config1 = create_test_config(Some(21));
        let mut config2 = create_test_config(Some(21));

        // Both configs use the same seed, so should produce same sequence
        let result1 = count.count(&mut config1);
        let result2 = count.count(&mut config2);

        assert_eq!(result1, result2, "Same seed should produce same results");
    }

    #[test]
    fn test_option_count_some_fixed() {
        let mut config = create_test_config(None);
        let opt_count = Some(Count::Fixed(7));

        assert_eq!(opt_count.count(&mut config), 7);
    }

    #[test]
    fn test_option_count_some_range() {
        let mut config = create_test_config(None);
        let opt_count = Some(Count::Range((2, 8)));

        for _ in 0..10 {
            let result = opt_count.count(&mut config);
            assert!((2..=8).contains(&result), "Value {} not in range [2, 8]", result);
        }
    }

    #[test]
    fn test_option_count_none() {
        let mut config = create_test_config(None);
        let opt_count: Option<Count> = None;

        // None should default to Count::Fixed(1)
        assert_eq!(opt_count.count(&mut config), 1);
        assert_eq!(opt_count.count(&mut config), 1);
    }

    #[test]
    fn test_count_clone() {
        let count = Count::Fixed(42);
        let cloned = count.clone();

        let mut config = create_test_config(None);
        assert_eq!(count.count(&mut config), cloned.count(&mut config));
    }

    #[test]
    fn test_count_debug() {
        let fixed = Count::Fixed(10);
        let range = Count::Range((5, 15));

        // Test that Debug is implemented (should not panic)
        let _ = format!("{:?}", fixed);
        let _ = format!("{:?}", range);
    }

    #[test]
    fn test_count_deserialize_fixed() {
        use serde_json;

        let json = "42";
        let count: Count = serde_json::from_str(json).unwrap();

        match count {
            Count::Fixed(n) => assert_eq!(n, 42),
            Count::Range(_) => panic!("Expected Fixed variant"),
        }
    }

    #[test]
    fn test_count_deserialize_range() {
        use serde_json;

        let json = "[5, 10]";
        let count: Count = serde_json::from_str(json).unwrap();

        match count {
            Count::Range((a, b)) => {
                assert_eq!(a, 5);
                assert_eq!(b, 10);
            },
            Count::Fixed(_) => panic!("Expected Range variant"),
        }
    }

    #[test]
    fn test_count_deserialize_invalid() {
        use serde_json;

        let invalid_json = "\"not_a_number\"";
        let result: Result<Count, _> = serde_json::from_str(invalid_json);

        assert!(result.is_err(), "Should fail to deserialize invalid input");
    }

    #[test]
    fn test_count_range_distribution() {
        let mut config = create_test_config(None);
        let count = Count::Range((1, 3));

        let mut results = std::collections::HashMap::new();

        // Generate many samples to check distribution
        for _ in 0..300 {
            let result = count.count(&mut config);
            *results.entry(result).or_insert(0) += 1;
        }

        // Should have generated all possible values
        assert!(results.contains_key(&1), "Should generate value 1");
        assert!(results.contains_key(&2), "Should generate value 2");
        assert!(results.contains_key(&3), "Should generate value 3");

        // Should not generate values outside range
        assert!(!results.contains_key(&0), "Should not generate value 0");
        assert!(!results.contains_key(&4), "Should not generate value 4");
    }

    #[test]
    fn test_multiple_configs_independence() {
        let count = Count::Range((1, 100));

        // Create configs with different seeds
        let mut config1 = create_test_config(None);
        config1.rng = StdRng::seed_from_u64(123);

        let mut config2 = create_test_config(None);
        config2.rng = StdRng::seed_from_u64(456);

        let result1 = count.count(&mut config1);
        let result2 = count.count(&mut config2);

        // With different seeds, results might be different (not guaranteed, but likely)
        // At minimum, both should be in valid range
        assert!((1..=100).contains(&result1));
        assert!((1..=100).contains(&result2));
    }

    #[test]
    fn test_edge_case_max_u64() {
        let mut config = create_test_config(None);
        let large_value = u64::MAX - 1;
        let count = Count::Fixed(large_value);

        assert_eq!(count.count(&mut config), large_value);
    }
}
