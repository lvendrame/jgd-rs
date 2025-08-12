use rand::Rng;
use serde::Deserialize;

use crate::type_spec::GeneratorConfig;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Len {
    Fixed(u64),
    Range((u64,u64))
}

pub trait GetLength {
    fn len(&self, config: &mut GeneratorConfig) -> u64;
}

impl GetLength for Len {
    fn len(&self, config: &mut GeneratorConfig) -> u64 {
        match self {
            Len::Fixed(n) => *n,
            Len::Range((a, b)) => config.rng.random_range(*a..=*b),
        }
    }
}

impl GetLength for Option<Len> {
    fn len(&self, config: &mut GeneratorConfig) -> u64 {
        self.clone().unwrap_or(Len::Fixed(1)).len(config)
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
    fn test_len_fixed() {
        let mut config = create_test_config(None);
        let len = Len::Fixed(5);

        assert_eq!(len.len(&mut config), 5);
        // Should always return the same value
        assert_eq!(len.len(&mut config), 5);
        assert_eq!(len.len(&mut config), 5);
    }

    #[test]
    fn test_len_fixed_zero() {
        let mut config = create_test_config(None);
        let len = Len::Fixed(0);

        assert_eq!(len.len(&mut config), 0);
    }

    #[test]
    fn test_len_fixed_large_number() {
        let mut config = create_test_config(None);
        let len = Len::Fixed(1000);

        assert_eq!(len.len(&mut config), 1000);
    }

    #[test]
    fn test_len_range_single_value() {
        let mut config = create_test_config(None);
        let len = Len::Range((5, 5));

        // Range with same min and max should always return that value
        assert_eq!(len.len(&mut config), 5);
        assert_eq!(len.len(&mut config), 5);
    }

    #[test]
    fn test_len_range_within_bounds() {
        let mut config = create_test_config(None);
        let len = Len::Range((1, 10));

        // Test multiple times to ensure values are within range
        for _ in 0..20 {
            let result = len.len(&mut config);
            assert!((1..=10).contains(&result), "Value {} not in range [1, 10]", result);
        }
    }

    #[test]
    fn test_len_range_zero_to_value() {
        let mut config = create_test_config(None);
        let len = Len::Range((0, 3));

        for _ in 0..20 {
            let result = len.len(&mut config);
            assert!(result <= 3, "Value {} exceeds maximum 3", result);
        }
    }

    #[test]
    fn test_len_range_large_range() {
        let mut config = create_test_config(None);
        let len = Len::Range((100, 200));

        for _ in 0..10 {
            let result = len.len(&mut config);
            assert!((100..=200).contains(&result), "Value {} not in range [100, 200]", result);
        }
    }

    #[test]
    fn test_len_range_deterministic_with_seed() {
        // Test that the same seed produces the same sequence
        let len = Len::Range((1, 100));

        let mut config1 = create_test_config(Some(21));
        let mut config2 = create_test_config(Some(21));

        // Both configs use the same seed, so should produce same sequence
        let result1 = len.len(&mut config1);
        let result2 = len.len(&mut config2);

        assert_eq!(result1, result2, "Same seed should produce same results");
    }

    #[test]
    fn test_option_len_some_fixed() {
        let mut config = create_test_config(None);
        let opt_len = Some(Len::Fixed(7));

        assert_eq!(opt_len.len(&mut config), 7);
    }

    #[test]
    fn test_option_len_some_range() {
        let mut config = create_test_config(None);
        let opt_len = Some(Len::Range((2, 8)));

        for _ in 0..10 {
            let result = opt_len.len(&mut config);
            assert!((2..=8).contains(&result), "Value {} not in range [2, 8]", result);
        }
    }

    #[test]
    fn test_option_len_none() {
        let mut config = create_test_config(None);
        let opt_len: Option<Len> = None;

        // None should default to Len::Fixed(1)
        assert_eq!(opt_len.len(&mut config), 1);
        assert_eq!(opt_len.len(&mut config), 1);
    }

    #[test]
    fn test_len_clone() {
        let len = Len::Fixed(42);
        let cloned = len.clone();

        let mut config = create_test_config(None);
        assert_eq!(len.len(&mut config), cloned.len(&mut config));
    }

    #[test]
    fn test_len_debug() {
        let fixed = Len::Fixed(10);
        let range = Len::Range((5, 15));

        // Test that Debug is implemented (should not panic)
        let _ = format!("{:?}", fixed);
        let _ = format!("{:?}", range);
    }

    #[test]
    fn test_len_deserialize_fixed() {
        use serde_json;

        let json = "42";
        let len: Len = serde_json::from_str(json).unwrap();

        match len {
            Len::Fixed(n) => assert_eq!(n, 42),
            Len::Range(_) => panic!("Expected Fixed variant"),
        }
    }

    #[test]
    fn test_len_deserialize_range() {
        use serde_json;

        let json = "[5, 10]";
        let len: Len = serde_json::from_str(json).unwrap();

        match len {
            Len::Range((a, b)) => {
                assert_eq!(a, 5);
                assert_eq!(b, 10);
            },
            Len::Fixed(_) => panic!("Expected Range variant"),
        }
    }

    #[test]
    fn test_len_deserialize_invalid() {
        use serde_json;

        let invalid_json = "\"not_a_number\"";
        let result: Result<Len, _> = serde_json::from_str(invalid_json);

        assert!(result.is_err(), "Should fail to deserialize invalid input");
    }

    #[test]
    fn test_len_range_distribution() {
        let mut config = create_test_config(None);
        let len = Len::Range((1, 3));

        let mut results = std::collections::HashMap::new();

        // Generate many samples to check distribution
        for _ in 0..300 {
            let result = len.len(&mut config);
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
        let len = Len::Range((1, 100));

        // Create configs with different seeds
        let mut config1 = create_test_config(None);
        config1.rng = StdRng::seed_from_u64(123);

        let mut config2 = create_test_config(None);
        config2.rng = StdRng::seed_from_u64(456);

        let result1 = len.len(&mut config1);
        let result2 = len.len(&mut config2);

        // With different seeds, results might be different (not guaranteed, but likely)
        // At minimum, both should be in valid range
        assert!((1..=100).contains(&result1));
        assert!((1..=100).contains(&result2));
    }

    #[test]
    fn test_edge_case_max_u64() {
        let mut config = create_test_config(None);
        let large_value = u64::MAX - 1;
        let len = Len::Fixed(large_value);

        assert_eq!(len.len(&mut config), large_value);
    }
}
