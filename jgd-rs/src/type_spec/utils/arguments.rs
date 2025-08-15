use chrono::{DateTime, NaiveDateTime, Utc};

pub enum Arguments {
    None,
    Fixed(String),
    Range(String, String),
}

impl From<&str> for Arguments {
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

    /// Helper function to parse single numeric argument like "5" or "35.7"
    fn parse_number<T: std::str::FromStr>(arg: &str, default_value: T) -> T {
        if let Ok(value) = arg.parse::<T>() {
            return value;
        }

        default_value
    }

    // Helper function to parse time argument like "23:56:04"
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

    // Helper function to parse datetime argument like "2015-09-05 23:56:04" or "2014-5-17T12:34:56+09:30"
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

    pub fn get_string<'a>(&'a self, default_value: &'a str) -> &'a str {
        match self {
            Arguments::None => default_value,
            Arguments::Fixed(arg) => arg,
            Arguments::Range(arg, _) => arg,
        }
    }

    pub fn get_string_tuple<'a>(&'a self, default_start: &'a str, default_end: &'a str) -> (&'a str, &'a str) {
        match self {
            Arguments::None => (default_start, default_end),
            Arguments::Fixed(arg) => (arg, default_end),
            Arguments::Range(arg1, arg2) => (arg1, arg2),
        }
    }

    pub fn get_number<T: std::str::FromStr>(&self, default_value: T) -> T {
        match self {
            Arguments::None => default_value,
            Arguments::Fixed(arg) => Self::parse_number(arg, default_value),
            Arguments::Range(arg, _) => Self::parse_number(arg, default_value),
        }
    }

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

    pub fn get_time(&self, default_value: time::OffsetDateTime) -> time::OffsetDateTime {
        match self {
            Arguments::None => default_value,
            Arguments::Fixed(arg) => Self::parse_time(arg, default_value),
            Arguments::Range(arg, _) => Self::parse_time(arg, default_value),
        }
    }

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

    pub fn get_datetime(&self, default_value: DateTime<Utc>) -> DateTime<Utc> {
        match self {
            Arguments::None => default_value,
            Arguments::Fixed(arg) => Self::parse_datetime(arg, default_value),
            Arguments::Range(arg, _) => Self::parse_datetime(arg, default_value),
        }
    }

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
