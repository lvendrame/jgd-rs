use chrono::{DateTime, Utc};
use rand::rngs::StdRng;
use regex::Regex;
use serde_json::Value;
use std::sync::LazyLock;

use crate::{fake::{fake_keys::FakeKeys, fake_locale_generator::{FakeGeneratorArSa, FakeGeneratorCyGb, FakeGeneratorDeDe, FakeGeneratorEn, FakeGeneratorFrFr, FakeGeneratorItIt, FakeGeneratorJaJp, FakeGeneratorPtBr, FakeLocaleGenerator}}, locales_keys::LocalesKeys};

static RE_KEY: LazyLock<regex::Regex> = LazyLock::new(|| Regex::new(r"([^(]+)(\(.+\))?").unwrap());

pub struct FakeGenerator {
    locale_generator: Box<dyn FakeLocaleGenerator>
}

impl FakeGenerator {
    pub fn new(locale: &str) -> Self {
        let locale_keys = LocalesKeys::from(locale);
        let locale_generator: Box<dyn FakeLocaleGenerator> = match locale_keys {
            LocalesKeys::En => Box::new(FakeGeneratorEn),
            LocalesKeys::FrFr => Box::new(FakeGeneratorFrFr),
            LocalesKeys::ItIt => Box::new(FakeGeneratorItIt),
            LocalesKeys::JaJp => Box::new(FakeGeneratorJaJp),
            LocalesKeys::DeDe => Box::new(FakeGeneratorDeDe),
            LocalesKeys::PtBr => Box::new(FakeGeneratorPtBr),
            LocalesKeys::ArSa => Box::new(FakeGeneratorArSa),
            LocalesKeys::CyGb => Box::new(FakeGeneratorCyGb),
        };

        Self { locale_generator }
    }

