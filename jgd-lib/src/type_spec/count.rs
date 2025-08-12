use rand::Rng;
use serde::Deserialize;

use crate::type_spec::GeneratorConfig;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Count {
    Fixed(u64),
    Range((u64,u64))
}

pub trait GetCount {
    fn count(&self, config: &mut GeneratorConfig) -> u64;
}

impl GetCount for Count {
    fn count(&self, config: &mut GeneratorConfig) -> u64 {
        match self {
            Count::Fixed(n) => *n,
            Count::Range((a, b)) => config.rng.random_range(*a..=*b),
        }
    }
}

impl GetCount for Option<Count> {
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
