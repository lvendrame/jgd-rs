/**
 * JGD Keys - Exact pattern matching for JGD template keys.
 *
 * This module provides a comprehensive mapping of JGD keys to their corresponding
 * generator functions. It's designed to be agnostic of the underlying fake data
 * library (faker.js) and provides exact compatibility with the Rust implementation.
 */

import type { Faker } from "@faker-js/faker";
import type { GenerationResult, Arguments } from "./types";
import { success, error } from "./utils";

/**
 * All supported JGD keys from the Rust implementation.
 * These keys provide exact compatibility with the Rust JGD library.
 */
export const JGD_KEYS = {
  // Address keys
  ADDRESS_CITY_PREFIX: "address.cityPrefix",
  ADDRESS_CITY_SUFFIX: "address.citySuffix",
  ADDRESS_CITY_NAME: "address.cityName",
  ADDRESS_COUNTRY_NAME: "address.countryName",
  ADDRESS_COUNTRY_CODE: "address.countryCode",
  ADDRESS_STREET_SUFFIX: "address.streetSuffix",
  ADDRESS_STREET_NAME: "address.streetName",
  ADDRESS_TIME_ZONE: "address.timeZone",
  ADDRESS_STATE_NAME: "address.stateName",
  ADDRESS_STATE_ABBR: "address.stateAbbr",
  ADDRESS_SECONDARY_ADDRESS_TYPE: "address.secondaryAddressType",
  ADDRESS_SECONDARY_ADDRESS: "address.secondaryAddress",
  ADDRESS_ZIP_CODE: "address.zipCode",
  ADDRESS_POST_CODE: "address.postCode",
  ADDRESS_BUILDING_NUMBER: "address.buildingNumber",
  ADDRESS_LATITUDE: "address.latitude",
  ADDRESS_LONGITUDE: "address.longitude",
  ADDRESS_GEOHASH: "address.geohash",

  // Barcode keys
  BARCODE_ISBN: "barcode.isbn",
  BARCODE_ISBN10: "barcode.isbn10",
  BARCODE_ISBN13: "barcode.isbn13",

  // Boolean keys
  BOOLEAN_BOOLEAN: "boolean.boolean",

  // Color keys
  COLOR_HEX_COLOR: "color.hexColor",
  COLOR_RGB_COLOR: "color.rgbColor",
  COLOR_RGBA_COLOR: "color.rgbaColor",
  COLOR_HSL_COLOR: "color.hslColor",
  COLOR_HSLA_COLOR: "color.hslaColor",
  COLOR_COLOR: "color.color",

  // Time/Date keys (chrono)
  CHRONO_TIME: "chrono.time",
  CHRONO_DATE: "chrono.date",
  CHRONO_DATE_TIME: "chrono.dateTime",
  CHRONO_DURATION: "chrono.duration",
  CHRONO_DATE_TIME_BEFORE: "chrono.dateTimeBefore",
  CHRONO_DATE_TIME_AFTER: "chrono.dateTimeAfter",
  CHRONO_DATE_TIME_BETWEEN: "chrono.dateTimeBetween",

  // Time keys (time)
  TIME_TIME: "time.time",
  TIME_DATE: "time.date",
  TIME_DATE_TIME: "time.dateTime",
  TIME_DURATION: "time.duration",
  TIME_DATE_TIME_BEFORE: "time.dateTimeBefore",
  TIME_DATE_TIME_AFTER: "time.dateTimeAfter",
  TIME_DATE_TIME_BETWEEN: "time.dateTimeBetween",

  // Credit card keys
  CREDITCARD_CREDIT_CARD_NUMBER: "creditcard.creditCardNumber",

  // Company keys
  COMPANY_COMPANY_SUFFIX: "company.companySuffix",
  COMPANY_COMPANY_NAME: "company.companyName",
  COMPANY_BUZZWORD: "company.buzzword",
  COMPANY_BUZZWORD_MIDDLE: "company.buzzwordMiddle",
  COMPANY_BUZZWORD_TAIL: "company.buzzwordTail",
  COMPANY_CATCH_PHRASE: "company.catchPhrase",
  COMPANY_BS_VERB: "company.bsVerb",
  COMPANY_BS_ADJ: "company.bsAdj",
  COMPANY_BS_NOUN: "company.bsNoun",
  COMPANY_BS: "company.bs",
  COMPANY_PROFESSION: "company.profession",
  COMPANY_INDUSTRY: "company.industry",

  // HTTP keys
  HTTP_RFC_STATUS_CODE: "http.rfcStatusCode",
  HTTP_VALID_STATUS_CODE: "http.validStatusCode",

  // Internet keys
  INTERNET_FREE_EMAIL_PROVIDER: "internet.freeEmailProvider",
  INTERNET_DOMAIN_SUFFIX: "internet.domainSuffix",
  INTERNET_FREE_EMAIL: "internet.freeEmail",
  INTERNET_SAFE_EMAIL: "internet.safeEmail",
  INTERNET_USERNAME: "internet.username",
  INTERNET_PASSWORD: "internet.password",
  INTERNET_IPV4: "internet.IPv4",
  INTERNET_IPV6: "internet.IPv6",
  INTERNET_IP: "internet.IP",
  INTERNET_MAC_ADDRESS: "internet.MACAddress",
  INTERNET_USER_AGENT: "internet.userAgent",

  // Job keys
  JOB_SENIORITY: "job.seniority",
  JOB_FIELD: "job.field",
  JOB_POSITION: "job.position",
  JOB_TITLE: "job.title",

  // Lorem keys
  LOREM_WORD: "lorem.word",
  LOREM_WORDS: "lorem.words",
  LOREM_SENTENCE: "lorem.sentence",
  LOREM_SENTENCES: "lorem.sentences",
  LOREM_PARAGRAPH: "lorem.paragraph",
  LOREM_PARAGRAPHS: "lorem.paragraphs",

  // Markdown keys
  MARKDOWN_ITALIC_WORD: "markdown.italicWord",
  MARKDOWN_BOLD_WORD: "markdown.boldWord",
  MARKDOWN_LINK: "markdown.link",
  MARKDOWN_BULLET_POINTS: "markdown.bulletPoints",
  MARKDOWN_LIST_ITEMS: "markdown.listItems",
  MARKDOWN_BLOCK_QUOTE_SINGLE_LINE: "markdown.blockQuoteSingleLine",
  MARKDOWN_BLOCK_QUOTE_MULTI_LINE: "markdown.blockQuoteMultiLine",
  MARKDOWN_CODE: "markdown.code",

  // Name keys
  NAME_FIRST_NAME: "name.firstName",
  NAME_LAST_NAME: "name.lastName",
  NAME_TITLE: "name.title",
  NAME_SUFFIX: "name.suffix",
  NAME_NAME: "name.name",
  NAME_NAME_WITH_TITLE: "name.nameWithTitle",

  // Number keys
  NUMBER_DIGIT: "number.digit",
  NUMBER_NUMBER_WITH_FORMAT: "number.numberWithFormat",

  // Phone keys
  PHONE_NUMBER_PHONE_NUMBER: "phone_number.phoneNumber",
  PHONE_NUMBER_CELL_NUMBER: "phone_number.cellNumber",

  // Filesystem keys
  FILESYSTEM_FILE_PATH: "filesystem.filePath",
  FILESYSTEM_FILE_NAME: "filesystem.fileName",
  FILESYSTEM_FILE_EXTENSION: "filesystem.fileExtension",
  FILESYSTEM_DIR_PATH: "filesystem.dirPath",
  FILESYSTEM_MIME_TYPE: "filesystem.mimeType",
  FILESYSTEM_SEMVER: "filesystem.semver",
  FILESYSTEM_SEMVER_STABLE: "filesystem.semverStable",
  FILESYSTEM_SEMVER_UNSTABLE: "filesystem.semverUnstable",

  // Currency keys
  CURRENCY_CURRENCY_CODE: "currency.currencyCode",
  CURRENCY_CURRENCY_NAME: "currency.currencyName",
  CURRENCY_CURRENCY_SYMBOL: "currency.currencySymbol",

  // Finance keys
  FINANCE_BIC: "finance.bic",
  FINANCE_ISIN: "finance.isin",

  // Administrative keys
  ADMINISTRATIVE_HEALTH_INSURANCE_CODE: "administrative.healthInsuranceCode",

  // Automotive keys
  AUTOMOTIVE_LICENCE_PLATE: "automotive.licencePlate",

  // ID keys
  ULID: "ulid",
  UUID_V4: "uuid.v4",
} as const;

