use rand::Rng;
use serde::Deserialize;
use serde_json::Value;

use crate::type_spec::JsonGenerator;

/// A specification for generating random numbers within a specified range.
///
/// `NumberSpec` defines constraints for number generation in JGD (JSON Generator Definition)
/// schemas, including the minimum and maximum values, and whether the generated numbers
/// should be integers or floating-point numbers.
///
/// This corresponds to the `Number` field type in the JGD schema specification.
///
/// # JGD Schema Representation
///
/// In a JGD schema, a number specification is represented as:
///
/// ```json
/// {
///   "number": {
///     "min": 0.0,
///     "max": 100.0,
///     "integer": true
///   }
/// }
/// ```
///
/// The `integer` field is optional and defaults to `false` if not specified.
///
/// # Examples
///
/// ```rust
/// use jgd_rs::{NumberSpec, JsonGenerator, GeneratorConfig};
/// use serde_json::Value;
///
/// // Create a spec for integers between 1 and 100
/// let int_spec = NumberSpec {
///     min: 1.0,
///     max: 100.0,
///     integer: true,
/// };
///
/// // Create a spec for floating-point numbers between 0.0 and 1.0
/// let float_spec = NumberSpec {
///     min: 0.0,
///     max: 1.0,
///     integer: false,
/// };
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct NumberSpec {
    /// The minimum value (inclusive) for generated numbers.
    ///
    /// This value defines the lower bound of the range for number generation.
    /// For integer generation, this will be cast to `i64` during generation.
    ///
    /// Maps to the `min` property in the JGD schema's number specification.
    pub min: f64,

    /// The maximum value (inclusive) for generated numbers.
    ///
    /// This value defines the upper bound of the range for number generation.
    /// For integer generation, this will be cast to `i64` during generation.
    ///
    /// Maps to the `max` property in the JGD schema's number specification.
    pub max: f64,

    /// Whether to generate integers instead of floating-point numbers.
    ///
    /// When `true`, the generated numbers will be integers within the range
    /// `[min as i64, max as i64]`. When `false` (default), floating-point
    /// numbers will be generated within the range `[min, max]`.
    ///
    /// Maps to the optional `integer` property in the JGD schema's number specification.
    /// Defaults to `false` when not specified in the schema.
    #[serde(default)]
    pub integer: bool
}

impl NumberSpec {
    /// Creates a new `NumberSpec` for generating floating-point numbers.
    ///
    /// This is a convenience constructor for creating number specifications that
    /// generate floating-point values, equivalent to the JGD schema:
    /// ```json
    /// { "number": { "min": <min>, "max": <max>, "integer": false } }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value (inclusive) for generated numbers
    /// * `max` - The maximum value (inclusive) for generated numbers
    ///
    /// # Returns
    ///
    /// A new `NumberSpec` configured to generate floating-point numbers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jgd_rs::NumberSpec;
    ///
    /// let spec = NumberSpec::new_float(0.0, 1.0);
    /// assert_eq!(spec.min, 0.0);
    /// assert_eq!(spec.max, 1.0);
    /// assert!(!spec.integer);
    /// ```
    pub fn new_float(min: f64, max: f64) -> Self {
        Self {
            min,
            max,
            integer: false,
        }
    }

    /// Creates a new `NumberSpec` for generating integer numbers.
    ///
    /// This is a convenience constructor for creating number specifications that
    /// generate integer values, equivalent to the JGD schema:
    /// ```json
    /// { "number": { "min": <min>, "max": <max>, "integer": true } }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value (inclusive) for generated numbers
    /// * `max` - The maximum value (inclusive) for generated numbers
    ///
    /// # Returns
    ///
    /// A new `NumberSpec` configured to generate integer numbers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jgd_rs::NumberSpec;
    ///
    /// let spec = NumberSpec::new_integer(1.0, 100.0);
    /// assert_eq!(spec.min, 1.0);
    /// assert_eq!(spec.max, 100.0);
    /// assert!(spec.integer);
    /// ```
    pub fn new_integer(min: f64, max: f64) -> Self {
        Self {
            min,
            max,
            integer: true,
        }
    }

    /// Validates that the number specification has a valid range.
    ///
    /// This is useful for validating JGD schema constraints before generation.
    /// A valid range requires that `min` is less than or equal to `max`.
    ///
    /// # Returns
    ///
    /// `true` if the range is valid for number generation, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jgd_rs::NumberSpec;
    ///
    /// let valid_spec = NumberSpec::new_float(0.0, 10.0);
    /// assert!(valid_spec.is_valid_range());
    ///
    /// let invalid_spec = NumberSpec::new_float(10.0, 0.0);
    /// assert!(!invalid_spec.is_valid_range());
    ///
    /// let single_value_spec = NumberSpec::new_integer(5.0, 5.0);
    /// assert!(single_value_spec.is_valid_range());
    /// ```
    pub fn is_valid_range(&self) -> bool {
        self.min <= self.max
    }

