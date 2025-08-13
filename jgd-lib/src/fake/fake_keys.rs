use std::collections::HashSet;

pub struct FakeKeys {
    pub sets: HashSet<&'static str>,
}

impl Default for FakeKeys {
    fn default() -> Self {
        Self::new()
    }
}

impl FakeKeys {
    pub const ADDRESS_CITY_PREFIX: &'static str = "address.cityPrefix";
    pub const ADDRESS_CITY_SUFFIX: &'static str = "address.citySuffix";
    pub const ADDRESS_CITY_NAME: &'static str = "address.cityName";
    pub const ADDRESS_COUNTRY_NAME: &'static str = "address.countryName";
    pub const ADDRESS_COUNTRY_CODE: &'static str = "address.countryCode";
    pub const ADDRESS_STREET_SUFFIX: &'static str = "address.streetSuffix";
    pub const ADDRESS_STREET_NAME: &'static str = "address.streetName";
    pub const ADDRESS_TIME_ZONE: &'static str = "address.timeZone";
    pub const ADDRESS_STATE_NAME: &'static str = "address.stateName";
    pub const ADDRESS_STATE_ABBR: &'static str = "address.stateAbbr";
    pub const ADDRESS_SECONDARY_ADDRESS_TYPE: &'static str = "address.secondaryAddressType";
    pub const ADDRESS_SECONDARY_ADDRESS: &'static str = "address.secondaryAddress";
    pub const ADDRESS_ZIP_CODE: &'static str = "address.zipCode";
    pub const ADDRESS_POST_CODE: &'static str = "address.postCode";
    pub const ADDRESS_BUILDING_NUMBER: &'static str = "address.buildingNumber";
    pub const ADDRESS_LATITUDE: &'static str = "address.latitude";
    pub const ADDRESS_LONGITUDE: &'static str = "address.longitude";
    pub const ADDRESS_GEOHASH: &'static str = "address.geohash";
    pub const BARCODE_ISBN: &'static str = "barcode.isbn";
    pub const BARCODE_ISBN10: &'static str = "barcode.isbn10";
    pub const BARCODE_ISBN13: &'static str = "barcode.isbn13";
    pub const BOOLEAN_BOOLEAN: &'static str = "boolean.boolean";
    pub const COLOR_HEX_COLOR: &'static str = "color.hexColor";
    pub const COLOR_RGB_COLOR: &'static str = "color.rgbColor";
    pub const COLOR_RGBA_COLOR: &'static str = "color.rgbaColor";
    pub const COLOR_HSL_COLOR: &'static str = "color.hslColor";
    pub const COLOR_HSLA_COLOR: &'static str = "color.hslaColor";
    pub const COLOR_COLOR: &'static str = "color.color";
    pub const CHRONO_TIME: &'static str = "chrono.time";
    pub const CHRONO_DATE: &'static str = "chrono.date";
    pub const CHRONO_DATE_TIME: &'static str = "chrono.dateTime";
    pub const CHRONO_DURATION: &'static str = "chrono.duration";
    pub const CHRONO_DATE_TIME_BEFORE: &'static str = "chrono.dateTimeBefore";
    pub const CHRONO_DATE_TIME_AFTER: &'static str = "chrono.dateTimeAfter";
    pub const CHRONO_DATE_TIME_BETWEEN: &'static str = "chrono.dateTimeBetween";
    pub const TIME_TIME: &'static str = "time.time";
    pub const TIME_DATE: &'static str = "time.date";
    pub const TIME_DATE_TIME: &'static str = "time.dateTime";
    pub const TIME_DURATION: &'static str = "time.duration";
    pub const TIME_DATE_TIME_BEFORE: &'static str = "time.dateTimeBefore";
    pub const TIME_DATE_TIME_AFTER: &'static str = "time.dateTimeAfter";
    pub const TIME_DATE_TIME_BETWEEN: &'static str = "time.dateTimeBetween";
    pub const CREDITCARD_CREDIT_CARD_NUMBER: &'static str = "creditcard.creditCardNumber";
    pub const COMPANY_COMPANY_SUFFIX: &'static str = "company.companySuffix";
    pub const COMPANY_COMPANY_NAME: &'static str = "company.companyName";
    pub const COMPANY_BUZZWORD: &'static str = "company.buzzword";
    pub const COMPANY_BUZZWORD_MIDDLE: &'static str = "company.buzzwordMiddle";
    pub const COMPANY_BUZZWORD_TAIL: &'static str = "company.buzzwordTail";
    pub const COMPANY_CATCH_PHRASE: &'static str = "company.catchPhrase";
    pub const COMPANY_BS_VERB: &'static str = "company.bsVerb";
    pub const COMPANY_BS_ADJ: &'static str = "company.bsAdj";
    pub const COMPANY_BS_NOUN: &'static str = "company.bsNoun";
    pub const COMPANY_BS: &'static str = "company.bs";
    pub const COMPANY_PROFESSION: &'static str = "company.profession";
    pub const COMPANY_INDUSTRY: &'static str = "company.industry";
    pub const HTTP_RFC_STATUS_CODE: &'static str = "http.rfcStatusCode";
    pub const HTTP_VALID_STATUS_CODE: &'static str = "http.validStatusCode";
    pub const INTERNET_FREE_EMAIL_PROVIDER: &'static str = "internet.freeEmailProvider";
    pub const INTERNET_DOMAIN_SUFFIX: &'static str = "internet.domainSuffix";
    pub const INTERNET_FREE_EMAIL: &'static str = "internet.freeEmail";
    pub const INTERNET_SAFE_EMAIL: &'static str = "internet.safeEmail";
    pub const INTERNET_USERNAME: &'static str = "internet.username";
    pub const INTERNET_PASSWORD: &'static str = "internet.password";
    pub const INTERNET_I_PV4: &'static str = "internet.IPv4";
    pub const INTERNET_I_PV6: &'static str = "internet.IPv6";
    pub const INTERNET_IP: &'static str = "internet.IP";
    pub const INTERNET_MAC_ADDRESS: &'static str = "internet.MACAddress";
    pub const INTERNET_USER_AGENT: &'static str = "internet.userAgent";
    pub const JOB_SENIORITY: &'static str = "job.seniority";
    pub const JOB_FIELD: &'static str = "job.field";
    pub const JOB_POSITION: &'static str = "job.position";
    pub const JOB_TITLE: &'static str = "job.title";
    pub const LOREM_WORD: &'static str = "lorem.word";
    pub const LOREM_WORDS: &'static str = "lorem.words";
    pub const LOREM_SENTENCE: &'static str = "lorem.sentence";
    pub const LOREM_SENTENCES: &'static str = "lorem.sentences";
    pub const LOREM_PARAGRAPH: &'static str = "lorem.paragraph";
    pub const LOREM_PARAGRAPHS: &'static str = "lorem.paragraphs";
    pub const MARKDOWN_ITALIC_WORD: &'static str = "markdown.italicWord";
    pub const MARKDOWN_BOLD_WORD: &'static str = "markdown.boldWord";
    pub const MARKDOWN_LINK: &'static str = "markdown.link";
    pub const MARKDOWN_BULLET_POINTS: &'static str = "markdown.bulletPoints";
    pub const MARKDOWN_LIST_ITEMS: &'static str = "markdown.listItems";
    pub const MARKDOWN_BLOCK_QUOTE_SINGLE_LINE: &'static str = "markdown.blockQuoteSingleLine";
    pub const MARKDOWN_BLOCK_QUOTE_MULTI_LINE: &'static str = "markdown.blockQuoteMultiLine";
    pub const MARKDOWN_CODE: &'static str = "markdown.code";
    pub const NAME_FIRST_NAME: &'static str = "name.firstName";
    pub const NAME_LAST_NAME: &'static str = "name.lastName";
    pub const NAME_TITLE: &'static str = "name.title";
    pub const NAME_SUFFIX: &'static str = "name.suffix";
    pub const NAME_NAME: &'static str = "name.name";
    pub const NAME_NAME_WITH_TITLE: &'static str = "name.nameWithTitle";
    pub const NUMBER_DIGIT: &'static str = "number.digit";
    pub const NUMBER_NUMBER_WITH_FORMAT: &'static str = "number.numberWithFormat<'a>";
    pub const PHONE_NUMBER_PHONE_NUMBER: &'static str = "phone_number.phoneNumber";
    pub const PHONE_NUMBER_CELL_NUMBER: &'static str = "phone_number.cellNumber";
    pub const FILESYSTEM_FILE_PATH: &'static str = "filesystem.filePath";
    pub const FILESYSTEM_FILE_NAME: &'static str = "filesystem.fileName";
    pub const FILESYSTEM_FILE_EXTENSION: &'static str = "filesystem.fileExtension";
    pub const FILESYSTEM_DIR_PATH: &'static str = "filesystem.dirPath";
    pub const FILESYSTEM_MIME_TYPE: &'static str = "filesystem.mimeType";
    pub const FILESYSTEM_SEMVER: &'static str = "filesystem.semver";
    pub const FILESYSTEM_SEMVER_STABLE: &'static str = "filesystem.semverStable";
    pub const FILESYSTEM_SEMVER_UNSTABLE: &'static str = "filesystem.semverUnstable";
    pub const CURRENCY_CURRENCY_CODE: &'static str = "currency.currencyCode";
    pub const CURRENCY_CURRENCY_NAME: &'static str = "currency.currencyName";
    pub const CURRENCY_CURRENCY_SYMBOL: &'static str = "currency.currencySymbol";
    pub const FINANCE_BIC: &'static str = "finance.bic";
    pub const FINANCE_ISIN: &'static str = "finance.isin";
    pub const ADMINISTRATIVE_HEALTH_INSURANCE_CODE: &'static str = "administrative.healthInsuranceCode";
    pub const AUTOMOTIVE_LICENCE_PLATE: &'static str = "automotive.licencePlate";