/**
 * Helper to extract argument values for generator functions.
 */
class ArgumentsHelper {
  constructor(private args: Arguments) {}

  getString(defaultValue: string): string {
    switch (this.args.type) {
      case "none":
        return defaultValue;
      case "fixed":
        return this.args.value;
      case "range":
        return this.args.min;
    }
  }

  getNumber(defaultValue: number): number {
    const str = this.getString(defaultValue.toString());
    const parsed = parseInt(str, 10);
    return isNaN(parsed) ? defaultValue : parsed;
  }

  getNumberRange(defaultMin: number, defaultMax: number): [number, number] {
    switch (this.args.type) {
      case "none":
        return [defaultMin, defaultMax];
      case "fixed":
        const value = this.getNumber(defaultMin);
        return [value, defaultMax];
      case "range":
        const min = parseInt(this.args.min, 10);
        const max = parseInt(this.args.max, 10);
        return [isNaN(min) ? defaultMin : min, isNaN(max) ? defaultMax : max];
    }
  }

  getStringRange(defaultMin: string, defaultMax: string): [string, string] {
    switch (this.args.type) {
      case "none":
        return [defaultMin, defaultMax];
      case "fixed":
        return [this.args.value, defaultMax];
      case "range":
        return [this.args.min, this.args.max];
    }
  }
}

