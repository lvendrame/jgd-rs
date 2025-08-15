use chrono::{DateTime, NaiveDateTime, Utc};

/// Represents parsed arguments from faker pattern parameters.
///
/// Arguments are typically extracted from parentheses in faker patterns like:
/// - `faker.name.first_name()` → `Arguments::None`
/// - `faker.number.number(100)` → `Arguments::Fixed("100")`
/// - `faker.number.between(1,10)` → `Arguments::Range("1", "10")`
/// - `faker.date.between(2020-01-01..2024-12-31)` → `Arguments::Range("2020-01-01", "2024-12-31")`
///
/// # Examples
///
/// ```rust
/// use jgd_rs::Arguments;
///
/// // Parse single value
/// let args = Arguments::from("(42)");
/// assert_eq!(args.get_number(0), 42);
///
/// // Parse range with comma
/// let args = Arguments::from("(1,10)");
/// let range = args.get_number_range(0, 100);
/// assert_eq!(range.start, 1);
/// assert_eq!(range.end, 10);
///
/// // Parse range with dots
/// let args = Arguments::from("(1..10)");
/// let (start, end) = args.get_string_tuple("0", "100");
/// assert_eq!(start, "1");
/// assert_eq!(end, "10");
/// ```
pub enum Arguments {
    /// No arguments provided (empty parentheses or no parentheses)
    None,
    /// Single fixed argument value
    Fixed(String),
    /// Range with start and end values (separated by comma or dots)
    Range(String, String),
}

impl From<&str> for Arguments {
    /// Parses a string into Arguments enum.
    ///
    /// Expects input in the format `(content)` where content can be:
    /// - Empty: `()` → `Arguments::None`
    /// - Single value: `(42)` → `Arguments::Fixed("42")`
    /// - Comma-separated: `(1,10)` → `Arguments::Range("1", "10")`
    /// - Dot-separated: `(1..10)` → `Arguments::Range("1", "10")`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jgd_rs::Arguments;
    ///
    /// let args = Arguments::from("(42)");
    /// assert!(matches!(args, Arguments::Fixed(ref s) if s == "42"));
    ///
    /// let args = Arguments::from("(1,10)");
    /// assert!(matches!(args, Arguments::Range(ref s1, ref s2) if s1 == "1" && s2 == "10"));
    ///
    /// let args = Arguments::from("(1..10)");
    /// assert!(matches!(args, Arguments::Range(ref s1, ref s2) if s1 == "1" && s2 == "10"));
    /// ```
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = if let Some(args_content) = value.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
            if args_content.contains("..") {
                args_content.split("..").map(|s| s.trim()).collect()
            } else if args_content.contains(',') {
                args_content.split(',').map(|s| s.trim()).collect()
            } else {
                vec![args_content.trim()]
            }
        } else {
            return Arguments::None;
        };

        if parts.len() > 1 {
            if parts[0].is_empty() && parts[1].is_empty() {
                return Arguments::None;
            }

            if parts[1].is_empty() {
                return Arguments::Fixed(parts[0].into());
            }

            return Arguments::Range(parts[0].into(), parts[1].into());
        }

        if !parts[0].is_empty() {
            return Arguments::Fixed(parts[0].into());
        }

        Arguments::None
    }
}

impl Arguments {

    /// Helper function to parse a single numeric argument.
    ///
    /// Attempts to parse the given string as type T. If parsing fails,
    /// returns the provided default value.
    ///
    /// # Arguments
    /// * `arg` - The string to parse
    /// * `default_value` - Value to return if parsing fails
    ///
    /// # Examples
    ///
    /// These are private helper methods used internally by the public getter methods.
    /// They demonstrate the parsing behavior for different types of arguments.
    ///
    /// ```text
    /// Arguments::parse_number("42", 0i32) -> 42
    /// Arguments::parse_number("invalid", 0i32) -> 0 (fallback to default)
    /// ```
    fn parse_number<T: std::str::FromStr>(arg: &str, default_value: T) -> T {
        if let Ok(value) = arg.parse::<T>() {
            return value;
        }

        default_value
    }