    /// Returns the size of the numeric range.
    ///
    /// This can be useful for understanding the distribution space of the
    /// number generation within the JGD schema context.
    ///
    /// # Returns
    ///
    /// The difference between `max` and `min`. Returns 0.0 if the range is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jgd_rs::NumberSpec;
    ///
    /// let spec = NumberSpec::new_float(0.0, 10.0);
    /// assert_eq!(spec.range_size(), 10.0);
    ///
    /// let single_value = NumberSpec::new_integer(5.0, 5.0);
    /// assert_eq!(single_value.range_size(), 0.0);
    /// ```
    pub fn range_size(&self) -> f64 {
        if self.is_valid_range() {
            self.max - self.min
        } else {
            0.0
        }
    }

    /// Returns the number of possible integer values in the range.
    ///
    /// This method is useful for understanding the cardinality of possible
    /// integer values when `integer` is `true` in the JGD schema specification.
    ///
    /// # Returns
    ///
    /// The number of possible integer values, or 0 if the range is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jgd_rs::NumberSpec;
    ///
    /// let spec = NumberSpec::new_integer(1.0, 5.0);
    /// assert_eq!(spec.integer_count(), 5); // 1, 2, 3, 4, 5
    ///
    /// let single = NumberSpec::new_integer(10.0, 10.0);
    /// assert_eq!(single.integer_count(), 1); // just 10
    /// ```
    pub fn integer_count(&self) -> u64 {
        if self.is_valid_range() {
            ((self.max as i64) - (self.min as i64) + 1).max(0) as u64
        } else {
            0
        }
    }
}

impl JsonGenerator for NumberSpec {
    /// Generates a random number according to the JGD number specification.
    ///
    /// This method produces either an integer or floating-point number within the
    /// specified range, depending on the `integer` field of the `NumberSpec`.
    /// The generated value will be serialized as a JSON number in the output.
    ///
    /// # Arguments
    ///
    /// * `config` - A mutable reference to the generator configuration containing
    ///   the random number generator and other generation context.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value::Number` containing either:
    /// - An integer value (when `integer` is `true`) within `[min as i64, max as i64]`
    /// - A floating-point value (when `integer` is `false`) within `[min, max]`
    ///
    /// # Schema Compliance
    ///
    /// This implementation ensures the generated values conform to the JGD schema's
    /// number specification requirements:
    /// - Respects the `min` and `max` bounds (inclusive)
    /// - Generates integers when `integer` is `true`
    /// - Generates floating-point numbers when `integer` is `false`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jgd_rs::{NumberSpec, JsonGenerator, GeneratorConfig};
    /// use serde_json::Value;
    ///
    /// let mut config = GeneratorConfig::new("EN", Some(42));
    ///
    /// // Generate integers between 1 and 10 (as per JGD schema)
    /// let int_spec = NumberSpec {
    ///     min: 1.0,
    ///     max: 10.0,
    ///     integer: true,
    /// };
    /// let value = int_spec.generate(&mut config);
    /// if let Value::Number(n) = value {
    ///     assert!(n.is_i64());
    ///     let int_val = n.as_i64().unwrap();
    ///     assert!((1..=10).contains(&int_val));
    /// }
    ///
    /// // Generate floats between 0.0 and 1.0 (as per JGD schema)
    /// let float_spec = NumberSpec {
    ///     min: 0.0,
    ///     max: 1.0,
    ///     integer: false,
    /// };
    /// let value = float_spec.generate(&mut config);
    /// if let Value::Number(n) = value {
    ///     let float_val = n.as_f64().unwrap();
    ///     assert!((0.0..=1.0).contains(&float_val));
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This method may panic if:
    /// - The random number generator fails to generate a value in the specified range
    /// - Integer conversion fails when `integer` is `true` and the range exceeds `i64` bounds
    ///
    /// # Performance Notes
    ///
    /// - Integer generation is slightly faster than floating-point generation
    /// - Range size does not significantly impact generation performance
    fn generate(&self, config: &mut super::GeneratorConfig) -> Value {
        if self.integer {
            Value::from(config.rng.random_range(self.min as i64 ..= self.max as i64))
        } else {
            Value::from(config.rng.random_range(self.min..=self.max))
        }
    }
}