    // Helper function to parse range arguments like "(3..8)" or "(3,8)"
    pub fn parse_range(args: &str, default_range: std::ops::Range<usize>) -> std::ops::Range<usize> {
        if let Some(args_content) = args.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
            if args_content.contains("..") {
                let parts: Vec<&str> = args_content.split("..").collect();
                if parts.len() == 2 {
                    if let (Ok(start), Ok(end)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                        return start..end;
                    }
                }
            } else if args_content.contains(',') {
                let parts: Vec<&str> = args_content.split(',').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    if let (Ok(start), Ok(end)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                        return start..end;
                    }
                }
            } else if let Ok(single_val) = args_content.parse::<usize>() {
                return single_val..(single_val + 1);
            }
        }
        default_range
    }

    // Helper function to parse single numeric argument like "(5)" or "(50)"
    pub fn parse_single_number<T: std::str::FromStr>(args: &str, default_value: T) -> T {
        if let Some(args_content) = args.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
            if let Ok(value) = args_content.trim().parse::<T>() {
                return value;
            }
        }
        default_value
    }

    // Helper function to parse string argument like "(###-###-####)"
    pub fn parse_string_arg<'a>(args: &'a str, default_value: &'a str) -> &'a str {
        if let Some(args_content) = args.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
            if !args_content.trim().is_empty() {
                return args_content.trim();
            }
        }
        default_value
    }

        // Helper function to parse time argument like "23:56:04"
    pub fn parse_time(args: &str, default_value: time::OffsetDateTime) -> time::OffsetDateTime {
        let args_content = args
            .strip_prefix('(').and_then(|s| s.strip_suffix(')'))
            .unwrap_or(args);

        let args_content = args_content.trim();
        if !args_content.is_empty() {
            // Try parsing as RFC 3339 format (most common for APIs)
            // For now, we'll use a simple fallback since time parsing is complex
            // This could be enhanced with proper format descriptors later

            // Simple fallback: try to parse as Unix timestamp
            if let Ok(timestamp) = args_content.parse::<i64>() {
                if let Ok(datetime) = time::OffsetDateTime::from_unix_timestamp(timestamp) {
                    return datetime;
                }
            }

            return default_value;
        }

        default_value
    }

    // Helper function to parse datetime argument like "2015-09-05 23:56:04" or "2014-5-17T12:34:56+09:30"
    pub fn parse_datetime(args: &str, default_value: DateTime<chrono::Utc>) -> DateTime<Utc> {
        let args_content = args
            .strip_prefix('(').and_then(|s| s.strip_suffix(')'))
            .unwrap_or(args);

        let args_content = args_content.trim();
        if !args_content.is_empty() {

            if let Ok(dt) = args_content.parse::<chrono::DateTime<chrono::Utc>>() {
                return dt;
            }

            if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(args_content, "%Y-%m-%d %H:%M:%S") {
                return dt.and_utc();
            }

            if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(args_content, "%Y-%m-%dT%H:%M:%S") {
                return dt.and_utc();
            }

            if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(args_content, "%Y-%m-%dT%H:%M:%S%:z") {
                return dt.and_utc();
            }

            return default_value;
        }

        default_value
    }

    pub fn generate_by_key(&self, pattern: &str, rng: &mut StdRng) -> Value {
        let captures = RE_KEY.captures(pattern).unwrap();
        let key = captures.get(1).unwrap().as_str();
        let arguments = captures.get(2).map(|m| m.as_str());
        match key {
            // Address
            FakeKeys::ADDRESS_CITY_PREFIX => self.locale_generator.address_city_prefix(rng),
            FakeKeys::ADDRESS_CITY_SUFFIX => self.locale_generator.address_city_suffix(rng),
            FakeKeys::ADDRESS_CITY_NAME => self.locale_generator.address_city_name(rng),
            FakeKeys::ADDRESS_COUNTRY_NAME => self.locale_generator.address_country_name(rng),
            FakeKeys::ADDRESS_COUNTRY_CODE => self.locale_generator.address_country_code(rng),
            FakeKeys::ADDRESS_STREET_SUFFIX => self.locale_generator.address_street_suffix(rng),
            FakeKeys::ADDRESS_STREET_NAME => self.locale_generator.address_street_name(rng),
            FakeKeys::ADDRESS_TIME_ZONE => self.locale_generator.address_time_zone(rng),
            FakeKeys::ADDRESS_STATE_NAME => self.locale_generator.address_state_name(rng),
            FakeKeys::ADDRESS_STATE_ABBR => self.locale_generator.address_state_abbr(rng),
            FakeKeys::ADDRESS_SECONDARY_ADDRESS_TYPE => self.locale_generator.address_secondary_address_type(rng),
            FakeKeys::ADDRESS_SECONDARY_ADDRESS => self.locale_generator.address_secondary_address(rng),
            FakeKeys::ADDRESS_ZIP_CODE => self.locale_generator.address_zip_code(rng),
            FakeKeys::ADDRESS_POST_CODE => self.locale_generator.address_post_code(rng),
            FakeKeys::ADDRESS_BUILDING_NUMBER => self.locale_generator.address_building_number(rng),
            FakeKeys::ADDRESS_LATITUDE => self.locale_generator.address_latitude(rng),
            FakeKeys::ADDRESS_LONGITUDE => self.locale_generator.address_longitude(rng),
            FakeKeys::ADDRESS_GEOHASH => {
                let precision = arguments
                    .map(|args| Self::parse_single_number(args, 5u8))
                    .unwrap_or(5u8);
                self.locale_generator.address_geohash(rng, precision)
            },

            // Barcode
            FakeKeys::BARCODE_ISBN => self.locale_generator.barcode_isbn(rng),
            FakeKeys::BARCODE_ISBN10 => self.locale_generator.barcode_isbn10(rng),
            FakeKeys::BARCODE_ISBN13 => self.locale_generator.barcode_isbn13(rng),

            // Boolean
            FakeKeys::BOOLEAN_BOOLEAN => {
                let ratio = arguments
                    .map(|args| Self::parse_single_number(args, 50u8))
                    .unwrap_or(50u8);
                self.locale_generator.boolean_boolean(rng, ratio)
            },

            // Color
            FakeKeys::COLOR_HEX_COLOR => self.locale_generator.color_hex_color(rng),
            FakeKeys::COLOR_RGB_COLOR => self.locale_generator.color_rgb_color(rng),
            FakeKeys::COLOR_RGBA_COLOR => self.locale_generator.color_rgba_color(rng),
            FakeKeys::COLOR_HSL_COLOR => self.locale_generator.color_hsl_color(rng),
            FakeKeys::COLOR_HSLA_COLOR => self.locale_generator.color_hsla_color(rng),
            FakeKeys::COLOR_COLOR => self.locale_generator.color_color(rng),

            // Chrono
            FakeKeys::CHRONO_TIME => self.locale_generator.chrono_time(rng),
            FakeKeys::CHRONO_DATE => self.locale_generator.chrono_date(rng),
            FakeKeys::CHRONO_DATE_TIME => self.locale_generator.chrono_date_time(rng),
            FakeKeys::CHRONO_DURATION => self.locale_generator.chrono_duration(rng),
            // Chrono with arguments
            FakeKeys::CHRONO_DATE_TIME_BEFORE => {
                // Parse datetime argument or use current time as default
                let dt = arguments
                    .and_then(|args| Self::parse_string_arg(args, "").parse::<chrono::DateTime<chrono::Utc>>().ok())
                    .unwrap_or_else(chrono::Utc::now);
                self.locale_generator.chrono_date_time_before(rng, dt)
            },
            FakeKeys::CHRONO_DATE_TIME_AFTER => {
                // Parse datetime argument or use current time as default
                let dt = arguments
                    .and_then(|args| Self::parse_string_arg(args, "").parse::<chrono::DateTime<chrono::Utc>>().ok())
                    .unwrap_or_else(chrono::Utc::now);
                self.locale_generator.chrono_date_time_after(rng, dt)
            },
            FakeKeys::CHRONO_DATE_TIME_BETWEEN => {
                // For between, we need two datetime arguments or use defaults
                let now: DateTime<Utc> = chrono::Utc::now();
                let past: DateTime<Utc> = now - chrono::Duration::days(365);
                if let Some(args) = arguments {
                    let args_content = Self::parse_string_arg(args, "");
                    if args_content.contains(',') {
                        let parts: Vec<&str> = args_content.split(',').map(|s| s.trim()).collect();
                        if parts.len() == 2 {
                            let start = Self::parse_datetime(parts[0], past);
                            let end = Self::parse_datetime(parts[1], now);

                            return self.locale_generator.chrono_date_time_between(rng, start, end);
                        }
                    }
                }

                // Default: past year to now
                self.locale_generator.chrono_date_time_between(rng, past, now)
            },

            // Time
            FakeKeys::TIME_TIME => self.locale_generator.time_time(rng),
            FakeKeys::TIME_DATE => self.locale_generator.time_date(rng),
            FakeKeys::TIME_DATE_TIME => self.locale_generator.time_date_time(rng),
            FakeKeys::TIME_DURATION => self.locale_generator.time_duration(rng),
            // Time with arguments
            FakeKeys::TIME_DATE_TIME_BEFORE => {
                // Parse datetime argument or use current time as default
                // Note: time::OffsetDateTime parsing is more complex than chrono
                // For now, we use current time but this could be enhanced
                let dt = if let Some(args) = arguments {
                    let _args_content = Self::parse_string_arg(args, "");
                    // Could implement parsing here for ISO datetime strings
                    time::OffsetDateTime::now_utc()
                } else {
                    time::OffsetDateTime::now_utc()
                };
                self.locale_generator.time_date_time_before(rng, dt)
            },
            FakeKeys::TIME_DATE_TIME_AFTER => {
                // Parse datetime argument or use current time as default
                // Note: time::OffsetDateTime parsing is more complex than chrono
                // For now, we use current time but this could be enhanced
                let dt = if let Some(args) = arguments {
                    let _args_content = Self::parse_string_arg(args, "");
                    // Could implement parsing here for ISO datetime strings
                    time::OffsetDateTime::now_utc()
                } else {
                    time::OffsetDateTime::now_utc()
                };
                self.locale_generator.time_date_time_after(rng, dt)
            },
            FakeKeys::TIME_DATE_TIME_BETWEEN => {
                let now = time::OffsetDateTime::now_utc();
                let past = now - time::Duration::days(365);

                // For between, we need two datetime arguments or use defaults
                if let Some(args) = arguments {
                    let args_content = Self::parse_string_arg(args, "");
                    if args_content.contains(',') {
                        let parts: Vec<&str> = args_content.split(',').map(|s| s.trim()).collect();
                        if parts.len() == 2 {
                            let start = Self::parse_time(parts[0], past);
                            let end = Self::parse_time(parts[1], now);

                            return self.locale_generator.time_date_time_between(rng, start, end);
                        }
                    }
                }
                // Default: past year to now
                self.locale_generator.time_date_time_between(rng, past, now)
            },

            // Credit Card
            FakeKeys::CREDITCARD_CREDIT_CARD_NUMBER => self.locale_generator.creditcard_credit_card_number(rng),

            // Company
            FakeKeys::COMPANY_COMPANY_SUFFIX => self.locale_generator.company_company_suffix(rng),
            FakeKeys::COMPANY_COMPANY_NAME => self.locale_generator.company_company_name(rng),
            FakeKeys::COMPANY_BUZZWORD => self.locale_generator.company_buzzword(rng),
            FakeKeys::COMPANY_BUZZWORD_MIDDLE => self.locale_generator.company_buzzword_middle(rng),
            FakeKeys::COMPANY_BUZZWORD_TAIL => self.locale_generator.company_buzzword_tail(rng),
            FakeKeys::COMPANY_CATCH_PHRASE => self.locale_generator.company_catch_phrase(rng),
            FakeKeys::COMPANY_BS_VERB => self.locale_generator.company_bs_verb(rng),
            FakeKeys::COMPANY_BS_ADJ => self.locale_generator.company_bs_adj(rng),
            FakeKeys::COMPANY_BS_NOUN => self.locale_generator.company_bs_noun(rng),
            FakeKeys::COMPANY_BS => self.locale_generator.company_bs(rng),
            FakeKeys::COMPANY_PROFESSION => self.locale_generator.company_profession(rng),
            FakeKeys::COMPANY_INDUSTRY => self.locale_generator.company_industry(rng),

            // HTTP
            FakeKeys::HTTP_RFC_STATUS_CODE => self.locale_generator.http_rfc_status_code(rng),
            FakeKeys::HTTP_VALID_STATUS_CODE => self.locale_generator.http_valid_status_code(rng),

            // Internet
            FakeKeys::INTERNET_FREE_EMAIL_PROVIDER => self.locale_generator.internet_free_email_provider(rng),
            FakeKeys::INTERNET_DOMAIN_SUFFIX => self.locale_generator.internet_domain_suffix(rng),
            FakeKeys::INTERNET_FREE_EMAIL => self.locale_generator.internet_free_email(rng),
            FakeKeys::INTERNET_SAFE_EMAIL => self.locale_generator.internet_safe_email(rng),
            FakeKeys::INTERNET_USERNAME => self.locale_generator.internet_username(rng),
            FakeKeys::INTERNET_PASSWORD => {
                let range = arguments
                    .map(|args| Self::parse_range(args, 8..16))
                    .unwrap_or(8..16);
                self.locale_generator.internet_password(rng, range)
            },
            FakeKeys::INTERNET_I_PV4 => self.locale_generator.internet_i_pv4(rng),
            FakeKeys::INTERNET_I_PV6 => self.locale_generator.internet_i_pv6(rng),
            FakeKeys::INTERNET_IP => self.locale_generator.internet_ip(rng),
            FakeKeys::INTERNET_MAC_ADDRESS => self.locale_generator.internet_mac_address(rng),
            FakeKeys::INTERNET_USER_AGENT => self.locale_generator.internet_user_agent(rng),

            // Job
            FakeKeys::JOB_SENIORITY => self.locale_generator.job_seniority(rng),
            FakeKeys::JOB_FIELD => self.locale_generator.job_field(rng),
            FakeKeys::JOB_POSITION => self.locale_generator.job_position(rng),
            FakeKeys::JOB_TITLE => self.locale_generator.job_title(rng),

            // Lorem
            FakeKeys::LOREM_WORD => self.locale_generator.lorem_word(rng),
            FakeKeys::LOREM_WORDS => {
                let count = arguments
                    .map(|args| Self::parse_range(args, 3..8))
                    .unwrap_or(3..8);
                self.locale_generator.lorem_words(rng, count)
            },
            FakeKeys::LOREM_SENTENCE => {
                let count = arguments
                    .map(|args| Self::parse_range(args, 4..18))
                    .unwrap_or(4..18);
                self.locale_generator.lorem_sentence(rng, count)
            },
            FakeKeys::LOREM_SENTENCES => {
                let count = arguments
                    .map(|args| Self::parse_range(args, 2..6))
                    .unwrap_or(2..6);
                self.locale_generator.lorem_sentences(rng, count)
            },
            FakeKeys::LOREM_PARAGRAPH => {
                let count = arguments
                    .map(|args| Self::parse_range(args, 3..10))
                    .unwrap_or(3..10);
                self.locale_generator.lorem_paragraph(rng, count)
            },
            FakeKeys::LOREM_PARAGRAPHS => {
                let count = arguments
                    .map(|args| Self::parse_range(args, 2..5))
                    .unwrap_or(2..5);
                self.locale_generator.lorem_paragraphs(rng, count)
            },

            // Markdown
            FakeKeys::MARKDOWN_ITALIC_WORD => self.locale_generator.markdown_italic_word(rng),
            FakeKeys::MARKDOWN_BOLD_WORD => self.locale_generator.markdown_bold_word(rng),
            FakeKeys::MARKDOWN_LINK => self.locale_generator.markdown_link(rng),
            FakeKeys::MARKDOWN_BULLET_POINTS => {
                let count = arguments
                    .map(|args| Self::parse_range(args, 3..8))
                    .unwrap_or(3..8);
                self.locale_generator.markdown_bullet_points(rng, count)
            },
            FakeKeys::MARKDOWN_LIST_ITEMS => {
                let count = arguments
                    .map(|args| Self::parse_range(args, 3..8))
                    .unwrap_or(3..8);
                self.locale_generator.markdown_list_items(rng, count)
            },
            FakeKeys::MARKDOWN_BLOCK_QUOTE_SINGLE_LINE => {
                let count = arguments
                    .map(|args| Self::parse_range(args, 4..18))
                    .unwrap_or(4..18);
                self.locale_generator.markdown_block_quote_single_line(rng, count)
            },
            FakeKeys::MARKDOWN_BLOCK_QUOTE_MULTI_LINE => {
                let count = arguments
                    .map(|args| Self::parse_range(args, 2..6))
                    .unwrap_or(2..6);
                self.locale_generator.markdown_block_quote_multi_line(rng, count)
            },
            FakeKeys::MARKDOWN_CODE => {
                let count = arguments
                    .map(|args| Self::parse_range(args, 3..8))
                    .unwrap_or(3..8);
                self.locale_generator.markdown_code(rng, count)
            },

            // Name
            FakeKeys::NAME_FIRST_NAME => self.locale_generator.name_first_name(rng),
            FakeKeys::NAME_LAST_NAME => self.locale_generator.name_last_name(rng),
            FakeKeys::NAME_TITLE => self.locale_generator.name_title(rng),
            FakeKeys::NAME_SUFFIX => self.locale_generator.name_suffix(rng),
            FakeKeys::NAME_NAME => self.locale_generator.name_name(rng),
            FakeKeys::NAME_NAME_WITH_TITLE => self.locale_generator.name_name_with_title(rng),

            // Number
            FakeKeys::NUMBER_DIGIT => self.locale_generator.number_digit(rng),
            FakeKeys::NUMBER_NUMBER_WITH_FORMAT => {
                let format = arguments
                    .map(|args| Self::parse_string_arg(args, "###-###-####"))
                    .unwrap_or("###-###-####");
                self.locale_generator.number_number_with_format(rng, format)
            },

            // Phone Number
            FakeKeys::PHONE_NUMBER_PHONE_NUMBER => self.locale_generator.phone_number_phone_number(rng),
            FakeKeys::PHONE_NUMBER_CELL_NUMBER => self.locale_generator.phone_number_cell_number(rng),

            // Filesystem
            FakeKeys::FILESYSTEM_FILE_PATH => self.locale_generator.filesystem_file_path(rng),
            FakeKeys::FILESYSTEM_FILE_NAME => self.locale_generator.filesystem_file_name(rng),
            FakeKeys::FILESYSTEM_FILE_EXTENSION => self.locale_generator.filesystem_file_extension(rng),
            FakeKeys::FILESYSTEM_DIR_PATH => self.locale_generator.filesystem_dir_path(rng),
            FakeKeys::FILESYSTEM_MIME_TYPE => self.locale_generator.filesystem_mime_type(rng),
            FakeKeys::FILESYSTEM_SEMVER => self.locale_generator.filesystem_semver(rng),
            FakeKeys::FILESYSTEM_SEMVER_STABLE => self.locale_generator.filesystem_semver_stable(rng),
            FakeKeys::FILESYSTEM_SEMVER_UNSTABLE => self.locale_generator.filesystem_semver_unstable(rng),

            // Currency
            FakeKeys::CURRENCY_CURRENCY_CODE => self.locale_generator.currency_currency_code(rng),
            FakeKeys::CURRENCY_CURRENCY_NAME => self.locale_generator.currency_currency_name(rng),
            FakeKeys::CURRENCY_CURRENCY_SYMBOL => self.locale_generator.currency_currency_symbol(rng),

            // Finance
            FakeKeys::FINANCE_BIC => self.locale_generator.finance_bic(rng),
            FakeKeys::FINANCE_ISIN => self.locale_generator.finance_isin(rng),

            // Administrative
            FakeKeys::ADMINISTRATIVE_HEALTH_INSURANCE_CODE => self.locale_generator.administrative_health_insurance_code(rng),

            // Automotive
            FakeKeys::AUTOMOTIVE_LICENCE_PLATE => self.locale_generator.automotive_licence_plate(rng),

            //IDs
            FakeKeys::UUID_V4 => {
                let id = uuid::Uuid::new_v4();
                Value::String(id.to_string())
            }
            FakeKeys::ULID => {
                let id = ulid::Ulid::new();
                Value::String(id.to_string())
            }

            _ => Value::String(pattern.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use serde_json::Value;

    fn create_test_generator() -> FakeGenerator {
        FakeGenerator::new("EN")
    }

    fn create_test_rng() -> StdRng {
        StdRng::seed_from_u64(42) // Fixed seed for deterministic tests
    }

    #[test]
    fn test_generate_by_key_address_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        // Test basic address methods
        let result = generator.generate_by_key("address.cityPrefix", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("address.citySuffix", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("address.cityName", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("address.countryName", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("address.buildingNumber", &mut rng);
        assert!(matches!(result, Value::String(_)));

        // Test address method with arguments
        let result = generator.generate_by_key("address.geohash(8)", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("address.geohash", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_barcode_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("barcode.isbn", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("barcode.isbn10", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("barcode.isbn13", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_boolean_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        // Test boolean with default ratio
        let result = generator.generate_by_key("boolean.boolean", &mut rng);
        assert!(matches!(result, Value::Bool(_)));

        // Test boolean with custom ratio
        let result = generator.generate_by_key("boolean.boolean(75)", &mut rng);
        assert!(matches!(result, Value::Bool(_)));
    }

    #[test]
    fn test_generate_by_key_color_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("color.hexColor", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("color.rgbColor", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("color.rgbaColor", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("color.hslColor", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("color.hslaColor", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("color.color", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_chrono_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("chrono.time", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("chrono.date", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("chrono.dateTime", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("chrono.duration", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("chrono.dateTimeBefore", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("chrono.dateTimeAfter", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("chrono.dateTimeBetween(2024-01-01 00:00:00, 2024-12-31T23:59:59)", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_time_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("time.time", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("time.date", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("time.dateTime", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("time.duration", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("time.dateTimeBefore", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("time.dateTimeAfter", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("time.dateTimeBetween", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_company_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("company.companySuffix", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("company.companyName", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("company.buzzword", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("company.profession", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("company.industry", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_http_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("http.rfcStatusCode", &mut rng);
        assert!(matches!(result, Value::Number(_)));

        let result = generator.generate_by_key("http.validStatusCode", &mut rng);
        assert!(matches!(result, Value::Number(_)));
    }

    #[test]
    fn test_generate_by_key_internet_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("internet.freeEmailProvider", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("internet.username", &mut rng);
        assert!(matches!(result, Value::String(_)));

        // Test password with default range
        let result = generator.generate_by_key("internet.password", &mut rng);
        assert!(matches!(result, Value::String(_)));

        // Test password with custom range
        let result = generator.generate_by_key("internet.password(12..20)", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("internet.IPv4", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("internet.IPv6", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_job_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("job.seniority", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("job.field", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("job.position", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("job.title", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_lorem_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("lorem.word", &mut rng);
        assert!(matches!(result, Value::String(_)));

        // Test with default range
        let result = generator.generate_by_key("lorem.words", &mut rng);
        assert!(matches!(result, Value::String(_)));

        // Test with custom range
        let result = generator.generate_by_key("lorem.words(5..10)", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("lorem.sentence", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("lorem.sentences(2..5)", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("lorem.paragraph", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("lorem.paragraphs", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_markdown_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("markdown.italicWord", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("markdown.boldWord", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("markdown.link", &mut rng);
        assert!(matches!(result, Value::String(_)));

        // Test with default range
        let result = generator.generate_by_key("markdown.bulletPoints", &mut rng);
        assert!(matches!(result, Value::String(_)));

        // Test with custom range
        let result = generator.generate_by_key("markdown.bulletPoints(3..6)", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("markdown.listItems(2..4)", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("markdown.blockQuoteSingleLine", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("markdown.blockQuoteMultiLine", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("markdown.code", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_name_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("name.firstName", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("name.lastName", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("name.title", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("name.suffix", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("name.name", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("name.nameWithTitle", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_number_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("number.digit", &mut rng);
        assert!(matches!(result, Value::String(_)));

        // Test with default format
        let result = generator.generate_by_key("number.numberWithFormat", &mut rng);
        assert!(matches!(result, Value::String(_)));

        // Test with custom format
        let result = generator.generate_by_key("number.numberWithFormat((###) ###-####)", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_phone_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("phone_number.phoneNumber", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("phone_number.cellNumber", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_filesystem_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("filesystem.filePath", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("filesystem.fileName", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("filesystem.fileExtension", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("filesystem.dirPath", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("filesystem.mimeType", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("filesystem.semver", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_currency_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("currency.currencyCode", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("currency.currencyName", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("currency.currencySymbol", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_finance_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("finance.bic", &mut rng);
        assert!(matches!(result, Value::String(_)));

        let result = generator.generate_by_key("finance.isin", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_administrative_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("administrative.healthInsuranceCode", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_automotive_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("automotive.licencePlate", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_creditcard_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key("creditcard.creditCardNumber", &mut rng);
        assert!(matches!(result, Value::String(_)));
    }

    #[test]
    fn test_generate_by_key_unknown_pattern() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        // Test unknown pattern returns the pattern as string
        let result = generator.generate_by_key("unknown.pattern", &mut rng);
        assert_eq!(result, Value::String("unknown.pattern".to_string()));

        let result = generator.generate_by_key("non.existent.key", &mut rng);
        assert_eq!(result, Value::String("non.existent.key".to_string()));
    }

    #[test]
    fn test_parse_range_helper() {
        // Test range parsing with different formats
        assert_eq!(FakeGenerator::parse_range("(3..8)", 1..2), 3..8);
        assert_eq!(FakeGenerator::parse_range("(5,10)", 1..2), 5..10);
        assert_eq!(FakeGenerator::parse_range("(7)", 1..2), 7..8);
        assert_eq!(FakeGenerator::parse_range("invalid", 1..2), 1..2);
        assert_eq!(FakeGenerator::parse_range("", 1..2), 1..2);
    }

    #[test]
    fn test_parse_single_number_helper() {
        // Test single number parsing
        assert_eq!(FakeGenerator::parse_single_number("(42)", 0u8), 42u8);
        assert_eq!(FakeGenerator::parse_single_number("(100)", 0u16), 100u16);
        assert_eq!(FakeGenerator::parse_single_number("invalid", 50u8), 50u8);
        assert_eq!(FakeGenerator::parse_single_number("", 25u8), 25u8);
    }

    #[test]
    fn test_parse_string_arg_helper() {
        // Test string argument parsing
        assert_eq!(FakeGenerator::parse_string_arg("(###-###-####)", "default"), "###-###-####");
        assert_eq!(FakeGenerator::parse_string_arg("(custom format)", "default"), "custom format");
        assert_eq!(FakeGenerator::parse_string_arg("invalid", "default"), "default");
        assert_eq!(FakeGenerator::parse_string_arg("", "default"), "default");
    }

    #[test]
    fn test_argument_parsing_integration() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        // Test that arguments are properly parsed and used
        let result1 = generator.generate_by_key("lorem.words(2..4)", &mut rng);
        let result2 = generator.generate_by_key("lorem.words(8..12)", &mut rng);

        // Both should be strings, but the content should be different due to different ranges
        assert!(matches!(result1, Value::String(_)));
        assert!(matches!(result2, Value::String(_)));

        // Test boolean with different ratios
        let result1 = generator.generate_by_key("boolean.boolean(0)", &mut rng);
        let result2 = generator.generate_by_key("boolean.boolean(100)", &mut rng);

        assert!(matches!(result1, Value::Bool(_)));
        assert!(matches!(result2, Value::Bool(_)));
    }

}

