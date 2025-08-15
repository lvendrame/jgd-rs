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
