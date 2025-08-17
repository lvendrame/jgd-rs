use chrono::{DateTime, Utc};
use rand::rngs::StdRng;
use serde_json::Value;

use crate::{fake::{fake_keys::FakeKeys, fake_locale_generator::{FakeGeneratorArSa, FakeGeneratorCyGb, FakeGeneratorDeDe, FakeGeneratorEn, FakeGeneratorFrFr, FakeGeneratorItIt, FakeGeneratorJaJp, FakeGeneratorPtBr, FakeLocaleGenerator}}, locales_keys::LocalesKeys, Replacer};

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

    pub fn generate_by_key(&self, replacer: &Replacer, rng: &mut StdRng) -> Result<Value, String> {
        match replacer.key.as_str() {
            // Address
            FakeKeys::ADDRESS_CITY_PREFIX => Ok(self.locale_generator.address_city_prefix(rng)),
            FakeKeys::ADDRESS_CITY_SUFFIX => Ok(self.locale_generator.address_city_suffix(rng)),
            FakeKeys::ADDRESS_CITY_NAME => Ok(self.locale_generator.address_city_name(rng)),
            FakeKeys::ADDRESS_COUNTRY_NAME => Ok(self.locale_generator.address_country_name(rng)),
            FakeKeys::ADDRESS_COUNTRY_CODE => Ok(self.locale_generator.address_country_code(rng)),
            FakeKeys::ADDRESS_STREET_SUFFIX => Ok(self.locale_generator.address_street_suffix(rng)),
            FakeKeys::ADDRESS_STREET_NAME => Ok(self.locale_generator.address_street_name(rng)),
            FakeKeys::ADDRESS_TIME_ZONE => Ok(self.locale_generator.address_time_zone(rng)),
            FakeKeys::ADDRESS_STATE_NAME => Ok(self.locale_generator.address_state_name(rng)),
            FakeKeys::ADDRESS_STATE_ABBR => Ok(self.locale_generator.address_state_abbr(rng)),
            FakeKeys::ADDRESS_SECONDARY_ADDRESS_TYPE => Ok(self.locale_generator.address_secondary_address_type(rng)),
            FakeKeys::ADDRESS_SECONDARY_ADDRESS => Ok(self.locale_generator.address_secondary_address(rng)),
            FakeKeys::ADDRESS_ZIP_CODE => Ok(self.locale_generator.address_zip_code(rng)),
            FakeKeys::ADDRESS_POST_CODE => Ok(self.locale_generator.address_post_code(rng)),
            FakeKeys::ADDRESS_BUILDING_NUMBER => Ok(self.locale_generator.address_building_number(rng)),
            FakeKeys::ADDRESS_LATITUDE => Ok(self.locale_generator.address_latitude(rng)),
            FakeKeys::ADDRESS_LONGITUDE => Ok(self.locale_generator.address_longitude(rng)),
            FakeKeys::ADDRESS_GEOHASH => {
                let precision = replacer.arguments.get_number(5u8);
                Ok(self.locale_generator.address_geohash(rng, precision))
            },

            // Barcode
            FakeKeys::BARCODE_ISBN => Ok(self.locale_generator.barcode_isbn(rng)),
            FakeKeys::BARCODE_ISBN10 => Ok(self.locale_generator.barcode_isbn10(rng)),
            FakeKeys::BARCODE_ISBN13 => Ok(self.locale_generator.barcode_isbn13(rng)),

            // Boolean
            FakeKeys::BOOLEAN_BOOLEAN => {
                let ratio = replacer.arguments.get_number(5u8);
                Ok(self.locale_generator.boolean_boolean(rng, ratio))
            },

            // Color
            FakeKeys::COLOR_HEX_COLOR => Ok(self.locale_generator.color_hex_color(rng)),
            FakeKeys::COLOR_RGB_COLOR => Ok(self.locale_generator.color_rgb_color(rng)),
            FakeKeys::COLOR_RGBA_COLOR => Ok(self.locale_generator.color_rgba_color(rng)),
            FakeKeys::COLOR_HSL_COLOR => Ok(self.locale_generator.color_hsl_color(rng)),
            FakeKeys::COLOR_HSLA_COLOR => Ok(self.locale_generator.color_hsla_color(rng)),
            FakeKeys::COLOR_COLOR => Ok(self.locale_generator.color_color(rng)),

            // Chrono
            FakeKeys::CHRONO_TIME => Ok(self.locale_generator.chrono_time(rng)),
            FakeKeys::CHRONO_DATE => Ok(self.locale_generator.chrono_date(rng)),
            FakeKeys::CHRONO_DATE_TIME => Ok(self.locale_generator.chrono_date_time(rng)),
            FakeKeys::CHRONO_DURATION => Ok(self.locale_generator.chrono_duration(rng)),
            // Chrono with arguments
            FakeKeys::CHRONO_DATE_TIME_BEFORE => {
                // Parse datetime argument or use current time as default
                let dt = replacer.arguments.get_datetime(chrono::Utc::now());
                Ok(self.locale_generator.chrono_date_time_before(rng, dt))
            },
            FakeKeys::CHRONO_DATE_TIME_AFTER => {
                // Parse datetime argument or use current time as default
                let dt = replacer.arguments.get_datetime(chrono::Utc::now());
                Ok(self.locale_generator.chrono_date_time_after(rng, dt))
            },
            FakeKeys::CHRONO_DATE_TIME_BETWEEN => {
                // For between, we need two datetime arguments or use defaults
                let now: DateTime<Utc> = chrono::Utc::now();
                let (start, end) = replacer.arguments
                    .get_datetime_range(now - chrono::Duration::days(365), now);

                // Default: past year to now
                Ok(self.locale_generator.chrono_date_time_between(rng, start, end))
            },

            // Time
            FakeKeys::TIME_TIME => Ok(self.locale_generator.time_time(rng)),
            FakeKeys::TIME_DATE => Ok(self.locale_generator.time_date(rng)),
            FakeKeys::TIME_DATE_TIME => Ok(self.locale_generator.time_date_time(rng)),
            FakeKeys::TIME_DURATION => Ok(self.locale_generator.time_duration(rng)),
            // Time with arguments
            FakeKeys::TIME_DATE_TIME_BEFORE => {
                let dt = replacer.arguments.get_time(time::OffsetDateTime::now_utc());
                Ok(self.locale_generator.time_date_time_before(rng, dt))
            },
            FakeKeys::TIME_DATE_TIME_AFTER => {
                let dt = replacer.arguments.get_time(time::OffsetDateTime::now_utc());
                Ok(self.locale_generator.time_date_time_after(rng, dt))
            },
            FakeKeys::TIME_DATE_TIME_BETWEEN => {
                let now = time::OffsetDateTime::now_utc();

                let (start, end) = replacer.arguments
                    .get_time_range(now - time::Duration::days(365), now);

                // Default: past year to now
                Ok(self.locale_generator.time_date_time_between(rng, start, end))
            },

            // Credit Card
            FakeKeys::CREDITCARD_CREDIT_CARD_NUMBER => Ok(self.locale_generator.creditcard_credit_card_number(rng)),

            // Company
            FakeKeys::COMPANY_COMPANY_SUFFIX => Ok(self.locale_generator.company_company_suffix(rng)),
            FakeKeys::COMPANY_COMPANY_NAME => Ok(self.locale_generator.company_company_name(rng)),
            FakeKeys::COMPANY_BUZZWORD => Ok(self.locale_generator.company_buzzword(rng)),
            FakeKeys::COMPANY_BUZZWORD_MIDDLE => Ok(self.locale_generator.company_buzzword_middle(rng)),
            FakeKeys::COMPANY_BUZZWORD_TAIL => Ok(self.locale_generator.company_buzzword_tail(rng)),
            FakeKeys::COMPANY_CATCH_PHRASE => Ok(self.locale_generator.company_catch_phrase(rng)),
            FakeKeys::COMPANY_BS_VERB => Ok(self.locale_generator.company_bs_verb(rng)),
            FakeKeys::COMPANY_BS_ADJ => Ok(self.locale_generator.company_bs_adj(rng)),
            FakeKeys::COMPANY_BS_NOUN => Ok(self.locale_generator.company_bs_noun(rng)),
            FakeKeys::COMPANY_BS => Ok(self.locale_generator.company_bs(rng)),
            FakeKeys::COMPANY_PROFESSION => Ok(self.locale_generator.company_profession(rng)),
            FakeKeys::COMPANY_INDUSTRY => Ok(self.locale_generator.company_industry(rng)),

            // HTTP
            FakeKeys::HTTP_RFC_STATUS_CODE => Ok(self.locale_generator.http_rfc_status_code(rng)),
            FakeKeys::HTTP_VALID_STATUS_CODE => Ok(self.locale_generator.http_valid_status_code(rng)),

            // Internet
            FakeKeys::INTERNET_FREE_EMAIL_PROVIDER => Ok(self.locale_generator.internet_free_email_provider(rng)),
            FakeKeys::INTERNET_DOMAIN_SUFFIX => Ok(self.locale_generator.internet_domain_suffix(rng)),
            FakeKeys::INTERNET_FREE_EMAIL => Ok(self.locale_generator.internet_free_email(rng)),
            FakeKeys::INTERNET_SAFE_EMAIL => Ok(self.locale_generator.internet_safe_email(rng)),
            FakeKeys::INTERNET_USERNAME => Ok(self.locale_generator.internet_username(rng)),
            FakeKeys::INTERNET_PASSWORD => {
                let range = replacer.arguments.get_number_range(8, 16);
                Ok(self.locale_generator.internet_password(rng, range))
            },
            FakeKeys::INTERNET_I_PV4 => Ok(self.locale_generator.internet_i_pv4(rng)),
            FakeKeys::INTERNET_I_PV6 => Ok(self.locale_generator.internet_i_pv6(rng)),
            FakeKeys::INTERNET_IP => Ok(self.locale_generator.internet_ip(rng)),
            FakeKeys::INTERNET_MAC_ADDRESS => Ok(self.locale_generator.internet_mac_address(rng)),
            FakeKeys::INTERNET_USER_AGENT => Ok(self.locale_generator.internet_user_agent(rng)),

            // Job
            FakeKeys::JOB_SENIORITY => Ok(self.locale_generator.job_seniority(rng)),
            FakeKeys::JOB_FIELD => Ok(self.locale_generator.job_field(rng)),
            FakeKeys::JOB_POSITION => Ok(self.locale_generator.job_position(rng)),
            FakeKeys::JOB_TITLE => Ok(self.locale_generator.job_title(rng)),

            // Lorem
            FakeKeys::LOREM_WORD => Ok(self.locale_generator.lorem_word(rng)),
            FakeKeys::LOREM_WORDS => {
                let count = replacer.arguments.get_number_range(3, 8);
                Ok(self.locale_generator.lorem_words(rng, count))
            },
            FakeKeys::LOREM_SENTENCE => {
                let count = replacer.arguments.get_number_range(4, 18);
                Ok(self.locale_generator.lorem_sentence(rng, count))
            },
            FakeKeys::LOREM_SENTENCES => {
                let count = replacer.arguments.get_number_range(2, 6);
                Ok(self.locale_generator.lorem_sentences(rng, count))
            },
            FakeKeys::LOREM_PARAGRAPH => {
                let count = replacer.arguments.get_number_range(3, 10);
                Ok(self.locale_generator.lorem_paragraph(rng, count))
            },
            FakeKeys::LOREM_PARAGRAPHS => {
                let count = replacer.arguments.get_number_range(2, 5);
                Ok(self.locale_generator.lorem_paragraphs(rng, count))
            },

            // Markdown
            FakeKeys::MARKDOWN_ITALIC_WORD => Ok(self.locale_generator.markdown_italic_word(rng)),
            FakeKeys::MARKDOWN_BOLD_WORD => Ok(self.locale_generator.markdown_bold_word(rng)),
            FakeKeys::MARKDOWN_LINK => Ok(self.locale_generator.markdown_link(rng)),
            FakeKeys::MARKDOWN_BULLET_POINTS => {
                let count = replacer.arguments.get_number_range(3, 8);
                Ok(self.locale_generator.markdown_bullet_points(rng, count))
            },
            FakeKeys::MARKDOWN_LIST_ITEMS => {
                let count = replacer.arguments.get_number_range(3, 8);
                Ok(self.locale_generator.markdown_list_items(rng, count))
            },
            FakeKeys::MARKDOWN_BLOCK_QUOTE_SINGLE_LINE => {
                let count = replacer.arguments.get_number_range(4, 18);
                Ok(self.locale_generator.markdown_block_quote_single_line(rng, count))
            },
            FakeKeys::MARKDOWN_BLOCK_QUOTE_MULTI_LINE => {
                let count = replacer.arguments.get_number_range(2, 6);
                Ok(self.locale_generator.markdown_block_quote_multi_line(rng, count))
            },
            FakeKeys::MARKDOWN_CODE => {
                let count = replacer.arguments.get_number_range(3, 8);
                Ok(self.locale_generator.markdown_code(rng, count))
            },

            // Name
            FakeKeys::NAME_FIRST_NAME => Ok(self.locale_generator.name_first_name(rng)),
            FakeKeys::NAME_LAST_NAME => Ok(self.locale_generator.name_last_name(rng)),
            FakeKeys::NAME_TITLE => Ok(self.locale_generator.name_title(rng)),
            FakeKeys::NAME_SUFFIX => Ok(self.locale_generator.name_suffix(rng)),
            FakeKeys::NAME_NAME => Ok(self.locale_generator.name_name(rng)),
            FakeKeys::NAME_NAME_WITH_TITLE => Ok(self.locale_generator.name_name_with_title(rng)),

            // Number
            FakeKeys::NUMBER_DIGIT => Ok(self.locale_generator.number_digit(rng)),
            FakeKeys::NUMBER_NUMBER_WITH_FORMAT => {
                let format = replacer.arguments.get_string("###-###-####");
                Ok(self.locale_generator.number_number_with_format(rng, format))
            },

            // Phone Number
            FakeKeys::PHONE_NUMBER_PHONE_NUMBER => Ok(self.locale_generator.phone_number_phone_number(rng)),
            FakeKeys::PHONE_NUMBER_CELL_NUMBER => Ok(self.locale_generator.phone_number_cell_number(rng)),

            // Filesystem
            FakeKeys::FILESYSTEM_FILE_PATH => Ok(self.locale_generator.filesystem_file_path(rng)),
            FakeKeys::FILESYSTEM_FILE_NAME => Ok(self.locale_generator.filesystem_file_name(rng)),
            FakeKeys::FILESYSTEM_FILE_EXTENSION => Ok(self.locale_generator.filesystem_file_extension(rng)),
            FakeKeys::FILESYSTEM_DIR_PATH => Ok(self.locale_generator.filesystem_dir_path(rng)),
            FakeKeys::FILESYSTEM_MIME_TYPE => Ok(self.locale_generator.filesystem_mime_type(rng)),
            FakeKeys::FILESYSTEM_SEMVER => Ok(self.locale_generator.filesystem_semver(rng)),
            FakeKeys::FILESYSTEM_SEMVER_STABLE => Ok(self.locale_generator.filesystem_semver_stable(rng)),
            FakeKeys::FILESYSTEM_SEMVER_UNSTABLE => Ok(self.locale_generator.filesystem_semver_unstable(rng)),

            // Currency
            FakeKeys::CURRENCY_CURRENCY_CODE => Ok(self.locale_generator.currency_currency_code(rng)),
            FakeKeys::CURRENCY_CURRENCY_NAME => Ok(self.locale_generator.currency_currency_name(rng)),
            FakeKeys::CURRENCY_CURRENCY_SYMBOL => Ok(self.locale_generator.currency_currency_symbol(rng)),

            // Finance
            FakeKeys::FINANCE_BIC => Ok(self.locale_generator.finance_bic(rng)),
            FakeKeys::FINANCE_ISIN => Ok(self.locale_generator.finance_isin(rng)),

            // Administrative
            FakeKeys::ADMINISTRATIVE_HEALTH_INSURANCE_CODE => Ok(self.locale_generator.administrative_health_insurance_code(rng)),

            // Automotive
            FakeKeys::AUTOMOTIVE_LICENCE_PLATE => Ok(self.locale_generator.automotive_licence_plate(rng)),

            //IDs
            FakeKeys::UUID_V4 => {
                let id = uuid::Uuid::new_v4();
                Ok(Value::String(id.to_string()))
            }
            FakeKeys::ULID => {
                let id = ulid::Ulid::new();
                Ok(Value::String(id.to_string()))
            }

            _ => Err(format!("Error to generate unknown key {}", replacer.tag)),
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
        let result = generator.generate_by_key(&Replacer::from("${address.cityPrefix}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${address.citySuffix}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${address.cityName}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${address.countryName}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${address.buildingNumber}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        // Test address method with arguments
        let result = generator.generate_by_key(&Replacer::from("${address.geohash(8)}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${address.geohash}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_barcode_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${barcode.isbn}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${barcode.isbn10}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${barcode.isbn13}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_boolean_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        // Test boolean with default ratio
        let result = generator.generate_by_key(&Replacer::from("${boolean.boolean}"), &mut rng);
        assert!(matches!(result, Ok(Value::Bool(_))));

        // Test boolean with custom ratio
        let result = generator.generate_by_key(&Replacer::from("${boolean.boolean(75)}"), &mut rng);
        assert!(matches!(result, Ok(Value::Bool(_))));
    }

    #[test]
    fn test_generate_by_key_color_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${color.hexColor}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${color.rgbColor}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${color.rgbaColor}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${color.hslColor}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${color.hslaColor}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${color.color}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_chrono_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${chrono.time}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${chrono.date}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${chrono.dateTime}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${chrono.duration}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${chrono.dateTimeBefore}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${chrono.dateTimeAfter}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${chrono.dateTimeBetween(2024-01-01 00:00:00, 2024-12-31T23:59:59)}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_time_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${time.time}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${time.date}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${time.dateTime}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${time.duration}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${time.dateTimeBefore}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${time.dateTimeAfter}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${time.dateTimeBetween}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_company_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${company.companySuffix}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${company.companyName}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${company.buzzword}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${company.profession}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${company.industry}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_http_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${http.rfcStatusCode}"), &mut rng);
        assert!(matches!(result, Ok(Value::Number(_))));

        let result = generator.generate_by_key(&Replacer::from("${http.validStatusCode}"), &mut rng);
        assert!(matches!(result, Ok(Value::Number(_))));
    }

    #[test]
    fn test_generate_by_key_internet_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${internet.freeEmailProvider}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${internet.username}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        // Test password with default range
        let result = generator.generate_by_key(&Replacer::from("${internet.password}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        // Test password with custom range
        let result = generator.generate_by_key(&Replacer::from("${internet.password(12..20)}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${internet.IPv4}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${internet.IPv6}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_job_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${job.seniority}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${job.field}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${job.position}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${job.title}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_lorem_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${lorem.word}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        // Test with default range
        let result = generator.generate_by_key(&Replacer::from("${lorem.words}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        // Test with custom range
        let result = generator.generate_by_key(&Replacer::from("${lorem.words(5..10)}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${lorem.sentence}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${lorem.sentences(2..5)}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${lorem.paragraph}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${lorem.paragraphs}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_markdown_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${markdown.italicWord}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${markdown.boldWord}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${markdown.link}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        // Test with default range
        let result = generator.generate_by_key(&Replacer::from("${markdown.bulletPoints}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        // Test with custom range
        let result = generator.generate_by_key(&Replacer::from("${markdown.bulletPoints(3..6)}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${markdown.listItems(2..4)}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${markdown.blockQuoteSingleLine}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${markdown.blockQuoteMultiLine}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${markdown.code}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_name_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${name.firstName}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${name.lastName}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${name.title}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${name.suffix}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${name.name}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${name.nameWithTitle}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_number_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${number.digit}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        // Test with default format
        let result = generator.generate_by_key(&Replacer::from("${number.numberWithFormat}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        // Test with custom format
        let result = generator.generate_by_key(&Replacer::from("${number.numberWithFormat((###) ###-####)}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_phone_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${phone_number.phoneNumber}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${phone_number.cellNumber}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_filesystem_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${filesystem.filePath}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${filesystem.fileName}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${filesystem.fileExtension}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${filesystem.dirPath}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${filesystem.mimeType}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${filesystem.semver}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_currency_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${currency.currencyCode}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${currency.currencyName}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${currency.currencySymbol}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_finance_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${finance.bic}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));

        let result = generator.generate_by_key(&Replacer::from("${finance.isin}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_administrative_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${administrative.healthInsuranceCode}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_automotive_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${automotive.licencePlate}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_creditcard_methods() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        let result = generator.generate_by_key(&Replacer::from("${creditcard.creditCardNumber}"), &mut rng);
        assert!(matches!(result, Ok(Value::String(_))));
    }

    #[test]
    fn test_generate_by_key_unknown_pattern() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        // Test unknown pattern returns an error
        let result = generator.generate_by_key(&Replacer::from("${unknown.pattern}"), &mut rng);
        assert!(result.is_err());

        let result = generator.generate_by_key(&Replacer::from("${non.existent.key}"), &mut rng);
        assert!(result.is_err());
    }

    #[test]
    fn test_argument_parsing_integration() {
        let generator = create_test_generator();
        let mut rng = create_test_rng();

        // Test that arguments are properly parsed and used
        let result1 = generator.generate_by_key(&Replacer::from("${lorem.words(2..4)}"), &mut rng);
        let result2 = generator.generate_by_key(&Replacer::from("${lorem.words(8..12)}"), &mut rng);

        // Both should be strings, but the content should be different due to different ranges
        assert!(matches!(result1, Ok(Value::String(_))));
        assert!(matches!(result2, Ok(Value::String(_))));

        // Test boolean with different ratios
        let result1 = generator.generate_by_key(&Replacer::from("${boolean.boolean(0)}"), &mut rng);
        let result2 = generator.generate_by_key(&Replacer::from("${boolean.boolean(100)}"), &mut rng);

        assert!(matches!(result1, Ok(Value::Bool(_))));
        assert!(matches!(result2, Ok(Value::Bool(_))));
    }

}