    /// Helper function to parse a time argument.
    ///
    /// Attempts to parse the given string as a time value. Currently supports
    /// Unix timestamp parsing. If parsing fails, returns the provided default value.
    ///
    /// # Arguments
    /// * `arg` - The string to parse (e.g., Unix timestamp)
    /// * `default_value` - Value to return if parsing fails
    ///
    /// # Examples
    ///
    /// This is a private helper method used internally for time parsing.
    ///
    /// ```text
    /// Arguments::parse_time("1640995200", default) -> OffsetDateTime from Unix timestamp
    /// Arguments::parse_time("invalid", default) -> default (fallback)
    /// ```
    fn parse_time(arg: &str, default_value: time::OffsetDateTime) -> time::OffsetDateTime {
        // Try parsing as RFC 3339 format (most common for APIs)
        // For now, we'll use a simple fallback since time parsing is complex
        // This could be enhanced with proper format descriptors later

        // Simple fallback: try to parse as Unix timestamp
        if let Ok(timestamp) = arg.parse::<i64>() {
            if let Ok(datetime) = time::OffsetDateTime::from_unix_timestamp(timestamp) {
                return datetime;
            }
        }

        default_value
    }

    /// Helper function to parse a datetime argument.
    ///
    /// Attempts to parse the given string as a DateTime<Utc> using multiple formats:
    /// - Direct ISO 8601 UTC parse
    /// - RFC3339 (handles Z, offsets, fractional seconds)
    /// - Common patterns with timezone
    /// - Naive formats (assumes UTC)
    ///
    /// # Arguments
    /// * `arg` - The string to parse (e.g., "2024-01-01T00:00:00Z", "2024-01-01 00:00:00")
    /// * `default_value` - Value to return if parsing fails
    ///
    /// # Examples
    ///
    /// This is a private helper method used internally for datetime parsing.
    ///
    /// ```text
    /// Arguments::parse_datetime("2024-01-01T00:00:00Z", default) -> DateTime<Utc> parsed
    /// Arguments::parse_datetime("2024-01-01 00:00:00", default) -> DateTime<Utc> parsed as naive + UTC
    /// Arguments::parse_datetime("invalid", default) -> default (fallback)
    /// ```
    fn parse_datetime(arg: &str, default_value: DateTime<Utc>) -> DateTime<Utc> {
        if let Ok(dt) = arg.parse::<DateTime<Utc>>() {
            return dt;
        }

        // 1. Direct ISO 8601 UTC parse
        if let Ok(dt) = arg.parse::<DateTime<Utc>>() {
            return dt;
        }

        // 2. RFC3339 (handles Z, offsets, fractional seconds)
        if let Ok(dt) = DateTime::parse_from_rfc3339(arg) {
            return dt.with_timezone(&Utc);
        }

        // 3. Try common patterns with timezone
        let tz_formats = [
            "%Y-%m-%dT%H:%M:%S%:z",
            "%Y-%m-%dT%H:%M:%SZ",
            "%Y-%m-%dT%H:%M:%S%.3f%:z",
            "%Y-%m-%dT%H:%M:%S%.3fZ",
        ];
        for fmt in tz_formats {
            if let Ok(dt) = DateTime::parse_from_str(arg, fmt) {
                return dt.with_timezone(&Utc);
            }
        }

        // 4. Try naive formats and assume UTC
        let naive_formats = [
            "%Y-%m-%d %H:%M:%S",
            "%Y-%m-%dT%H:%M:%S",
            "%Y-%m-%d %H:%M:%S%.3f",
            "%Y-%m-%dT%H:%M:%S%.3f",
        ];
        for fmt in naive_formats {
            if let Ok(ndt) = NaiveDateTime::parse_from_str(arg, fmt) {
                return ndt.and_utc();
            }
        }

        default_value
    }