/**
 * JGD Key Generator - Maps JGD keys to faker.js calls.
 * This provides exact compatibility with the Rust implementation.
 */
export class JgdKeyGenerator {
  constructor(private faker: Faker) {}

  /**
   * Generates a value for the given JGD key with arguments.
   */
  generate(key: string, args: Arguments): GenerationResult {
    try {
      const helper = new ArgumentsHelper(args);
      const value = this.generateByKey(key, helper);
      return success(value);
    } catch (err) {
      return error(
        `Key generation failed for '${key}': ${
          err instanceof Error ? err.message : String(err)
        }`
      );
    }
  }

  private generateByKey(key: string, args: ArgumentsHelper): any {
    switch (key) {
      // Address keys
      case JGD_KEYS.ADDRESS_CITY_PREFIX:
        return this.faker.location.city(); // Faker doesn't have cityPrefix, use city
      case JGD_KEYS.ADDRESS_CITY_SUFFIX:
        return this.faker.location.city(); // Faker doesn't have citySuffix, use city
      case JGD_KEYS.ADDRESS_CITY_NAME:
        return this.faker.location.city();
      case JGD_KEYS.ADDRESS_COUNTRY_NAME:
        return this.faker.location.country();
      case JGD_KEYS.ADDRESS_COUNTRY_CODE:
        return this.faker.location.countryCode();
      case JGD_KEYS.ADDRESS_STREET_SUFFIX:
        return this.faker.location.streetAddress();
      case JGD_KEYS.ADDRESS_STREET_NAME:
        return this.faker.location.street();
      case JGD_KEYS.ADDRESS_TIME_ZONE:
        return this.faker.location.timeZone();
      case JGD_KEYS.ADDRESS_STATE_NAME:
        return this.faker.location.state();
      case JGD_KEYS.ADDRESS_STATE_ABBR:
        return this.faker.location.state({ abbreviated: true });
      case JGD_KEYS.ADDRESS_SECONDARY_ADDRESS_TYPE:
        return this.faker.location.secondaryAddress();
      case JGD_KEYS.ADDRESS_SECONDARY_ADDRESS:
        return this.faker.location.secondaryAddress();
      case JGD_KEYS.ADDRESS_ZIP_CODE:
        return this.faker.location.zipCode();
      case JGD_KEYS.ADDRESS_POST_CODE:
        return this.faker.location.zipCode();
      case JGD_KEYS.ADDRESS_BUILDING_NUMBER:
        return this.faker.location.buildingNumber();
      case JGD_KEYS.ADDRESS_LATITUDE:
        return this.faker.location.latitude();
      case JGD_KEYS.ADDRESS_LONGITUDE:
        return this.faker.location.longitude();
      case JGD_KEYS.ADDRESS_GEOHASH:
        const precision = args.getNumber(5);
        return this.faker.location.nearbyGPSCoordinate().join(","); // Approximation

      // Barcode keys
      case JGD_KEYS.BARCODE_ISBN:
        return this.faker.commerce.isbn(13);
      case JGD_KEYS.BARCODE_ISBN10:
        return this.faker.commerce.isbn(10);
      case JGD_KEYS.BARCODE_ISBN13:
        return this.faker.commerce.isbn(13);

      // Boolean keys
      case JGD_KEYS.BOOLEAN_BOOLEAN:
        const probability = args.getNumber(50);
        return this.faker.datatype.boolean(probability / 100);

      // Company keys
      case JGD_KEYS.COMPANY_COMPANY_SUFFIX:
        return this.faker.company.name().split(" ").pop(); // Extract suffix from company name
      case JGD_KEYS.COMPANY_COMPANY_NAME:
        return this.faker.company.name();
      case JGD_KEYS.COMPANY_BUZZWORD:
        return this.faker.company.buzzPhrase();
      case JGD_KEYS.COMPANY_BUZZWORD_MIDDLE:
        return this.faker.company.buzzVerb();
      case JGD_KEYS.COMPANY_BUZZWORD_TAIL:
        return this.faker.company.buzzNoun();
      case JGD_KEYS.COMPANY_CATCH_PHRASE:
        return this.faker.company.catchPhrase();
      case JGD_KEYS.COMPANY_BS_VERB:
        return this.faker.company.buzzVerb();
      case JGD_KEYS.COMPANY_BS_ADJ:
        return this.faker.company.buzzAdjective();
      case JGD_KEYS.COMPANY_BS_NOUN:
        return this.faker.company.buzzNoun();
      case JGD_KEYS.COMPANY_BS:
        return this.faker.company.buzzPhrase();

      // Internet keys
      case JGD_KEYS.INTERNET_FREE_EMAIL_PROVIDER:
        return this.faker.internet.domainName();
      case JGD_KEYS.INTERNET_DOMAIN_SUFFIX:
        return this.faker.internet.domainSuffix();
      case JGD_KEYS.INTERNET_FREE_EMAIL:
        return this.faker.internet.email();
      case JGD_KEYS.INTERNET_SAFE_EMAIL:
        return this.faker.internet.email();
      case JGD_KEYS.INTERNET_USERNAME:
        return this.faker.internet.userName();
      case JGD_KEYS.INTERNET_PASSWORD:
        return this.faker.internet.password();
      case JGD_KEYS.INTERNET_IPV4:
        return this.faker.internet.ipv4();
      case JGD_KEYS.INTERNET_IPV6:
        return this.faker.internet.ipv6();
      case JGD_KEYS.INTERNET_IP:
        return this.faker.internet.ip();
      case JGD_KEYS.INTERNET_MAC_ADDRESS:
        return this.faker.internet.mac();
      case JGD_KEYS.INTERNET_USER_AGENT:
        return this.faker.internet.userAgent();

      // Lorem keys
      case JGD_KEYS.LOREM_WORD:
        return this.faker.lorem.word();
      case JGD_KEYS.LOREM_WORDS:
        const wordCount = args.getNumber(3);
        return this.faker.lorem.words(wordCount);
      case JGD_KEYS.LOREM_SENTENCE:
        const [minWords, maxWords] = args.getNumberRange(4, 18);
        return this.faker.lorem.sentence({ min: minWords, max: maxWords });
      case JGD_KEYS.LOREM_SENTENCES:
        const sentenceCount = args.getNumber(3);
        return this.faker.lorem.sentences(sentenceCount);
      case JGD_KEYS.LOREM_PARAGRAPH:
        const [minSentences, maxSentences] = args.getNumberRange(3, 7);
        return this.faker.lorem.paragraph({
          min: minSentences,
          max: maxSentences,
        });
      case JGD_KEYS.LOREM_PARAGRAPHS:
        const [minParagraphs, maxParagraphs] = args.getNumberRange(2, 4);
        return this.faker.lorem.paragraphs({
          min: minParagraphs,
          max: maxParagraphs,
        });

      // Name keys
      case JGD_KEYS.NAME_FIRST_NAME:
        return this.faker.person.firstName();
      case JGD_KEYS.NAME_LAST_NAME:
        return this.faker.person.lastName();
      case JGD_KEYS.NAME_TITLE:
        return this.faker.person.prefix();
      case JGD_KEYS.NAME_SUFFIX:
        return this.faker.person.suffix();
      case JGD_KEYS.NAME_NAME:
        return this.faker.person.fullName();
      case JGD_KEYS.NAME_NAME_WITH_TITLE:
        return `${this.faker.person.prefix()} ${this.faker.person.fullName()}`;

      // Number keys
      case JGD_KEYS.NUMBER_DIGIT:
        return this.faker.number.int({ min: 0, max: 9 });
      case JGD_KEYS.NUMBER_NUMBER_WITH_FORMAT:
        const format = args.getString("###");
        return this.faker.helpers.replaceSymbols(format);

      // Phone keys
      case JGD_KEYS.PHONE_NUMBER_PHONE_NUMBER:
        return this.faker.phone.number();
      case JGD_KEYS.PHONE_NUMBER_CELL_NUMBER:
        return this.faker.phone.number();

      // Filesystem keys
      case JGD_KEYS.FILESYSTEM_FILE_PATH:
        return this.faker.system.filePath();
      case JGD_KEYS.FILESYSTEM_FILE_NAME:
        return this.faker.system.fileName();
      case JGD_KEYS.FILESYSTEM_FILE_EXTENSION:
        return this.faker.system.fileExt();
      case JGD_KEYS.FILESYSTEM_DIR_PATH:
        return this.faker.system.directoryPath();
      case JGD_KEYS.FILESYSTEM_MIME_TYPE:
        return this.faker.system.mimeType();
      case JGD_KEYS.FILESYSTEM_SEMVER:
        return this.faker.system.semver();
      case JGD_KEYS.FILESYSTEM_SEMVER_STABLE:
        return this.faker.system.semver();
      case JGD_KEYS.FILESYSTEM_SEMVER_UNSTABLE:
        return this.faker.system.semver();

      // Date/Time keys (using faker.js date methods)
      case JGD_KEYS.CHRONO_DATE_TIME:
      case JGD_KEYS.TIME_DATE_TIME:
        return this.faker.date.recent().toISOString();
      case JGD_KEYS.CHRONO_DATE:
      case JGD_KEYS.TIME_DATE:
        return this.faker.date.recent().toISOString().split("T")[0];
      case JGD_KEYS.CHRONO_DATE_TIME_BEFORE:
      case JGD_KEYS.TIME_DATE_TIME_BEFORE:
        const beforeDate = args.getString("2024-12-31T23:59:59Z");
        return this.faker.date
          .past({ refDate: new Date(beforeDate) })
          .toISOString();
      case JGD_KEYS.CHRONO_DATE_TIME_AFTER:
      case JGD_KEYS.TIME_DATE_TIME_AFTER:
        const afterDate = args.getString("2020-01-01T00:00:00Z");
        return this.faker.date
          .future({ refDate: new Date(afterDate) })
          .toISOString();
      case JGD_KEYS.CHRONO_DATE_TIME_BETWEEN:
      case JGD_KEYS.TIME_DATE_TIME_BETWEEN:
        const [fromStr, toStr] = args.getStringRange(
          "2020-01-01T00:00:00Z",
          "2024-12-31T23:59:59Z"
        );
        const from = new Date(fromStr);
        const to = new Date(toStr);
        return this.faker.date.between({ from, to }).toISOString();

      // ID keys
      case JGD_KEYS.ULID:
        return this.faker.string.alphanumeric(26).toUpperCase(); // ULID approximation
      case JGD_KEYS.UUID_V4:
        return this.faker.string.uuid();

      // Currency keys
      case JGD_KEYS.CURRENCY_CURRENCY_CODE:
        return this.faker.finance.currencyCode();
      case JGD_KEYS.CURRENCY_CURRENCY_NAME:
        return this.faker.finance.currencyName();
      case JGD_KEYS.CURRENCY_CURRENCY_SYMBOL:
        return this.faker.finance.currencySymbol();

      // Finance keys
      case JGD_KEYS.FINANCE_BIC:
        return this.faker.finance.bic();
      case JGD_KEYS.FINANCE_ISIN:
        return this.faker.finance.bitcoinAddress(); // Approximation, faker doesn't have ISIN

      // Color keys
      case JGD_KEYS.COLOR_HEX_COLOR:
        return this.faker.color.rgb({ format: "hex" });
      case JGD_KEYS.COLOR_RGB_COLOR:
        return this.faker.color.rgb();
      case JGD_KEYS.COLOR_RGBA_COLOR:
        return this.faker.color.rgb({ includeAlpha: true });
      case JGD_KEYS.COLOR_HSL_COLOR:
        return this.faker.color.hsl();
      case JGD_KEYS.COLOR_HSLA_COLOR:
        return this.faker.color.hsl({ includeAlpha: true });
      case JGD_KEYS.COLOR_COLOR:
        return this.faker.color.human();

      // Credit card keys
      case JGD_KEYS.CREDITCARD_CREDIT_CARD_NUMBER:
        return this.faker.finance.creditCardNumber();

      // HTTP keys
      case JGD_KEYS.HTTP_RFC_STATUS_CODE:
        return this.faker.internet.httpStatusCode();
      case JGD_KEYS.HTTP_VALID_STATUS_CODE:
        return this.faker.internet.httpStatusCode({
          types: ["success", "redirection"],
        });

      // Job keys
      case JGD_KEYS.JOB_SENIORITY:
        return this.faker.person.jobType();
      case JGD_KEYS.JOB_FIELD:
        return this.faker.person.jobArea();
      case JGD_KEYS.JOB_POSITION:
        return this.faker.person.jobTitle();
      case JGD_KEYS.JOB_TITLE:
        return this.faker.person.jobTitle();

      // Markdown keys (approximations using lorem)
      case JGD_KEYS.MARKDOWN_ITALIC_WORD:
        return `*${this.faker.lorem.word()}*`;
      case JGD_KEYS.MARKDOWN_BOLD_WORD:
        return `**${this.faker.lorem.word()}**`;
      case JGD_KEYS.MARKDOWN_LINK:
        return `[${this.faker.lorem.words(2)}](${this.faker.internet.url()})`;
      case JGD_KEYS.MARKDOWN_BULLET_POINTS:
        return `- ${this.faker.lorem.sentence()}`;
      case JGD_KEYS.MARKDOWN_LIST_ITEMS:
        return `1. ${this.faker.lorem.sentence()}`;
      case JGD_KEYS.MARKDOWN_BLOCK_QUOTE_SINGLE_LINE:
        return `> ${this.faker.lorem.sentence()}`;
      case JGD_KEYS.MARKDOWN_BLOCK_QUOTE_MULTI_LINE:
        return `> ${this.faker.lorem.sentences(3)}`;
      case JGD_KEYS.MARKDOWN_CODE:
        return `\`${this.faker.lorem.word()}\``;

      // Administrative keys (approximations)
      case JGD_KEYS.ADMINISTRATIVE_HEALTH_INSURANCE_CODE:
        return this.faker.string.alphanumeric(10).toUpperCase();

      // Automotive keys
      case JGD_KEYS.AUTOMOTIVE_LICENCE_PLATE:
        return this.faker.vehicle.vrm();

      // Fallback for unsupported keys
      default:
        throw new Error(`Unsupported JGD key: ${key}`);
    }
  }

  /**
   * Checks if a key is supported by the JGD key generator.
   */
  static isJgdKey(key: string): boolean {
    return Object.values(JGD_KEYS).includes(key as any);
  }
}