    pub const ULID: &'static str = "ulid";
    pub const UUID_V4: &'static str = "uuid.v4";

    pub fn new() -> Self {
        let mut sets = HashSet::new();

        // Address constants
        sets.insert(Self::ADDRESS_CITY_PREFIX);
        sets.insert(Self::ADDRESS_CITY_SUFFIX);
        sets.insert(Self::ADDRESS_CITY_NAME);
        sets.insert(Self::ADDRESS_COUNTRY_NAME);
        sets.insert(Self::ADDRESS_COUNTRY_CODE);
        sets.insert(Self::ADDRESS_STREET_SUFFIX);
        sets.insert(Self::ADDRESS_STREET_NAME);
        sets.insert(Self::ADDRESS_TIME_ZONE);
        sets.insert(Self::ADDRESS_STATE_NAME);
        sets.insert(Self::ADDRESS_STATE_ABBR);
        sets.insert(Self::ADDRESS_SECONDARY_ADDRESS_TYPE);
        sets.insert(Self::ADDRESS_SECONDARY_ADDRESS);
        sets.insert(Self::ADDRESS_ZIP_CODE);
        sets.insert(Self::ADDRESS_POST_CODE);
        sets.insert(Self::ADDRESS_BUILDING_NUMBER);
        sets.insert(Self::ADDRESS_LATITUDE);
        sets.insert(Self::ADDRESS_LONGITUDE);
        sets.insert(Self::ADDRESS_GEOHASH);

        // Barcode constants
        sets.insert(Self::BARCODE_ISBN);
        sets.insert(Self::BARCODE_ISBN10);
        sets.insert(Self::BARCODE_ISBN13);

        // Boolean constants
        sets.insert(Self::BOOLEAN_BOOLEAN);

        // Color constants
        sets.insert(Self::COLOR_HEX_COLOR);
        sets.insert(Self::COLOR_RGB_COLOR);
        sets.insert(Self::COLOR_RGBA_COLOR);
        sets.insert(Self::COLOR_HSL_COLOR);
        sets.insert(Self::COLOR_HSLA_COLOR);
        sets.insert(Self::COLOR_COLOR);

        // Chrono constants
        sets.insert(Self::CHRONO_TIME);
        sets.insert(Self::CHRONO_DATE);
        sets.insert(Self::CHRONO_DATE_TIME);
        sets.insert(Self::CHRONO_DURATION);
        sets.insert(Self::CHRONO_DATE_TIME_BEFORE);
        sets.insert(Self::CHRONO_DATE_TIME_AFTER);
        sets.insert(Self::CHRONO_DATE_TIME_BETWEEN);

        // Time constants
        sets.insert(Self::TIME_TIME);
        sets.insert(Self::TIME_DATE);
        sets.insert(Self::TIME_DATE_TIME);
        sets.insert(Self::TIME_DURATION);
        sets.insert(Self::TIME_DATE_TIME_BEFORE);
        sets.insert(Self::TIME_DATE_TIME_AFTER);
        sets.insert(Self::TIME_DATE_TIME_BETWEEN);

        // Credit card constants
        sets.insert(Self::CREDITCARD_CREDIT_CARD_NUMBER);

        // Company constants
        sets.insert(Self::COMPANY_COMPANY_SUFFIX);
        sets.insert(Self::COMPANY_COMPANY_NAME);
        sets.insert(Self::COMPANY_BUZZWORD);
        sets.insert(Self::COMPANY_BUZZWORD_MIDDLE);
        sets.insert(Self::COMPANY_BUZZWORD_TAIL);
        sets.insert(Self::COMPANY_CATCH_PHRASE);
        sets.insert(Self::COMPANY_BS_VERB);
        sets.insert(Self::COMPANY_BS_ADJ);
        sets.insert(Self::COMPANY_BS_NOUN);
        sets.insert(Self::COMPANY_BS);
        sets.insert(Self::COMPANY_PROFESSION);
        sets.insert(Self::COMPANY_INDUSTRY);

        // HTTP constants
        sets.insert(Self::HTTP_RFC_STATUS_CODE);
        sets.insert(Self::HTTP_VALID_STATUS_CODE);

        // Internet constants
        sets.insert(Self::INTERNET_FREE_EMAIL_PROVIDER);
        sets.insert(Self::INTERNET_DOMAIN_SUFFIX);
        sets.insert(Self::INTERNET_FREE_EMAIL);
        sets.insert(Self::INTERNET_SAFE_EMAIL);
        sets.insert(Self::INTERNET_USERNAME);
        sets.insert(Self::INTERNET_PASSWORD);
        sets.insert(Self::INTERNET_I_PV4);
        sets.insert(Self::INTERNET_I_PV6);
        sets.insert(Self::INTERNET_IP);
        sets.insert(Self::INTERNET_MAC_ADDRESS);
        sets.insert(Self::INTERNET_USER_AGENT);

        // Job constants
        sets.insert(Self::JOB_SENIORITY);
        sets.insert(Self::JOB_FIELD);
        sets.insert(Self::JOB_POSITION);
        sets.insert(Self::JOB_TITLE);

        // Lorem constants
        sets.insert(Self::LOREM_WORD);
        sets.insert(Self::LOREM_WORDS);
        sets.insert(Self::LOREM_SENTENCE);
        sets.insert(Self::LOREM_SENTENCES);
        sets.insert(Self::LOREM_PARAGRAPH);
        sets.insert(Self::LOREM_PARAGRAPHS);

        // Markdown constants
        sets.insert(Self::MARKDOWN_ITALIC_WORD);
        sets.insert(Self::MARKDOWN_BOLD_WORD);
        sets.insert(Self::MARKDOWN_LINK);
        sets.insert(Self::MARKDOWN_BULLET_POINTS);
        sets.insert(Self::MARKDOWN_LIST_ITEMS);
        sets.insert(Self::MARKDOWN_BLOCK_QUOTE_SINGLE_LINE);
        sets.insert(Self::MARKDOWN_BLOCK_QUOTE_MULTI_LINE);
        sets.insert(Self::MARKDOWN_CODE);

        // Name constants
        sets.insert(Self::NAME_FIRST_NAME);
        sets.insert(Self::NAME_LAST_NAME);
        sets.insert(Self::NAME_TITLE);
        sets.insert(Self::NAME_SUFFIX);
        sets.insert(Self::NAME_NAME);
        sets.insert(Self::NAME_NAME_WITH_TITLE);

        // Number constants
        sets.insert(Self::NUMBER_DIGIT);
        sets.insert(Self::NUMBER_NUMBER_WITH_FORMAT);

        // Phone number constants
        sets.insert(Self::PHONE_NUMBER_PHONE_NUMBER);
        sets.insert(Self::PHONE_NUMBER_CELL_NUMBER);

        // Filesystem constants
        sets.insert(Self::FILESYSTEM_FILE_PATH);
        sets.insert(Self::FILESYSTEM_FILE_NAME);
        sets.insert(Self::FILESYSTEM_FILE_EXTENSION);
        sets.insert(Self::FILESYSTEM_DIR_PATH);
        sets.insert(Self::FILESYSTEM_MIME_TYPE);
        sets.insert(Self::FILESYSTEM_SEMVER);
        sets.insert(Self::FILESYSTEM_SEMVER_STABLE);
        sets.insert(Self::FILESYSTEM_SEMVER_UNSTABLE);

        // Currency constants
        sets.insert(Self::CURRENCY_CURRENCY_CODE);
        sets.insert(Self::CURRENCY_CURRENCY_NAME);
        sets.insert(Self::CURRENCY_CURRENCY_SYMBOL);

        // Finance constants
        sets.insert(Self::FINANCE_BIC);
        sets.insert(Self::FINANCE_ISIN);

        // Administrative constants
        sets.insert(Self::ADMINISTRATIVE_HEALTH_INSURANCE_CODE);

        // Automotive constants
        sets.insert(Self::AUTOMOTIVE_LICENCE_PLATE);

        // IDs
        sets.insert(Self::ULID);
        sets.insert(Self::UUID_V4);

        Self { sets }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.sets.contains(key)
    }
}
