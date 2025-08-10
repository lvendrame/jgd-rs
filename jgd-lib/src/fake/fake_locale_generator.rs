use fake::{faker, Fake};
use rand::rngs::StdRng;
use serde_json::Value;

pub trait FakeLocaleGenerator {

    fn address_city_prefix(&self, rng: &mut StdRng) -> Value;
    fn address_city_suffix(&self, rng: &mut StdRng) -> Value;
    fn address_city_name(&self, rng: &mut StdRng) -> Value;
    fn address_country_name(&self, rng: &mut StdRng) -> Value;
    fn address_country_code(&self, rng: &mut StdRng) -> Value;
    fn address_street_suffix(&self, rng: &mut StdRng) -> Value;
    fn address_street_name(&self, rng: &mut StdRng) -> Value;
    fn address_time_zone(&self, rng: &mut StdRng) -> Value;
    fn address_state_name(&self, rng: &mut StdRng) -> Value;
    fn address_state_abbr(&self, rng: &mut StdRng) -> Value;
    fn address_secondary_address_type(&self, rng: &mut StdRng) -> Value;
    fn address_secondary_address(&self, rng: &mut StdRng) -> Value;
    fn address_zip_code(&self, rng: &mut StdRng) -> Value;
    fn address_post_code(&self, rng: &mut StdRng) -> Value;
    fn address_building_number(&self, rng: &mut StdRng) -> Value;
    fn address_latitude(&self, rng: &mut StdRng) -> Value;
    fn address_longitude(&self, rng: &mut StdRng) -> Value;
    fn address_geohash(&self, rng: &mut StdRng, precision: u8) -> Value;
    fn barcode_isbn(&self, rng: &mut StdRng) -> Value;
    fn barcode_isbn10(&self, rng: &mut StdRng) -> Value;
    fn barcode_isbn13(&self, rng: &mut StdRng) -> Value;
    fn boolean_boolean(&self, rng: &mut StdRng, ratio: u8) -> Value;
    fn color_hex_color(&self, rng: &mut StdRng) -> Value;
    fn color_rgb_color(&self, rng: &mut StdRng) -> Value;
    fn color_rgba_color(&self, rng: &mut StdRng) -> Value;
    fn color_hsl_color(&self, rng: &mut StdRng) -> Value;
    fn color_hsla_color(&self, rng: &mut StdRng) -> Value;
    fn color_color(&self, rng: &mut StdRng) -> Value;
    fn chrono_time(&self, rng: &mut StdRng) -> Value;
    fn chrono_date(&self, rng: &mut StdRng) -> Value;
    fn chrono_date_time(&self, rng: &mut StdRng) -> Value;
    fn chrono_duration(&self, rng: &mut StdRng) -> Value;
    fn chrono_date_time_before(&self, rng: &mut StdRng, dt: chrono::DateTime<chrono::Utc>) -> Value;
    fn chrono_date_time_after(&self, rng: &mut StdRng, dt: chrono::DateTime<chrono::Utc>) -> Value;
    fn chrono_date_time_between(&self, rng: &mut StdRng, start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Value;
    fn time_time(&self, rng: &mut StdRng) -> Value;
    fn time_date(&self, rng: &mut StdRng) -> Value;
    fn time_date_time(&self, rng: &mut StdRng) -> Value;
    fn time_duration(&self, rng: &mut StdRng) -> Value;
    fn time_date_time_before(&self, rng: &mut StdRng, dt: time::OffsetDateTime) -> Value;
    fn time_date_time_after(&self, rng: &mut StdRng, dt: time::OffsetDateTime) -> Value;
    fn time_date_time_between(&self, rng: &mut StdRng, start: time::OffsetDateTime, end: time::OffsetDateTime) -> Value;
    fn creditcard_credit_card_number(&self, rng: &mut StdRng) -> Value;
    fn company_company_suffix(&self, rng: &mut StdRng) -> Value;
    fn company_company_name(&self, rng: &mut StdRng) -> Value;
    fn company_buzzword(&self, rng: &mut StdRng) -> Value;
    fn company_buzzword_middle(&self, rng: &mut StdRng) -> Value;
    fn company_buzzword_tail(&self, rng: &mut StdRng) -> Value;
    fn company_catch_phrase(&self, rng: &mut StdRng) -> Value;
    fn company_bs_verb(&self, rng: &mut StdRng) -> Value;
    fn company_bs_adj(&self, rng: &mut StdRng) -> Value;
    fn company_bs_noun(&self, rng: &mut StdRng) -> Value;
    fn company_bs(&self, rng: &mut StdRng) -> Value;
    fn company_profession(&self, rng: &mut StdRng) -> Value;
    fn company_industry(&self, rng: &mut StdRng) -> Value;
    fn http_rfc_status_code(&self, rng: &mut StdRng) -> Value;
    fn http_valid_status_code(&self, rng: &mut StdRng) -> Value;
    fn internet_free_email_provider(&self, rng: &mut StdRng) -> Value;
    fn internet_domain_suffix(&self, rng: &mut StdRng) -> Value;
    fn internet_free_email(&self, rng: &mut StdRng) -> Value;
    fn internet_safe_email(&self, rng: &mut StdRng) -> Value;
    fn internet_username(&self, rng: &mut StdRng) -> Value;
    fn internet_password(&self, rng: &mut StdRng, len_range: std::ops::Range<usize>) -> Value;
    fn internet_i_pv4(&self, rng: &mut StdRng) -> Value;
    fn internet_i_pv6(&self, rng: &mut StdRng) -> Value;
    fn internet_ip(&self, rng: &mut StdRng) -> Value;
    fn internet_mac_address(&self, rng: &mut StdRng) -> Value;
    fn internet_user_agent(&self, rng: &mut StdRng) -> Value;
    fn job_seniority(&self, rng: &mut StdRng) -> Value;
    fn job_field(&self, rng: &mut StdRng) -> Value;
    fn job_position(&self, rng: &mut StdRng) -> Value;
    fn job_title(&self, rng: &mut StdRng) -> Value;
    fn lorem_word(&self, rng: &mut StdRng) -> Value;
    fn lorem_words(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value;
    fn lorem_sentence(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value;
    fn lorem_sentences(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value;
    fn lorem_paragraph(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value;
    fn lorem_paragraphs(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value;
    fn markdown_italic_word(&self, rng: &mut StdRng) -> Value;
    fn markdown_bold_word(&self, rng: &mut StdRng) -> Value;
    fn markdown_link(&self, rng: &mut StdRng) -> Value;
    fn markdown_bullet_points(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value;
    fn markdown_list_items(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value;
    fn markdown_block_quote_single_line(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value;
    fn markdown_block_quote_multi_line(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value;
    fn markdown_code(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value;
    fn name_first_name(&self, rng: &mut StdRng) -> Value;
    fn name_last_name(&self, rng: &mut StdRng) -> Value;
    fn name_title(&self, rng: &mut StdRng) -> Value;
    fn name_suffix(&self, rng: &mut StdRng) -> Value;
    fn name_name(&self, rng: &mut StdRng) -> Value;
    fn name_name_with_title(&self, rng: &mut StdRng) -> Value;
    fn number_digit(&self, rng: &mut StdRng) -> Value;
    fn number_number_with_format<'a>(&self, rng: &mut StdRng, fmt: &'a str) -> Value;
    fn phone_number_phone_number(&self, rng: &mut StdRng) -> Value;
    fn phone_number_cell_number(&self, rng: &mut StdRng) -> Value;
    fn filesystem_file_path(&self, rng: &mut StdRng) -> Value;
    fn filesystem_file_name(&self, rng: &mut StdRng) -> Value;
    fn filesystem_file_extension(&self, rng: &mut StdRng) -> Value;
    fn filesystem_dir_path(&self, rng: &mut StdRng) -> Value;
    fn filesystem_mime_type(&self, rng: &mut StdRng) -> Value;
    fn filesystem_semver(&self, rng: &mut StdRng) -> Value;
    fn filesystem_semver_stable(&self, rng: &mut StdRng) -> Value;
    fn filesystem_semver_unstable(&self, rng: &mut StdRng) -> Value;
    fn currency_currency_code(&self, rng: &mut StdRng) -> Value;
    fn currency_currency_name(&self, rng: &mut StdRng) -> Value;
    fn currency_currency_symbol(&self, rng: &mut StdRng) -> Value;
    fn finance_bic(&self, rng: &mut StdRng) -> Value;
    fn finance_isin(&self, rng: &mut StdRng) -> Value;
    fn administrative_health_insurance_code(&self, rng: &mut StdRng) -> Value;
    fn automotive_licence_plate(&self, rng: &mut StdRng) -> Value;
}

macro_rules! locale_generator {
    ($locale:ident, $struct_name:ident) => {
        pub struct $struct_name;

        impl FakeLocaleGenerator for $struct_name {
            fn address_city_prefix(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::CityPrefix().fake_with_rng(rng))
            }

            fn address_city_suffix(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::CitySuffix().fake_with_rng(rng))
            }
            fn address_city_name(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::CityName().fake_with_rng(rng))
            }
            fn address_country_name(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::CountryName().fake_with_rng(rng))
            }
            fn address_country_code(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::CountryCode().fake_with_rng(rng))
            }
            fn address_street_suffix(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::StreetSuffix().fake_with_rng(rng))
            }
            fn address_street_name(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::StreetName().fake_with_rng(rng))
            }
            fn address_time_zone(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::TimeZone().fake_with_rng(rng))
            }
            fn address_state_name(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::StateName().fake_with_rng(rng))
            }
            fn address_state_abbr(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::StateAbbr().fake_with_rng(rng))
            }
            fn address_secondary_address_type(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::SecondaryAddressType().fake_with_rng(rng))
            }
            fn address_secondary_address(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::SecondaryAddress().fake_with_rng(rng))
            }
            fn address_zip_code(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::ZipCode().fake_with_rng(rng))
            }
            fn address_post_code(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::PostCode().fake_with_rng(rng))
            }
            fn address_building_number(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::BuildingNumber().fake_with_rng(rng))
            }
            fn address_latitude(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::Latitude().fake_with_rng(rng))
            }
            fn address_longitude(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::address::$locale::Longitude().fake_with_rng(rng))
            }
            fn address_geohash(&self, rng: &mut StdRng, precision: u8) -> Value {
                Value::String(faker::address::$locale::Geohash(precision).fake_with_rng(rng))
            }
            fn barcode_isbn(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::barcode::$locale::Isbn().fake_with_rng(rng))
            }
            fn barcode_isbn10(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::barcode::$locale::Isbn10().fake_with_rng(rng))
            }
            fn barcode_isbn13(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::barcode::$locale::Isbn13().fake_with_rng(rng))
            }
            fn boolean_boolean(&self, rng: &mut StdRng, ratio: u8) -> Value {
                Value::Bool(faker::boolean::$locale::Boolean(ratio).fake_with_rng(rng))
            }
            fn color_hex_color(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::color::$locale::HexColor().fake_with_rng(rng))
            }
            fn color_rgb_color(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::color::$locale::RgbColor().fake_with_rng(rng))
            }
            fn color_rgba_color(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::color::$locale::RgbaColor().fake_with_rng(rng))
            }
            fn color_hsl_color(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::color::$locale::HslColor().fake_with_rng(rng))
            }
            fn color_hsla_color(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::color::$locale::HslaColor().fake_with_rng(rng))
            }
            fn color_color(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::color::$locale::Color().fake_with_rng(rng))
            }
            fn chrono_time(&self, rng: &mut StdRng) -> Value {
                let time: chrono::NaiveTime = faker::chrono::$locale::Time().fake_with_rng(rng);
                Value::String(time.to_string())
            }
            fn chrono_date(&self, rng: &mut StdRng) -> Value {
                let date: chrono::NaiveDate = faker::chrono::$locale::Date().fake_with_rng(rng);
                Value::String(date.to_string())
            }
            fn chrono_date_time(&self, rng: &mut StdRng) -> Value {
                let dt: chrono::DateTime<chrono::Utc> = faker::chrono::$locale::DateTime().fake_with_rng(rng);
                Value::String(dt.to_rfc3339())
            }
            fn chrono_duration(&self, rng: &mut StdRng) -> Value {
                let duration: chrono::Duration = faker::chrono::$locale::Duration().fake_with_rng(rng);
                Value::String(duration.to_string())
            }
            fn chrono_date_time_before(&self, rng: &mut StdRng, dt: chrono::DateTime<chrono::Utc>) -> Value {
                let before: chrono::DateTime<chrono::Utc> = faker::chrono::$locale::DateTimeBefore(dt).fake_with_rng(rng);
                Value::String(before.to_rfc3339())
            }
            fn chrono_date_time_after(&self, rng: &mut StdRng, dt: chrono::DateTime<chrono::Utc>) -> Value {
                let after: chrono::DateTime<chrono::Utc> = faker::chrono::$locale::DateTimeAfter(dt).fake_with_rng(rng);
                Value::String(after.to_rfc3339())
            }
            fn chrono_date_time_between(&self, rng: &mut StdRng, start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Value {
                let between: chrono::DateTime<chrono::Utc> = faker::chrono::$locale::DateTimeBetween(start, end).fake_with_rng(rng);
                Value::String(between.to_rfc3339())
            }
            fn time_time(&self, rng: &mut StdRng) -> Value {
                let time: time::Time = faker::time::$locale::Time().fake_with_rng(rng);
                Value::String(time.to_string())
            }
            fn time_date(&self, rng: &mut StdRng) -> Value {
                let date: time::Date = faker::time::$locale::Date().fake_with_rng(rng);
                Value::String(date.to_string())
            }
            fn time_date_time(&self, rng: &mut StdRng) -> Value {
                let dt: time::OffsetDateTime = faker::time::$locale::DateTime().fake_with_rng(rng);
                Value::String(dt.to_string())
            }
            fn time_duration(&self, rng: &mut StdRng) -> Value {
                let duration: time::Duration = faker::time::$locale::Duration().fake_with_rng(rng);
                Value::String(duration.to_string())
            }
            fn time_date_time_before(&self, rng: &mut StdRng, dt: time::OffsetDateTime) -> Value {
                let before: time::OffsetDateTime = faker::time::$locale::DateTimeBefore(dt).fake_with_rng(rng);
                Value::String(before.to_string())
            }
            fn time_date_time_after(&self, rng: &mut StdRng, dt: time::OffsetDateTime) -> Value {
                let after: time::OffsetDateTime = faker::time::$locale::DateTimeAfter(dt).fake_with_rng(rng);
                Value::String(after.to_string())
            }
            fn time_date_time_between(&self, rng: &mut StdRng, start: time::OffsetDateTime, end: time::OffsetDateTime) -> Value {
                let between: time::OffsetDateTime = faker::time::$locale::DateTimeBetween(start, end).fake_with_rng(rng);
                Value::String(between.to_string())
            }
            fn creditcard_credit_card_number(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::creditcard::$locale::CreditCardNumber().fake_with_rng(rng))
            }
            fn company_company_suffix(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::company::$locale::CompanySuffix().fake_with_rng(rng))
            }
            fn company_company_name(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::company::$locale::CompanyName().fake_with_rng(rng))
            }
            fn company_buzzword(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::company::$locale::Buzzword().fake_with_rng(rng))
            }
            fn company_buzzword_middle(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::company::$locale::BuzzwordMiddle().fake_with_rng(rng))
            }
            fn company_buzzword_tail(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::company::$locale::BuzzwordTail().fake_with_rng(rng))
            }
            fn company_catch_phrase(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::company::$locale::CatchPhrase().fake_with_rng(rng))
            }
            fn company_bs_verb(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::company::$locale::BsVerb().fake_with_rng(rng))
            }
            fn company_bs_adj(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::company::$locale::BsAdj().fake_with_rng(rng))
            }
            fn company_bs_noun(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::company::$locale::BsNoun().fake_with_rng(rng))
            }
            fn company_bs(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::company::$locale::Bs().fake_with_rng(rng))
            }
            fn company_profession(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::company::$locale::Profession().fake_with_rng(rng))
            }
            fn company_industry(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::company::$locale::Industry().fake_with_rng(rng))
            }
            fn http_rfc_status_code(&self, rng: &mut StdRng) -> Value {
                let codes = [200, 201, 204, 301, 302, 400, 401, 403, 404, 500, 502, 503];
                let idx: usize = (0..codes.len()).fake_with_rng(rng);
                Value::Number(codes[idx].into())
            }
            fn http_valid_status_code(&self, rng: &mut StdRng) -> Value {
                let codes = [200, 201, 204, 301, 302, 400, 401, 403, 404];
                let idx: usize = (0..codes.len()).fake_with_rng(rng);
                Value::Number(codes[idx].into())
            }
            fn internet_free_email_provider(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::internet::$locale::FreeEmailProvider().fake_with_rng(rng))
            }
            fn internet_domain_suffix(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::internet::$locale::DomainSuffix().fake_with_rng(rng))
            }
            fn internet_free_email(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::internet::$locale::FreeEmail().fake_with_rng(rng))
            }
            fn internet_safe_email(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::internet::$locale::SafeEmail().fake_with_rng(rng))
            }
            fn internet_username(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::internet::$locale::Username().fake_with_rng(rng))
            }
            fn internet_password(&self, rng: &mut StdRng, len_range: std::ops::Range<usize>) -> Value {
                Value::String(faker::internet::$locale::Password(len_range).fake_with_rng(rng))
            }
            fn internet_i_pv4(&self, rng: &mut StdRng) -> Value {
                let ipv4: std::net::Ipv4Addr = faker::internet::$locale::IPv4().fake_with_rng(rng);
                Value::String(ipv4.to_string())
            }
            fn internet_i_pv6(&self, rng: &mut StdRng) -> Value {
                let ipv6: std::net::Ipv6Addr = faker::internet::$locale::IPv6().fake_with_rng(rng);
                Value::String(ipv6.to_string())
            }
            fn internet_ip(&self, rng: &mut StdRng) -> Value {
                let ip: std::net::IpAddr = faker::internet::$locale::IP().fake_with_rng(rng);
                Value::String(ip.to_string())
            }
            fn internet_mac_address(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::internet::$locale::MACAddress().fake_with_rng(rng))
            }
            fn internet_user_agent(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::internet::$locale::UserAgent().fake_with_rng(rng))
            }
            fn job_seniority(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::job::$locale::Seniority().fake_with_rng(rng))
            }
            fn job_field(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::job::$locale::Field().fake_with_rng(rng))
            }
            fn job_position(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::job::$locale::Position().fake_with_rng(rng))
            }
            fn job_title(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::job::$locale::Title().fake_with_rng(rng))
            }
            fn lorem_word(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::lorem::$locale::Word().fake_with_rng(rng))
            }
            fn lorem_words(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
                let words: Vec<String> = faker::lorem::$locale::Words(count).fake_with_rng(rng);
                Value::String(words.join(" "))
            }
            fn lorem_sentence(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
                Value::String(faker::lorem::$locale::Sentence(count).fake_with_rng(rng))
            }
            fn lorem_sentences(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
                let sentences: Vec<String> = faker::lorem::$locale::Sentences(count).fake_with_rng(rng);
                Value::String(sentences.join(" "))
            }
            fn lorem_paragraph(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
                Value::String(faker::lorem::$locale::Paragraph(count).fake_with_rng(rng))
            }
            fn lorem_paragraphs(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
                let paragraphs: Vec<String> = faker::lorem::$locale::Paragraphs(count).fake_with_rng(rng);
                Value::String(paragraphs.join("\n\n"))
            }
            fn markdown_italic_word(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::markdown::$locale::ItalicWord().fake_with_rng(rng))
            }
            fn markdown_bold_word(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::markdown::$locale::BoldWord().fake_with_rng(rng))
            }
            fn markdown_link(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::markdown::$locale::Link().fake_with_rng(rng))
            }
            fn markdown_bullet_points(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
                // Use custom implementation as faker::markdown::BulletPoints might not be available
                let num_items: usize = count.fake_with_rng(rng);
                let mut items = Vec::new();
                for _ in 0..num_items {
                    let item: String = faker::lorem::$locale::Sentence(3..8).fake_with_rng(rng);
                    items.push(format!("• {}", item));
                }
                Value::String(items.join("\n"))
            }
            fn markdown_list_items(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
                // Use custom implementation as faker::markdown::ListItems might not be available
                let num_items: usize = count.fake_with_rng(rng);
                let mut items = Vec::new();
                for i in 1..=num_items {
                    let item: String = faker::lorem::$locale::Sentence(3..8).fake_with_rng(rng);
                    items.push(format!("{}. {}", i, item));
                }
                Value::String(items.join("\n"))
            }
            fn markdown_block_quote_single_line(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
                Value::String(faker::markdown::$locale::BlockQuoteSingleLine(count).fake_with_rng(rng))
            }
            fn markdown_block_quote_multi_line(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
                // Use custom implementation as faker::markdown::BlockQuoteMultiLine might not be available
                let num_lines: usize = count.fake_with_rng(rng);
                let mut lines = Vec::new();
                for _ in 0..num_lines {
                    let line: String = faker::lorem::$locale::Sentence(3..8).fake_with_rng(rng);
                    lines.push(format!("> {}", line));
                }
                Value::String(lines.join("\n"))
            }
            fn markdown_code(&self, rng: &mut StdRng, count: std::ops::Range<usize>) -> Value {
                Value::String(faker::markdown::$locale::Code(count).fake_with_rng(rng))
            }
            fn name_first_name(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::name::$locale::FirstName().fake_with_rng(rng))
            }
            fn name_last_name(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::name::$locale::LastName().fake_with_rng(rng))
            }
            fn name_title(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::name::$locale::Title().fake_with_rng(rng))
            }
            fn name_suffix(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::name::$locale::Suffix().fake_with_rng(rng))
            }
            fn name_name(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::name::$locale::Name().fake_with_rng(rng))
            }
            fn name_name_with_title(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::name::$locale::NameWithTitle().fake_with_rng(rng))
            }
            fn number_digit(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::number::$locale::Digit().fake_with_rng(rng))
            }
            fn number_number_with_format<'a>(&self, rng: &mut StdRng, fmt: &'a str) -> Value {
                Value::String(faker::number::$locale::NumberWithFormat(fmt).fake_with_rng(rng))
            }
            fn phone_number_phone_number(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::phone_number::$locale::PhoneNumber().fake_with_rng(rng))
            }
            fn phone_number_cell_number(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::phone_number::$locale::CellNumber().fake_with_rng(rng))
            }
            fn filesystem_file_path(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::filesystem::$locale::FilePath().fake_with_rng(rng))
            }
            fn filesystem_file_name(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::filesystem::$locale::FileName().fake_with_rng(rng))
            }
            fn filesystem_file_extension(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::filesystem::$locale::FileExtension().fake_with_rng(rng))
            }
            fn filesystem_dir_path(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::filesystem::$locale::DirPath().fake_with_rng(rng))
            }
            fn filesystem_mime_type(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::filesystem::$locale::MimeType().fake_with_rng(rng))
            }
            fn filesystem_semver(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::filesystem::$locale::Semver().fake_with_rng(rng))
            }
            fn filesystem_semver_stable(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::filesystem::$locale::SemverStable().fake_with_rng(rng))
            }
            fn filesystem_semver_unstable(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::filesystem::$locale::SemverUnstable().fake_with_rng(rng))
            }
            fn currency_currency_code(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::currency::$locale::CurrencyCode().fake_with_rng(rng))
            }
            fn currency_currency_name(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::currency::$locale::CurrencyName().fake_with_rng(rng))
            }
            fn currency_currency_symbol(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::currency::$locale::CurrencySymbol().fake_with_rng(rng))
            }
            fn finance_bic(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::finance::$locale::Bic().fake_with_rng(rng))
            }
            fn finance_isin(&self, rng: &mut StdRng) -> Value {
                Value::String(faker::finance::$locale::Isin().fake_with_rng(rng))
            }
            fn administrative_health_insurance_code(&self, rng: &mut StdRng) -> Value {
                // Simple implementation since faker::administrative::$locale::HealthInsuranceCode might not be supported
                use rand::Rng;
                let part1: u16 = rng.random_range(100..=999);
                let part2: u8 = rng.random_range(10..=99);
                let part3: u16 = rng.random_range(1000..=9999);
                let code: String = format!("{:03}-{:02}-{:04}", part1, part2, part3);
                Value::String(code)
            }
            fn automotive_licence_plate(&self, rng: &mut StdRng) -> Value {
                // Simple implementation since faker::automotive::$locale::LicencePlate might not be supported
                use rand::Rng;
                let letters: String = (0..3).map(|_| {
                    let c_idx: usize = rng.random_range(0..26);
                    (b'A' + c_idx as u8) as char
                }).collect();
                let numbers: u16 = rng.random_range(0..999);
                Value::String(format!("{}-{:03}", letters, numbers))
            }
        }
    };
}

locale_generator!(en, FakeGeneratorEn);
locale_generator!(fr_fr, FakeGeneratorFrFr);
locale_generator!(it_it, FakeGeneratorItIt);
locale_generator!(ja_jp, FakeGeneratorJaJp);
locale_generator!(de_de, FakeGeneratorDeDe);
locale_generator!(pt_br, FakeGeneratorPtBr);
locale_generator!(ar_sa, FakeGeneratorArSa);
locale_generator!(cy_gb, FakeGeneratorCyGb);

// faker::finance::pt_pt;
// faker::finance::zh_cn;
// faker::finance::zh_tw