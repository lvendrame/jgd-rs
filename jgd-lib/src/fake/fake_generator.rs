use fake::{faker, Fake};
use rand::rngs::StdRng;
use regex::Regex;
use serde_json::Value;
use std::sync::LazyLock;

use crate::fake::{fake_keys::FakeKeys, fake_locale_generator::FakeLocaleGenerator};

static RE_KEY: LazyLock<regex::Regex> = LazyLock::new(|| Regex::new(r"([^(]+)(\(.+\))?").unwrap());

pub struct FakeGenerator {
    locale_generator: Box<dyn FakeLocaleGenerator>
}

impl FakeGenerator {
    pub fn new(locale_generator: Box<dyn FakeLocaleGenerator>) -> Self {
        Self { locale_generator }
    }

    // Helper function to parse range arguments like "(3..8)" or "(3,8)"
    fn parse_range(args: &str, default_range: std::ops::Range<usize>) -> std::ops::Range<usize> {
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
    fn parse_single_number<T: std::str::FromStr>(args: &str, default_value: T) -> T {
        if let Some(args_content) = args.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
            if let Ok(value) = args_content.trim().parse::<T>() {
                return value;
            }
        }
        default_value
    }

    // Helper function to parse string argument like "(###-###-####)"
    fn parse_string_arg<'a>(args: &'a str, default_value: &'a str) -> &'a str {
        if let Some(args_content) = args.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
            if !args_content.trim().is_empty() {
                return args_content.trim();
            }
        }
        default_value
    }

        pub fn address_city_prefix(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::CityPrefix().fake_with_rng(rng))
        }

        pub fn address_city_suffix(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::CitySuffix().fake_with_rng(rng))
        }
        pub fn address_city_name(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::CityName().fake_with_rng(rng))
        }
        pub fn address_country_name(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::CountryName().fake_with_rng(rng))
        }
        pub fn address_country_code(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::CountryCode().fake_with_rng(rng))
        }
        pub fn address_street_suffix(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::StreetSuffix().fake_with_rng(rng))
        }
        pub fn address_street_name(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::StreetName().fake_with_rng(rng))
        }
        pub fn address_time_zone(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::TimeZone().fake_with_rng(rng))
        }
        pub fn address_state_name(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::StateName().fake_with_rng(rng))
        }
        pub fn address_state_abbr(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::StateAbbr().fake_with_rng(rng))
        }
        pub fn address_secondary_address_type(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::SecondaryAddressType().fake_with_rng(rng))
        }
        pub fn address_secondary_address(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::SecondaryAddress().fake_with_rng(rng))
        }
        pub fn address_zip_code(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::ZipCode().fake_with_rng(rng))
        }
        pub fn address_post_code(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::PostCode().fake_with_rng(rng))
        }
        pub fn address_building_number(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::BuildingNumber().fake_with_rng(rng))
        }
        pub fn address_latitude(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::Latitude().fake_with_rng(rng))
        }
        pub fn address_longitude(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::address::en::Longitude().fake_with_rng(rng))
        }
        pub fn address_geohash(&self, rng: &mut StdRng, precision: u8) -> Value {
            Value::String(faker::address::en::Geohash(precision).fake_with_rng(rng))
        }
        pub fn barcode_isbn(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::barcode::en::Isbn().fake_with_rng(rng))
        }
        pub fn barcode_isbn10(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::barcode::en::Isbn10().fake_with_rng(rng))
        }
        pub fn barcode_isbn13(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::barcode::en::Isbn13().fake_with_rng(rng))
        }
        pub fn boolean_boolean(&self, rng: &mut StdRng, ratio: u8) -> Value {
            Value::Bool(faker::boolean::en::Boolean(ratio).fake_with_rng(rng))
        }
        pub fn color_hex_color(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::color::en::HexColor().fake_with_rng(rng))
        }
        pub fn color_rgb_color(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::color::en::RgbColor().fake_with_rng(rng))
        }
        pub fn color_rgba_color(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::color::en::RgbaColor().fake_with_rng(rng))
        }
        pub fn color_hsl_color(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::color::en::HslColor().fake_with_rng(rng))
        }
        pub fn color_hsla_color(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::color::en::HslaColor().fake_with_rng(rng))
        }
        pub fn color_color(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::color::en::Color().fake_with_rng(rng))
        }
        pub fn chrono_time(&self, rng: &mut StdRng) -> Value {
            let time: chrono::NaiveTime = faker::chrono::en::Time().fake_with_rng(rng);
            Value::String(time.to_string())
        }
        pub fn chrono_date(&self, rng: &mut StdRng) -> Value {
            let date: chrono::NaiveDate = faker::chrono::en::Date().fake_with_rng(rng);
            Value::String(date.to_string())
        }
        pub fn chrono_date_time(&self, rng: &mut StdRng) -> Value {
            let dt: chrono::DateTime<chrono::Utc> = faker::chrono::en::DateTime().fake_with_rng(rng);
            Value::String(dt.to_rfc3339())
        }
        pub fn chrono_duration(&self, rng: &mut StdRng) -> Value {
            let duration: chrono::Duration = faker::chrono::en::Duration().fake_with_rng(rng);
            Value::String(duration.to_string())
        }
        pub fn chrono_date_time_before(&self, rng: &mut StdRng, dt: chrono::DateTime<chrono::Utc>) -> Value {
            let before: chrono::DateTime<chrono::Utc> = faker::chrono::en::DateTimeBefore(dt).fake_with_rng(rng);
            Value::String(before.to_rfc3339())
        }
        pub fn chrono_date_time_after(&self, rng: &mut StdRng, dt: chrono::DateTime<chrono::Utc>) -> Value {
            let after: chrono::DateTime<chrono::Utc> = faker::chrono::en::DateTimeAfter(dt).fake_with_rng(rng);
            Value::String(after.to_rfc3339())
        }
        pub fn chrono_date_time_between(&self, rng: &mut StdRng, start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Value {
            let between: chrono::DateTime<chrono::Utc> = faker::chrono::en::DateTimeBetween(start, end).fake_with_rng(rng);
            Value::String(between.to_rfc3339())
        }
        pub fn time_time(&self, rng: &mut StdRng) -> Value {
            let time: time::Time = faker::time::en::Time().fake_with_rng(rng);
            Value::String(time.to_string())
        }
        pub fn time_date(&self, rng: &mut StdRng) -> Value {
            let date: time::Date = faker::time::en::Date().fake_with_rng(rng);
            Value::String(date.to_string())
        }
        pub fn time_date_time(&self, rng: &mut StdRng) -> Value {
            let dt: time::OffsetDateTime = faker::time::en::DateTime().fake_with_rng(rng);
            Value::String(dt.to_string())
        }
        pub fn time_duration(&self, rng: &mut StdRng) -> Value {
            let duration: time::Duration = faker::time::en::Duration().fake_with_rng(rng);
            Value::String(duration.to_string())
        }
        pub fn time_date_time_before(&self, rng: &mut StdRng, dt: time::OffsetDateTime) -> Value {
            let before: time::OffsetDateTime = faker::time::en::DateTimeBefore(dt).fake_with_rng(rng);
            Value::String(before.to_string())
        }
        pub fn time_date_time_after(&self, rng: &mut StdRng, dt: time::OffsetDateTime) -> Value {
            let after: time::OffsetDateTime = faker::time::en::DateTimeAfter(dt).fake_with_rng(rng);
            Value::String(after.to_string())
        }
        pub fn time_date_time_between(&self, rng: &mut StdRng, start: time::OffsetDateTime, end: time::OffsetDateTime) -> Value {
            let between: time::OffsetDateTime = faker::time::en::DateTimeBetween(start, end).fake_with_rng(rng);
            Value::String(between.to_string())
        }
        pub fn creditcard_credit_card_number(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::creditcard::en::CreditCardNumber().fake_with_rng(rng))
        }
        pub fn company_company_suffix(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::company::en::CompanySuffix().fake_with_rng(rng))
        }
        pub fn company_company_name(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::company::en::CompanyName().fake_with_rng(rng))
        }
        pub fn company_buzzword(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::company::en::Buzzword().fake_with_rng(rng))
        }
        pub fn company_buzzword_middle(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::company::en::BuzzwordMiddle().fake_with_rng(rng))
        }
        pub fn company_buzzword_tail(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::company::en::BuzzwordTail().fake_with_rng(rng))
        }
        pub fn company_catch_phrase(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::company::en::CatchPhrase().fake_with_rng(rng))
        }
        pub fn company_bs_verb(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::company::en::BsVerb().fake_with_rng(rng))
        }
        pub fn company_bs_adj(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::company::en::BsAdj().fake_with_rng(rng))
        }
        pub fn company_bs_noun(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::company::en::BsNoun().fake_with_rng(rng))
        }
        pub fn company_bs(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::company::en::Bs().fake_with_rng(rng))
        }
        pub fn company_profession(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::company::en::Profession().fake_with_rng(rng))
        }
        pub fn company_industry(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::company::en::Industry().fake_with_rng(rng))
        }
        pub fn http_rfc_status_code(&self, rng: &mut StdRng) -> Value {
            let codes = [200, 201, 204, 301, 302, 400, 401, 403, 404, 500, 502, 503];
            let idx: usize = (0..codes.len()).fake_with_rng(rng);
            Value::Number(codes[idx].into())
        }
        pub fn http_valid_status_code(&self, rng: &mut StdRng) -> Value {
            let codes = [200, 201, 204, 301, 302, 400, 401, 403, 404];
            let idx: usize = (0..codes.len()).fake_with_rng(rng);
            Value::Number(codes[idx].into())
        }
        pub fn internet_free_email_provider(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::internet::en::FreeEmailProvider().fake_with_rng(rng))
        }
        pub fn internet_domain_suffix(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::internet::en::DomainSuffix().fake_with_rng(rng))
        }
        pub fn internet_free_email(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::internet::en::FreeEmail().fake_with_rng(rng))
        }
        pub fn internet_safe_email(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::internet::en::SafeEmail().fake_with_rng(rng))
        }
        pub fn internet_username(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::internet::en::Username().fake_with_rng(rng))
        }
        pub fn internet_password(&self, rng: &mut StdRng, len_range: std::ops::Range<usize>) -> Value {
            Value::String(faker::internet::en::Password(len_range).fake_with_rng(rng))
        }
        pub fn internet_i_pv4(&self, rng: &mut StdRng) -> Value {
            let ipv4: std::net::Ipv4Addr = faker::internet::en::IPv4().fake_with_rng(rng);
            Value::String(ipv4.to_string())
        }
        pub fn internet_i_pv6(&self, rng: &mut StdRng) -> Value {
            let ipv6: std::net::Ipv6Addr = faker::internet::en::IPv6().fake_with_rng(rng);
            Value::String(ipv6.to_string())
        }
        pub fn internet_ip(&self, rng: &mut StdRng) -> Value {
            let ip: std::net::IpAddr = faker::internet::en::IP().fake_with_rng(rng);
            Value::String(ip.to_string())
        }
        pub fn internet_mac_address(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::internet::en::MACAddress().fake_with_rng(rng))
        }
        pub fn internet_user_agent(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::internet::en::UserAgent().fake_with_rng(rng))
        }
        pub fn job_seniority(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::job::en::Seniority().fake_with_rng(rng))
        }
        pub fn job_field(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::job::en::Field().fake_with_rng(rng))
        }
        pub fn job_position(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::job::en::Position().fake_with_rng(rng))
        }
        pub fn job_title(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::job::en::Title().fake_with_rng(rng))
        }
        pub fn lorem_word(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::lorem::en::Word().fake_with_rng(rng))
        }
        pub fn lorem_words(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
            let words: Vec<String> = faker::lorem::en::Words(count).fake_with_rng(rng);
            Value::String(words.join(" "))
        }
        pub fn lorem_sentence(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
            Value::String(faker::lorem::en::Sentence(count).fake_with_rng(rng))
        }
        pub fn lorem_sentences(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
            let sentences: Vec<String> = faker::lorem::en::Sentences(count).fake_with_rng(rng);
            Value::String(sentences.join(" "))
        }
        pub fn lorem_paragraph(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
            Value::String(faker::lorem::en::Paragraph(count).fake_with_rng(rng))
        }
        pub fn lorem_paragraphs(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
            let paragraphs: Vec<String> = faker::lorem::en::Paragraphs(count).fake_with_rng(rng);
            Value::String(paragraphs.join("\n\n"))
        }
        pub fn markdown_italic_word(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::markdown::en::ItalicWord().fake_with_rng(rng))
        }
        pub fn markdown_bold_word(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::markdown::en::BoldWord().fake_with_rng(rng))
        }
        pub fn markdown_link(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::markdown::en::Link().fake_with_rng(rng))
        }
        pub fn markdown_bullet_points(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
            // Use custom implementation as faker::markdown::BulletPoints might not be available
            let num_items: usize = count.fake_with_rng(rng);
            let mut items = Vec::new();
            for _ in 0..num_items {
                let item: String = faker::lorem::en::Sentence(3..8).fake_with_rng(rng);
                items.push(format!("• {}", item));
            }
            Value::String(items.join("\n"))
        }
        pub fn markdown_list_items(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
            // Use custom implementation as faker::markdown::ListItems might not be available
            let num_items: usize = count.fake_with_rng(rng);
            let mut items = Vec::new();
            for i in 1..=num_items {
                let item: String = faker::lorem::en::Sentence(3..8).fake_with_rng(rng);
                items.push(format!("{}. {}", i, item));
            }
            Value::String(items.join("\n"))
        }
        pub fn markdown_block_quote_single_line(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
            Value::String(faker::markdown::en::BlockQuoteSingleLine(count).fake_with_rng(rng))
        }
        pub fn markdown_block_quote_multi_line(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
            // Use custom implementation as faker::markdown::BlockQuoteMultiLine might not be available
            let num_lines: usize = count.fake_with_rng(rng);
            let mut lines = Vec::new();
            for _ in 0..num_lines {
                let line: String = faker::lorem::en::Sentence(3..8).fake_with_rng(rng);
                lines.push(format!("> {}", line));
            }
            Value::String(lines.join("\n"))
        }
        pub fn markdown_code(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
            Value::String(faker::markdown::en::Code(count).fake_with_rng(rng))
        }
        pub fn name_first_name(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::name::en::FirstName().fake_with_rng(rng))
        }
        pub fn name_last_name(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::name::en::LastName().fake_with_rng(rng))
        }
        pub fn name_title(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::name::en::Title().fake_with_rng(rng))
        }
        pub fn name_suffix(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::name::en::Suffix().fake_with_rng(rng))
        }
        pub fn name_name(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::name::en::Name().fake_with_rng(rng))
        }
        pub fn name_name_with_title(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::name::en::NameWithTitle().fake_with_rng(rng))
        }
        pub fn number_digit(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::number::en::Digit().fake_with_rng(rng))
        }
        pub fn number_number_with_format<'a>(&self, rng: &mut StdRng, fmt: &'a str) -> Value {
            Value::String(faker::number::en::NumberWithFormat(fmt).fake_with_rng(rng))
        }
        pub fn phone_number_phone_number(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::phone_number::en::PhoneNumber().fake_with_rng(rng))
        }
        pub fn phone_number_cell_number(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::phone_number::en::CellNumber().fake_with_rng(rng))
        }
        pub fn filesystem_file_path(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::filesystem::en::FilePath().fake_with_rng(rng))
        }
        pub fn filesystem_file_name(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::filesystem::en::FileName().fake_with_rng(rng))
        }
        pub fn filesystem_file_extension(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::filesystem::en::FileExtension().fake_with_rng(rng))
        }
        pub fn filesystem_dir_path(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::filesystem::en::DirPath().fake_with_rng(rng))
        }
        pub fn filesystem_mime_type(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::filesystem::en::MimeType().fake_with_rng(rng))
        }
        pub fn filesystem_semver(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::filesystem::en::Semver().fake_with_rng(rng))
        }
        pub fn filesystem_semver_stable(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::filesystem::en::SemverStable().fake_with_rng(rng))
        }
        pub fn filesystem_semver_unstable(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::filesystem::en::SemverUnstable().fake_with_rng(rng))
        }
        pub fn currency_currency_code(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::currency::en::CurrencyCode().fake_with_rng(rng))
        }
        pub fn currency_currency_name(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::currency::en::CurrencyName().fake_with_rng(rng))
        }
        pub fn currency_currency_symbol(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::currency::en::CurrencySymbol().fake_with_rng(rng))
        }
        pub fn finance_bic(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::finance::en::Bic().fake_with_rng(rng))
        }
        pub fn finance_isin(&self, rng: &mut StdRng) -> Value {
            Value::String(faker::finance::en::Isin().fake_with_rng(rng))
        }
        pub fn administrative_health_insurance_code(&self, rng: &mut StdRng) -> Value {
            // Simple implementation since faker::administrative::en::HealthInsuranceCode might not be supported
            use rand::Rng;
            let part1: u16 = rng.random_range(100..=999);
            let part2: u8 = rng.random_range(10..=99);
            let part3: u16 = rng.random_range(1000..=9999);
            let code: String = format!("{:03}-{:02}-{:04}", part1, part2, part3);
            Value::String(code)
        }
        pub fn automotive_licence_plate(&self, rng: &mut StdRng) -> Value {
            // Simple implementation since faker::automotive::en::LicencePlate might not be supported
            use rand::Rng;
            let letters: String = (0..3).map(|_| {
                let c_idx: usize = rng.random_range(0..26);
                (b'A' + c_idx as u8) as char
            }).collect();
            let numbers: u16 = rng.random_range(0..999);
            Value::String(format!("{}-{:03}", letters, numbers))
        }

        pub fn generate_by_key(&self, pattern: &str, rng: &mut StdRng) -> Value {
            let captures = RE_KEY.captures(pattern).unwrap();
            let key = captures.get(1).unwrap().as_str();
            let arguments = captures.get(2).map(|m| m.as_str());
            match key {
                // Address
                FakeKeys::ADDRESS_CITY_PREFIX => self.address_city_prefix(rng),
                FakeKeys::ADDRESS_CITY_SUFFIX => self.address_city_suffix(rng),
                FakeKeys::ADDRESS_CITY_NAME => self.address_city_name(rng),
                FakeKeys::ADDRESS_COUNTRY_NAME => self.address_country_name(rng),
                FakeKeys::ADDRESS_COUNTRY_CODE => self.address_country_code(rng),
                FakeKeys::ADDRESS_STREET_SUFFIX => self.address_street_suffix(rng),
                FakeKeys::ADDRESS_STREET_NAME => self.address_street_name(rng),
                FakeKeys::ADDRESS_TIME_ZONE => self.address_time_zone(rng),
                FakeKeys::ADDRESS_STATE_NAME => self.address_state_name(rng),
                FakeKeys::ADDRESS_STATE_ABBR => self.address_state_abbr(rng),
                FakeKeys::ADDRESS_SECONDARY_ADDRESS_TYPE => self.address_secondary_address_type(rng),
                FakeKeys::ADDRESS_SECONDARY_ADDRESS => self.address_secondary_address(rng),
                FakeKeys::ADDRESS_ZIP_CODE => self.address_zip_code(rng),
                FakeKeys::ADDRESS_POST_CODE => self.address_post_code(rng),
                FakeKeys::ADDRESS_BUILDING_NUMBER => self.address_building_number(rng),
                FakeKeys::ADDRESS_LATITUDE => self.address_latitude(rng),
                FakeKeys::ADDRESS_LONGITUDE => self.address_longitude(rng),
                FakeKeys::ADDRESS_GEOHASH => {
                    let precision = arguments
                        .map(|args| Self::parse_single_number(args, 5u8))
                        .unwrap_or(5u8);
                    self.address_geohash(rng, precision)
                },

                // Barcode
                FakeKeys::BARCODE_ISBN => self.barcode_isbn(rng),
                FakeKeys::BARCODE_ISBN10 => self.barcode_isbn10(rng),
                FakeKeys::BARCODE_ISBN13 => self.barcode_isbn13(rng),

                // Boolean
                FakeKeys::BOOLEAN_BOOLEAN => {
                    let ratio = arguments
                        .map(|args| Self::parse_single_number(args, 50u8))
                        .unwrap_or(50u8);
                    self.boolean_boolean(rng, ratio)
                },

                // Color
                FakeKeys::COLOR_HEX_COLOR => self.color_hex_color(rng),
                FakeKeys::COLOR_RGB_COLOR => self.color_rgb_color(rng),
                FakeKeys::COLOR_RGBA_COLOR => self.color_rgba_color(rng),
                FakeKeys::COLOR_HSL_COLOR => self.color_hsl_color(rng),
                FakeKeys::COLOR_HSLA_COLOR => self.color_hsla_color(rng),
                FakeKeys::COLOR_COLOR => self.color_color(rng),

                // Chrono
                FakeKeys::CHRONO_TIME => self.chrono_time(rng),
                FakeKeys::CHRONO_DATE => self.chrono_date(rng),
                FakeKeys::CHRONO_DATE_TIME => self.chrono_date_time(rng),
                FakeKeys::CHRONO_DURATION => self.chrono_duration(rng),
                // Chrono with arguments
                FakeKeys::CHRONO_DATE_TIME_BEFORE => {
                    // Parse datetime argument or use current time as default
                    let dt = arguments
                        .and_then(|args| Self::parse_string_arg(args, "").parse::<chrono::DateTime<chrono::Utc>>().ok())
                        .unwrap_or_else(chrono::Utc::now);
                    self.chrono_date_time_before(rng, dt)
                },
                FakeKeys::CHRONO_DATE_TIME_AFTER => {
                    // Parse datetime argument or use current time as default
                    let dt = arguments
                        .and_then(|args| Self::parse_string_arg(args, "").parse::<chrono::DateTime<chrono::Utc>>().ok())
                        .unwrap_or_else(chrono::Utc::now);
                    self.chrono_date_time_after(rng, dt)
                },
                FakeKeys::CHRONO_DATE_TIME_BETWEEN => {
                    // For between, we need two datetime arguments or use defaults
                    if let Some(args) = arguments {
                        let args_content = Self::parse_string_arg(args, "");
                        if args_content.contains(',') {
                            let parts: Vec<&str> = args_content.split(',').map(|s| s.trim()).collect();
                            if parts.len() == 2 {
                                if let (Ok(start), Ok(end)) = (
                                    parts[0].parse::<chrono::DateTime<chrono::Utc>>(),
                                    parts[1].parse::<chrono::DateTime<chrono::Utc>>()
                                ) {
                                    return self.chrono_date_time_between(rng, start, end);
                                }
                            }
                        }
                    }
                    // Default: past year to now
                    let now = chrono::Utc::now();
                    let past = now - chrono::Duration::days(365);
                    self.chrono_date_time_between(rng, past, now)
                },

                // Time
                FakeKeys::TIME_TIME => self.time_time(rng),
                FakeKeys::TIME_DATE => self.time_date(rng),
                FakeKeys::TIME_DATE_TIME => self.time_date_time(rng),
                FakeKeys::TIME_DURATION => self.time_duration(rng),
                // Time with arguments
                FakeKeys::TIME_DATE_TIME_BEFORE => {
                    // For now, use current time as default since time::OffsetDateTime parsing is complex
                    let dt = time::OffsetDateTime::now_utc();
                    self.time_date_time_before(rng, dt)
                },
                FakeKeys::TIME_DATE_TIME_AFTER => {
                    // For now, use current time as default since time::OffsetDateTime parsing is complex
                    let dt = time::OffsetDateTime::now_utc();
                    self.time_date_time_after(rng, dt)
                },
                FakeKeys::TIME_DATE_TIME_BETWEEN => {
                    // Default: past year to now
                    let now = time::OffsetDateTime::now_utc();
                    let past = now - time::Duration::days(365);
                    self.time_date_time_between(rng, past, now)
                },

                // Credit Card
                FakeKeys::CREDITCARD_CREDIT_CARD_NUMBER => self.creditcard_credit_card_number(rng),

                // Company
                FakeKeys::COMPANY_COMPANY_SUFFIX => self.company_company_suffix(rng),
                FakeKeys::COMPANY_COMPANY_NAME => self.company_company_name(rng),
                FakeKeys::COMPANY_BUZZWORD => self.company_buzzword(rng),
                FakeKeys::COMPANY_BUZZWORD_MIDDLE => self.company_buzzword_middle(rng),
                FakeKeys::COMPANY_BUZZWORD_TAIL => self.company_buzzword_tail(rng),
                FakeKeys::COMPANY_CATCH_PHRASE => self.company_catch_phrase(rng),
                FakeKeys::COMPANY_BS_VERB => self.company_bs_verb(rng),
                FakeKeys::COMPANY_BS_ADJ => self.company_bs_adj(rng),
                FakeKeys::COMPANY_BS_NOUN => self.company_bs_noun(rng),
                FakeKeys::COMPANY_BS => self.company_bs(rng),
                FakeKeys::COMPANY_PROFESSION => self.company_profession(rng),
                FakeKeys::COMPANY_INDUSTRY => self.company_industry(rng),

                // HTTP
                FakeKeys::HTTP_RFC_STATUS_CODE => self.http_rfc_status_code(rng),
                FakeKeys::HTTP_VALID_STATUS_CODE => self.http_valid_status_code(rng),

                // Internet
                FakeKeys::INTERNET_FREE_EMAIL_PROVIDER => self.internet_free_email_provider(rng),
                FakeKeys::INTERNET_DOMAIN_SUFFIX => self.internet_domain_suffix(rng),
                FakeKeys::INTERNET_FREE_EMAIL => self.internet_free_email(rng),
                FakeKeys::INTERNET_SAFE_EMAIL => self.internet_safe_email(rng),
                FakeKeys::INTERNET_USERNAME => self.internet_username(rng),
                FakeKeys::INTERNET_PASSWORD => {
                    let range = arguments
                        .map(|args| Self::parse_range(args, 8..16))
                        .unwrap_or(8..16);
                    self.internet_password(rng, range)
                },
                FakeKeys::INTERNET_I_PV4 => self.internet_i_pv4(rng),
                FakeKeys::INTERNET_I_PV6 => self.internet_i_pv6(rng),
                FakeKeys::INTERNET_IP => self.internet_ip(rng),
                FakeKeys::INTERNET_MAC_ADDRESS => self.internet_mac_address(rng),
                FakeKeys::INTERNET_USER_AGENT => self.internet_user_agent(rng),

                // Job
                FakeKeys::JOB_SENIORITY => self.job_seniority(rng),
                FakeKeys::JOB_FIELD => self.job_field(rng),
                FakeKeys::JOB_POSITION => self.job_position(rng),
                FakeKeys::JOB_TITLE => self.job_title(rng),

                // Lorem
                FakeKeys::LOREM_WORD => self.lorem_word(rng),
                FakeKeys::LOREM_WORDS => {
                    let count = arguments
                        .map(|args| Self::parse_range(args, 3..8))
                        .unwrap_or(3..8);
                    self.lorem_words(rng, count)
                },
                FakeKeys::LOREM_SENTENCE => {
                    let count = arguments
                        .map(|args| Self::parse_range(args, 4..18))
                        .unwrap_or(4..18);
                    self.lorem_sentence(rng, count)
                },
                FakeKeys::LOREM_SENTENCES => {
                    let count = arguments
                        .map(|args| Self::parse_range(args, 2..6))
                        .unwrap_or(2..6);
                    self.lorem_sentences(rng, count)
                },
                FakeKeys::LOREM_PARAGRAPH => {
                    let count = arguments
                        .map(|args| Self::parse_range(args, 3..10))
                        .unwrap_or(3..10);
                    self.lorem_paragraph(rng, count)
                },
                FakeKeys::LOREM_PARAGRAPHS => {
                    let count = arguments
                        .map(|args| Self::parse_range(args, 2..5))
                        .unwrap_or(2..5);
                    self.lorem_paragraphs(rng, count)
                },

                // Markdown
                FakeKeys::MARKDOWN_ITALIC_WORD => self.markdown_italic_word(rng),
                FakeKeys::MARKDOWN_BOLD_WORD => self.markdown_bold_word(rng),
                FakeKeys::MARKDOWN_LINK => self.markdown_link(rng),
                FakeKeys::MARKDOWN_BULLET_POINTS => {
                    let count = arguments
                        .map(|args| Self::parse_range(args, 3..8))
                        .unwrap_or(3..8);
                    self.markdown_bullet_points(rng, count)
                },
                FakeKeys::MARKDOWN_LIST_ITEMS => {
                    let count = arguments
                        .map(|args| Self::parse_range(args, 3..8))
                        .unwrap_or(3..8);
                    self.markdown_list_items(rng, count)
                },
                FakeKeys::MARKDOWN_BLOCK_QUOTE_SINGLE_LINE => {
                    let count = arguments
                        .map(|args| Self::parse_range(args, 4..18))
                        .unwrap_or(4..18);
                    self.markdown_block_quote_single_line(rng, count)
                },
                FakeKeys::MARKDOWN_BLOCK_QUOTE_MULTI_LINE => {
                    let count = arguments
                        .map(|args| Self::parse_range(args, 2..6))
                        .unwrap_or(2..6);
                    self.markdown_block_quote_multi_line(rng, count)
                },
                FakeKeys::MARKDOWN_CODE => {
                    let count = arguments
                        .map(|args| Self::parse_range(args, 3..8))
                        .unwrap_or(3..8);
                    self.markdown_code(rng, count)
                },

                // Name
                FakeKeys::NAME_FIRST_NAME => self.name_first_name(rng),
                FakeKeys::NAME_LAST_NAME => self.name_last_name(rng),
                FakeKeys::NAME_TITLE => self.name_title(rng),
                FakeKeys::NAME_SUFFIX => self.name_suffix(rng),
                FakeKeys::NAME_NAME => self.name_name(rng),
                FakeKeys::NAME_NAME_WITH_TITLE => self.name_name_with_title(rng),

                // Number
                FakeKeys::NUMBER_DIGIT => self.number_digit(rng),
                FakeKeys::NUMBER_NUMBER_WITH_FORMAT => {
                    let format = arguments
                        .map(|args| Self::parse_string_arg(args, "###-###-####"))
                        .unwrap_or("###-###-####");
                    self.number_number_with_format(rng, format)
                },

                // Phone Number
                FakeKeys::PHONE_NUMBER_PHONE_NUMBER => self.phone_number_phone_number(rng),
                FakeKeys::PHONE_NUMBER_CELL_NUMBER => self.phone_number_cell_number(rng),

                // Filesystem
                FakeKeys::FILESYSTEM_FILE_PATH => self.filesystem_file_path(rng),
                FakeKeys::FILESYSTEM_FILE_NAME => self.filesystem_file_name(rng),
                FakeKeys::FILESYSTEM_FILE_EXTENSION => self.filesystem_file_extension(rng),
                FakeKeys::FILESYSTEM_DIR_PATH => self.filesystem_dir_path(rng),
                FakeKeys::FILESYSTEM_MIME_TYPE => self.filesystem_mime_type(rng),
                FakeKeys::FILESYSTEM_SEMVER => self.filesystem_semver(rng),
                FakeKeys::FILESYSTEM_SEMVER_STABLE => self.filesystem_semver_stable(rng),
                FakeKeys::FILESYSTEM_SEMVER_UNSTABLE => self.filesystem_semver_unstable(rng),

                // Currency
                FakeKeys::CURRENCY_CURRENCY_CODE => self.currency_currency_code(rng),
                FakeKeys::CURRENCY_CURRENCY_NAME => self.currency_currency_name(rng),
                FakeKeys::CURRENCY_CURRENCY_SYMBOL => self.currency_currency_symbol(rng),

                // Finance
                FakeKeys::FINANCE_BIC => self.finance_bic(rng),
                FakeKeys::FINANCE_ISIN => self.finance_isin(rng),

                // Administrative
                FakeKeys::ADMINISTRATIVE_HEALTH_INSURANCE_CODE => self.administrative_health_insurance_code(rng),

                // Automotive
                FakeKeys::AUTOMOTIVE_LICENCE_PLATE => self.automotive_licence_plate(rng),

                _ => Value::String(pattern.to_string()),
            }
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use serde_json::Value;

    // Mock implementation of FakeLocaleGenerator for testing
    struct MockLocaleGenerator;
    impl FakeLocaleGenerator for MockLocaleGenerator {}

    fn create_test_generator() -> FakeGenerator {
        FakeGenerator::new(Box::new(MockLocaleGenerator))
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

        let result = generator.generate_by_key("chrono.dateTimeBetween", &mut rng);
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