    /// Extracts a string value from the arguments.
    ///
    /// Returns the first argument for Fixed and Range variants,
    /// or the default value for None variant.
    ///
    /// # Arguments
    /// * `default_value` - Value to return if no arguments are present
    ///
    /// # Returns
    /// - `Arguments::None` → `default_value`
    /// - `Arguments::Fixed(arg)` → `arg`
    /// - `Arguments::Range(arg1, arg2)` → `arg1`
    ///
    /// # Examples
    /// ```rust
    /// use jgd_rs::Arguments;
    ///
    /// let args = Arguments::from("(hello)");
    /// assert_eq!(args.get_string("default"), "hello");
    ///
    /// let args = Arguments::from("(start,end)");
    /// assert_eq!(args.get_string("default"), "start");
    ///
    /// let args = Arguments::None;
    /// assert_eq!(args.get_string("default"), "default");
    /// ```
    pub fn get_string<'a>(&'a self, default_value: &'a str) -> &'a str {
        match self {
            Arguments::None => default_value,
            Arguments::Fixed(arg) => arg,
            Arguments::Range(arg, _) => arg,
        }
    }

    /// Extracts a tuple of string values from the arguments.
    ///
    /// Returns both start and end values for different argument types.
    ///
    /// # Arguments
    /// * `default_start` - Value to return as start if no arguments are present
    /// * `default_end` - Value to return as end if no arguments are present or only one argument
    ///
    /// # Returns
    /// - `Arguments::None` → `(default_start, default_end)`
    /// - `Arguments::Fixed(arg)` → `(arg, default_end)`
    /// - `Arguments::Range(arg1, arg2)` → `(arg1, arg2)`
    ///
    /// # Examples
    /// ```rust
    /// use jgd_rs::Arguments;
    ///
    /// let args = Arguments::from("(1,10)");
    /// let (start, end) = args.get_string_tuple("0", "100");
    /// assert_eq!(start, "1");
    /// assert_eq!(end, "10");
    ///
    /// let args = Arguments::from("(5)");
    /// let (start, end) = args.get_string_tuple("0", "100");
    /// assert_eq!(start, "5");
    /// assert_eq!(end, "100");
    /// ```
    pub fn get_string_tuple<'a>(&'a self, default_start: &'a str, default_end: &'a str) -> (&'a str, &'a str) {
        match self {
            Arguments::None => (default_start, default_end),
            Arguments::Fixed(arg) => (arg, default_end),
            Arguments::Range(arg1, arg2) => (arg1, arg2),
        }
    }

    /// Extracts a numeric value from the arguments.
    ///
    /// Attempts to parse the first argument as type T. If parsing fails
    /// or no arguments are present, returns the default value.
    ///
    /// # Arguments
    /// * `default_value` - Value to return if no arguments are present or parsing fails
    ///
    /// # Returns
    /// - `Arguments::None` → `default_value`
    /// - `Arguments::Fixed(arg)` → parsed `arg` or `default_value` if parsing fails
    /// - `Arguments::Range(arg1, arg2)` → parsed `arg1` or `default_value` if parsing fails
    ///
    /// # Examples
    /// ```rust
    /// use jgd_rs::Arguments;
    ///
    /// let args = Arguments::from("(42)");
    /// assert_eq!(args.get_number(0), 42);
    ///
    /// let args = Arguments::from("(invalid)");
    /// assert_eq!(args.get_number(100), 100);
    ///
    /// let args = Arguments::from("(1,10)");
    /// assert_eq!(args.get_number(0), 1);
    /// ```
    pub fn get_number<T: std::str::FromStr>(&self, default_value: T) -> T {
        match self {
            Arguments::None => default_value,
            Arguments::Fixed(arg) => Self::parse_number(arg, default_value),
            Arguments::Range(arg, _) => Self::parse_number(arg, default_value),
        }
    }

    /// Extracts a numeric range from the arguments.
    ///
    /// Creates a `Range<T>` from the arguments, parsing both start and end values.
    /// If parsing fails, uses the corresponding default value.
    ///
    /// # Arguments
    /// * `default_start` - Value to use as range start if no arguments or parsing fails
    /// * `default_end` - Value to use as range end if no second argument or parsing fails
    ///
    /// # Returns
    /// - `Arguments::None` → `default_start..default_end`
    /// - `Arguments::Fixed(arg)` → `parsed_arg..default_end`
    /// - `Arguments::Range(arg1, arg2)` → `parsed_arg1..parsed_arg2`
    ///
    /// # Examples
    /// ```rust
    /// use jgd_rs::Arguments;
    ///
    /// let args = Arguments::from("(1,10)");
    /// let range = args.get_number_range(0, 100);
    /// assert_eq!(range.start, 1);
    /// assert_eq!(range.end, 10);
    ///
    /// let args = Arguments::from("(5)");
    /// let range = args.get_number_range(0, 100);
    /// assert_eq!(range.start, 5);
    /// assert_eq!(range.end, 100);
    /// ```
    pub fn get_number_range<T: std::str::FromStr>(
        &self, default_start: T, default_end: T
    ) -> std::ops::Range<T> {
        match self {
            Arguments::None => default_start..default_end,
            Arguments::Fixed(arg) => Self::parse_number(arg, default_start)..default_end,
            Arguments::Range(arg1, arg2) =>
                Self::parse_number(arg1, default_start)..Self::parse_number(arg2, default_end),
        }
    }

    /// Extracts a time value from the arguments.
    ///
    /// Attempts to parse the first argument as a time value. If parsing fails
    /// or no arguments are present, returns the default value.
    ///
    /// # Arguments
    /// * `default_value` - Value to return if no arguments are present or parsing fails
    ///
    /// # Returns
    /// - `Arguments::None` → `default_value`
    /// - `Arguments::Fixed(arg)` → parsed `arg` or `default_value` if parsing fails
    /// - `Arguments::Range(arg1, arg2)` → parsed `arg1` or `default_value` if parsing fails
    ///
    /// # Examples
    /// ```rust
    /// use jgd_rs::Arguments;
    /// use time::OffsetDateTime;
    ///
    /// let default = OffsetDateTime::now_utc();
    /// let args = Arguments::from("(1640995200)"); // Unix timestamp
    /// let result = args.get_time(default);
    /// ```
    pub fn get_time(&self, default_value: time::OffsetDateTime) -> time::OffsetDateTime {
        match self {
            Arguments::None => default_value,
            Arguments::Fixed(arg) => Self::parse_time(arg, default_value),
            Arguments::Range(arg, _) => Self::parse_time(arg, default_value),
        }
    }

    /// Extracts a time range from the arguments.
    ///
    /// Creates a tuple of time values from the arguments, parsing both start and end values.
    /// If parsing fails, uses the corresponding default value.
    ///
    /// # Arguments
    /// * `default_start` - Value to use as start time if no arguments or parsing fails
    /// * `default_end` - Value to use as end time if no second argument or parsing fails
    ///
    /// # Returns
    /// - `Arguments::None` → `(default_start, default_end)`
    /// - `Arguments::Fixed(arg)` → `(parsed_arg, default_end)`
    /// - `Arguments::Range(arg1, arg2)` → `(parsed_arg1, parsed_arg2)`
    ///
    /// # Examples
    /// ```rust
    /// use jgd_rs::Arguments;
    /// use time::OffsetDateTime;
    ///
    /// let default_start = OffsetDateTime::now_utc();
    /// let default_end = OffsetDateTime::now_utc();
    /// let args = Arguments::from("(1640995200,1672531200)"); // Unix timestamps
    /// let (start, end) = args.get_time_range(default_start, default_end);
    /// ```
    pub fn get_time_range(
        &self, default_start: time::OffsetDateTime, default_end: time::OffsetDateTime
    ) -> (time::OffsetDateTime, time::OffsetDateTime) {
        match self {
            Arguments::None => (default_start, default_end),
            Arguments::Fixed(arg) => (Self::parse_time(arg, default_start), default_end),
            Arguments::Range(arg1, arg2) =>
                (Self::parse_time(arg1, default_start), Self::parse_time(arg2, default_end)),
        }
    }

    /// Extracts a datetime value from the arguments.
    ///
    /// Attempts to parse the first argument as a `DateTime<Utc>` using multiple formats.
    /// If parsing fails or no arguments are present, returns the default value.
    ///
    /// # Arguments
    /// * `default_value` - Value to return if no arguments are present or parsing fails
    ///
    /// # Returns
    /// - `Arguments::None` → `default_value`
    /// - `Arguments::Fixed(arg)` → parsed `arg` or `default_value` if parsing fails
    /// - `Arguments::Range(arg1, arg2)` → parsed `arg1` or `default_value` if parsing fails
    ///
    /// # Examples
    /// ```rust
    /// use jgd_rs::Arguments;
    /// use chrono::{DateTime, Utc};
    ///
    /// let default = DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z").unwrap().with_timezone(&Utc);
    /// let args = Arguments::from("(2024-01-01T00:00:00Z)");
    /// let result = args.get_datetime(default);
    /// ```
    pub fn get_datetime(&self, default_value: DateTime<Utc>) -> DateTime<Utc> {
        match self {
            Arguments::None => default_value,
            Arguments::Fixed(arg) => Self::parse_datetime(arg, default_value),
            Arguments::Range(arg, _) => Self::parse_datetime(arg, default_value),
        }
    }

    /// Extracts a datetime range from the arguments.
    ///
    /// Creates a tuple of `DateTime<Utc>` values from the arguments, parsing both start and end values.
    /// If parsing fails, uses the corresponding default value.
    ///
    /// # Arguments
    /// * `default_start` - Value to use as start datetime if no arguments or parsing fails
    /// * `default_end` - Value to use as end datetime if no second argument or parsing fails
    ///
    /// # Returns
    /// - `Arguments::None` → `(default_start, default_end)`
    /// - `Arguments::Fixed(arg)` → `(parsed_arg, default_end)`
    /// - `Arguments::Range(arg1, arg2)` → `(parsed_arg1, parsed_arg2)`
    ///
    /// # Examples
    /// ```rust
    /// use jgd_rs::Arguments;
    /// use chrono::{DateTime, Utc};
    ///
    /// let default_start = DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z").unwrap().with_timezone(&Utc);
    /// let default_end = DateTime::parse_from_rfc3339("2024-12-31T23:59:59Z").unwrap().with_timezone(&Utc);
    /// let args = Arguments::from("(2024-01-01T00:00:00Z,2024-12-31T23:59:59Z)");
    /// let (start, end) = args.get_datetime_range(default_start, default_end);
    /// ```
    pub fn get_datetime_range(
        &self, default_start: DateTime<Utc>, default_end: DateTime<Utc>
    ) -> (DateTime<Utc>, DateTime<Utc>) {
        match self {
            Arguments::None => (default_start, default_end),
            Arguments::Fixed(arg) => (Self::parse_datetime(arg, default_start), default_end),
            Arguments::Range(arg1, arg2) =>
                (Self::parse_datetime(arg1, default_start), Self::parse_datetime(arg2, default_end)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc, NaiveDate};

    #[test]
    fn test_parse_simple_string() {
        let args = Arguments::from("lorem");
        assert!(matches!(args, Arguments::None));
    }

    #[test]
    fn test_parse_empty_parentheses() {
        let args = Arguments::from("()");
        assert!(matches!(args, Arguments::None));
    }

    #[test]
    fn test_parse_single_number() {
        let args = Arguments::from("(1)");
        if let Arguments::Fixed(value) = args {
            assert_eq!(value, "1");
        } else {
            panic!("Expected Fixed argument");
        }
    }

    #[test]
    fn test_parse_comma_separated_numbers() {
        let args = Arguments::from("(1,2)");
        if let Arguments::Range(start, end) = args {
            assert_eq!(start, "1");
            assert_eq!(end, "2");
        } else {
            panic!("Expected Range argument");
        }
    }

    #[test]
    fn test_parse_comma_separated_with_spaces() {
        let args = Arguments::from("(1, 2)");
        if let Arguments::Range(start, end) = args {
            assert_eq!(start, "1");
            assert_eq!(end, "2");
        } else {
            panic!("Expected Range argument");
        }
    }

    #[test]
    fn test_parse_large_number() {
        let args = Arguments::from("(123)");
        if let Arguments::Fixed(value) = args {
            assert_eq!(value, "123");
        } else {
            panic!("Expected Fixed argument");
        }
    }

    #[test]
    fn test_parse_multiple_comma_separated() {
        let args = Arguments::from("(123,456,789)");
        if let Arguments::Range(start, end) = args {
            assert_eq!(start, "123");
            assert_eq!(end, "456");
        } else {
            panic!("Expected Range argument");
        }
    }

    #[test]
    fn test_parse_range_with_dots() {
        let args = Arguments::from("(1..2)");
        if let Arguments::Range(start, end) = args {
            assert_eq!(start, "1");
            assert_eq!(end, "2");
        } else {
            panic!("Expected Range argument");
        }
    }

    #[test]
    fn test_parse_range_with_dots_and_spaces() {
        let args = Arguments::from("(1.. 2)");
        if let Arguments::Range(start, end) = args {
            assert_eq!(start, "1");
            assert_eq!(end, "2");
        } else {
            panic!("Expected Range argument");
        }
    }

    #[test]
    fn test_parse_range_with_multiple_dots() {
        let args = Arguments::from("(1..2..3)");
        if let Arguments::Range(start, end) = args {
            assert_eq!(start, "1");
            assert_eq!(end, "2");
        } else {
            panic!("Expected Range argument");
        }
    }

    #[test]
    fn test_parse_percentage() {
        let args = Arguments::from("(75)");
        if let Arguments::Fixed(value) = args {
            assert_eq!(value, "75");
        } else {
            panic!("Expected Fixed argument");
        }
    }

    #[test]
    fn test_parse_datetime_with_space() {
        let args = Arguments::from("(2024-01-01 00:00:00)");
        if let Arguments::Fixed(value) = args {
            assert_eq!(value, "2024-01-01 00:00:00");
        } else {
            panic!("Expected Fixed argument");
        }
    }

    #[test]
    fn test_parse_datetime_iso() {
        let args = Arguments::from("(2024-12-31T23:59:59)");
        if let Arguments::Fixed(value) = args {
            assert_eq!(value, "2024-12-31T23:59:59");
        } else {
            panic!("Expected Fixed argument");
        }
    }

    #[test]
    fn test_parse_datetime_range_comma() {
        let args = Arguments::from("(2024-01-01 00:00:00, 2024-12-31T23:59:59)");
        if let Arguments::Range(start, end) = args {
            assert_eq!(start, "2024-01-01 00:00:00");
            assert_eq!(end, "2024-12-31T23:59:59");
        } else {
            panic!("Expected Range argument");
        }
    }

    #[test]
    fn test_parse_datetime_range_dots_with_spaces() {
        let args = Arguments::from("(2024-01-01 00:00:00.. 2024-12-31T23:59:59)");
        if let Arguments::Range(start, end) = args {
            assert_eq!(start, "2024-01-01 00:00:00");
            assert_eq!(end, "2024-12-31T23:59:59");
        } else {
            panic!("Expected Range argument");
        }
    }

    #[test]
    fn test_parse_datetime_range_comma_no_spaces() {
        let args = Arguments::from("(2024-01-01 00:00:00,2024-12-31T23:59:59)");
        if let Arguments::Range(start, end) = args {
            assert_eq!(start, "2024-01-01 00:00:00");
            assert_eq!(end, "2024-12-31T23:59:59");
        } else {
            panic!("Expected Range argument");
        }
    }

    #[test]
    fn test_parse_datetime_range_dots_no_spaces() {
        let args = Arguments::from("(2024-01-01 00:00:00..2024-12-31T23:59:59)");
        if let Arguments::Range(start, end) = args {
            assert_eq!(start, "2024-01-01 00:00:00");
            assert_eq!(end, "2024-12-31T23:59:59");
        } else {
            panic!("Expected Range argument");
        }
    }

    #[test]
    fn test_parse_time_start() {
        let args = Arguments::from("(00:00:00)");
        if let Arguments::Fixed(value) = args {
            assert_eq!(value, "00:00:00");
        } else {
            panic!("Expected Fixed argument");
        }
    }

    #[test]
    fn test_parse_time_end() {
        let args = Arguments::from("(23:59:59)");
        if let Arguments::Fixed(value) = args {
            assert_eq!(value, "23:59:59");
        } else {
            panic!("Expected Fixed argument");
        }
    }

    #[test]
    fn test_parse_time_range_comma() {
        let args = Arguments::from("(00:00:00, 23:59:59)");
        if let Arguments::Range(start, end) = args {
            assert_eq!(start, "00:00:00");
            assert_eq!(end, "23:59:59");
        } else {
            panic!("Expected Range argument");
        }
    }

    #[test]
    fn test_parse_time_range_dots_with_spaces() {
        let args = Arguments::from("(00:00:00.. 23:59:59)");
        if let Arguments::Range(start, end) = args {
            assert_eq!(start, "00:00:00");
            assert_eq!(end, "23:59:59");
        } else {
            panic!("Expected Range argument");
        }
    }

    #[test]
    fn test_parse_time_range_comma_no_spaces() {
        let args = Arguments::from("(00:00:00,23:59:59)");
        if let Arguments::Range(start, end) = args {
            assert_eq!(start, "00:00:00");
            assert_eq!(end, "23:59:59");
        } else {
            panic!("Expected Range argument");
        }
    }

    #[test]
    fn test_parse_time_range_dots_no_spaces() {
        let args = Arguments::from("(00:00:00..23:59:59)");
        if let Arguments::Range(start, end) = args {
            assert_eq!(start, "00:00:00");
            assert_eq!(end, "23:59:59");
        } else {
            panic!("Expected Range argument");
        }
    }

    #[test]
    fn test_parse_format_string() {
        let args = Arguments::from("((###) ###-####)");
        if let Arguments::Fixed(value) = args {
            assert_eq!(value, "(###) ###-####");
        } else {
            panic!("Expected Fixed argument");
        }
    }

    // Test getter methods
    #[test]
    fn test_get_string_default() {
        let args = Arguments::None;
        assert_eq!(args.get_string("default"), "default");
    }

    #[test]
    fn test_get_string_fixed() {
        let args = Arguments::Fixed("test".to_string());
        assert_eq!(args.get_string("default"), "test");
    }

    #[test]
    fn test_get_string_range() {
        let args = Arguments::Range("start".to_string(), "end".to_string());
        assert_eq!(args.get_string("default"), "start");
    }

    #[test]
    fn test_get_string_tuple_none() {
        let args = Arguments::None;
        let (start, end) = args.get_string_tuple("def_start", "def_end");
        assert_eq!(start, "def_start");
        assert_eq!(end, "def_end");
    }

    #[test]
    fn test_get_string_tuple_fixed() {
        let args = Arguments::Fixed("value".to_string());
        let (start, end) = args.get_string_tuple("def_start", "def_end");
        assert_eq!(start, "value");
        assert_eq!(end, "def_end");
    }

    #[test]
    fn test_get_string_tuple_range() {
        let args = Arguments::Range("start".to_string(), "end".to_string());
        let (start, end) = args.get_string_tuple("def_start", "def_end");
        assert_eq!(start, "start");
        assert_eq!(end, "end");
    }

    #[test]
    fn test_get_number_default() {
        let args = Arguments::None;
        assert_eq!(args.get_number(42), 42);
    }

    #[test]
    fn test_get_number_fixed() {
        let args = Arguments::Fixed("123".to_string());
        assert_eq!(args.get_number(42), 123);
    }

    #[test]
    fn test_get_number_fixed_invalid() {
        let args = Arguments::Fixed("invalid".to_string());
        assert_eq!(args.get_number(42), 42);
    }

    #[test]
    fn test_get_number_range() {
        let args = Arguments::Range("100".to_string(), "200".to_string());
        assert_eq!(args.get_number(42), 100);
    }

    #[test]
    fn test_get_number_range_invalid() {
        let args = Arguments::Range("invalid".to_string(), "200".to_string());
        assert_eq!(args.get_number(42), 42);
    }

    #[test]
    fn test_get_number_range_function() {
        let args = Arguments::None;
        let range = args.get_number_range(1, 10);
        assert_eq!(range.start, 1);
        assert_eq!(range.end, 10);
    }

    #[test]
    fn test_get_number_range_function_fixed() {
        let args = Arguments::Fixed("5".to_string());
        let range = args.get_number_range(1, 10);
        assert_eq!(range.start, 5);
        assert_eq!(range.end, 10);
    }

    #[test]
    fn test_get_number_range_function_range() {
        let args = Arguments::Range("3".to_string(), "7".to_string());
        let range = args.get_number_range(1, 10);
        assert_eq!(range.start, 3);
        assert_eq!(range.end, 7);
    }

    #[test]
    fn test_get_datetime_default() {
        let default_dt = DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z").unwrap().with_timezone(&Utc);
        let args = Arguments::None;
        assert_eq!(args.get_datetime(default_dt), default_dt);
    }

    #[test]
    fn test_get_datetime_fixed_valid() {
        let default_dt = DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z").unwrap().with_timezone(&Utc);
        let expected_dt = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap().with_timezone(&Utc);
        let args = Arguments::Fixed("2024-01-01T00:00:00Z".to_string());
        assert_eq!(args.get_datetime(default_dt), expected_dt);
    }

    #[test]
    fn test_get_datetime_fixed_space_format() {
        let default_dt = DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z").unwrap().with_timezone(&Utc);
        let expected_dt = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let args = Arguments::Fixed("2024-01-01 00:00:00".to_string());
        assert_eq!(args.get_datetime(default_dt), expected_dt);
    }

    #[test]
    fn test_get_datetime_fixed_invalid() {
        let default_dt = DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z").unwrap().with_timezone(&Utc);
        let args = Arguments::Fixed("invalid-date".to_string());
        assert_eq!(args.get_datetime(default_dt), default_dt);
    }

    #[test]
    fn test_get_datetime_range() {
        let default_start = DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z").unwrap().with_timezone(&Utc);
        let default_end = DateTime::parse_from_rfc3339("2020-12-31T23:59:59Z").unwrap().with_timezone(&Utc);
        let expected_start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let expected_end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap().and_hms_opt(23, 59, 59).unwrap().and_utc();

        let args = Arguments::Range("2024-01-01 00:00:00".to_string(), "2024-12-31 23:59:59".to_string());
        let (start, end) = args.get_datetime_range(default_start, default_end);
        assert_eq!(start, expected_start);
        assert_eq!(end, expected_end);
    }

    #[test]
    fn test_edge_case_empty_content() {
        let args = Arguments::from("()");
        assert!(matches!(args, Arguments::None));
    }

    #[test]
    fn test_edge_case_only_comma() {
        let args = Arguments::from("(,)");
        match args {
            Arguments::Range(start, end) => {
                assert_eq!(start, "");
                assert_eq!(end, "");
            },
            Arguments::Fixed(value) => {
                assert_eq!(value, "");
            },
            Arguments::None => {
                // Also acceptable
            }
        }
    }

    #[test]
    fn test_edge_case_only_dots() {
        let args = Arguments::from("(..)");
        match args {
            Arguments::Range(start, end) => {
                assert_eq!(start, "");
                assert_eq!(end, "");
            },
            Arguments::Fixed(value) => {
                assert_eq!(value, "");
            },
            Arguments::None => {
                // Also acceptable
            }
        }
    }

    #[test]
    fn test_edge_case_trailing_comma() {
        let args = Arguments::from("(123,)");
        if let Arguments::Fixed(value) = args {
            assert_eq!(value, "123");
        } else {
            panic!("Expected Fixed argument");
        }
    }

    #[test]
    fn test_edge_case_trailing_dots() {
        let args = Arguments::from("(123..)");
        if let Arguments::Fixed(value) = args {
            assert_eq!(value, "123");
        } else {
            panic!("Expected Fixed argument");
        }
    }
}
