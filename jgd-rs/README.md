# JGD-rs - JSON Generator Definition Rust Library

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A powerful Rust library for generating realistic JSON data using declarative schema definitions. JGD (JSON Generator Definition) provides a flexible way to create fake data with complex relationships, cross-references, and customizable generation rules.

## Features

- 🎯 **Declarative Schema**: Define data structure using JSON schemas
- 🌍 **Multi-locale Support**: Generate data in different languages (EN, FR_FR, DE_DE, IT_IT, PT_BR, JA_JP, AR_SA, CY_GB)
- 🔄 **Cross-references**: Link data between entities with automatic relationship management
- 🎲 **Deterministic Generation**: Use seeds for reproducible data generation
- 📊 **Rich Data Types**: Support for arrays, objects, numbers, booleans, and complex nested structures
- 🏭 **Faker Integration**: Built-in fake data generation using faker patterns
- ⚙️ **Flexible Counts**: Generate fixed or random counts of data
- 🔧 **Optional Fields**: Probability-based field generation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
jgd-rs = "0.1.0"
```

## Quick Start

### Basic Usage with Library Functions

The simplest way to use jgd-rs is through the convenience functions:

```rust
use jgd_rs::{generate_jgd_from_str, generate_jgd_from_file};
use std::path::PathBuf;

// Generate from JSON string
let schema = r#"{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "root": {
    "fields": {
      "name": "${name.firstName}",
      "email": "${internet.safeEmail}",
      "age": {
        "number": {
          "min": 18,
          "max": 65,
          "integer": true
        }
      }
    }
  }
}"#;

let json_data = generate_jgd_from_str(schema);
println!("{}", serde_json::to_string_pretty(&json_data).unwrap());

// Generate from file
let file_path = PathBuf::from("schema.jgd");
let json_data = generate_jgd_from_file(&file_path);
```

### Advanced Usage with Jgd Struct

For more control, use the `Jgd` struct directly:

```rust
use jgd_rs::Jgd;

let schema = r#"{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "seed": 42,
  "defaultLocale": "EN",
  "root": {
    "fields": {
      "id": "${ulid}",
      "profile": {
        "fields": {
          "name": "${name.name}",
          "email": "${internet.safeEmail}",
          "location": "${address.cityName}, ${address.stateName}"
        }
      }
    }
  }
}"#;

// Parse schema
let jgd = Jgd::from(schema);

// Generate data
let result = jgd.generate();
```

## Schema Modes

JGD supports two mutually exclusive generation modes:

### Root Mode

Generate a single entity (object or array):

```json
{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "root": {
    "fields": {
      "name": "${name.firstName}",
      "age": {
        "number": {
          "min": 18,
          "max": 65,
          "integer": true
        }
      }
    }
  }
}
```

### Entities Mode

Generate multiple named entities with relationships:

```json
{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "entities": {
    "users": {
      "count": 5,
      "fields": {
        "id": "${ulid}",
        "name": "${name.name}",
        "email": "${internet.safeEmail}"
      }
    },
    "posts": {
      "count": [10, 20],
      "fields": {
        "id": "${uuid.v4}",
        "authorId": {
          "ref": "users.id"
        },
        "title": "${lorem.sentence(3,8)}",
        "content": "${lorem.paragraphs(2,4)}"
      }
    }
  }
}
```

## Schema Structure

### Required Fields

- `$format`: Schema format version (always "jgd/v1")
- `version`: User-defined schema version
- Either `root` OR `entities` (mutually exclusive)

### Optional Fields

- `seed`: Random seed for deterministic generation
- `defaultLocale`: Locale for fake data (default: "EN")

### Field Types

#### Primitive Values

```json
{
  "name": "John Doe", // String literal
  "age": 30, // Number literal
  "active": true, // Boolean literal
  "data": null // Null value
}
```

#### Template Strings

```json
{
  "fullName": "${name.firstName} ${name.lastName}",
  "email": "${internet.safeEmail}",
  "id": "${ulid}"
}
```

#### Number Generation

```json
{
  "score": {
    "number": {
      "min": 0,
      "max": 100,
      "integer": true
    }
  }
}
```

#### Array Generation

```json
{
  "tags": {
    "array": {
      "count": [1, 5],
      "of": "${lorem.word}"
    }
  }
}
```

#### Optional Fields

```json
{
  "middleName": {
    "optional": {
      "of": "${name.firstName}",
      "prob": 0.3
    }
  }
}
```

#### Cross-references

```json
{
  "authorId": {
    "ref": "users.id"
  }
}
```

#### Nested Objects

```json
{
  "address": {
    "fields": {
      "street": "${address.streetName}",
      "city": "${address.cityName}",
      "zipCode": "${address.zipCode}"
    }
  }
}
```

## Fake Data Generation

The library uses faker patterns to generate realistic data. All template strings use the `${category.method}` format:

### Supported Categories

#### Address

- `address.cityPrefix` - City prefix (North, South, etc.)
- `address.citySuffix` - City suffix (town, ville, etc.)
- `address.cityName` - Full city name
- `address.countryName` - Country name
- `address.countryCode` - Country code (US, CA, etc.)
- `address.streetSuffix` - Street suffix (St, Ave, etc.)
- `address.streetName` - Street name
- `address.timeZone` - Time zone
- `address.stateName` - State name
- `address.stateAbbr` - State abbreviation
- `address.zipCode` - ZIP code
- `address.postCode` - Postal code
- `address.buildingNumber` - Building number
- `address.latitude` - Latitude coordinate
- `address.longitude` - Longitude coordinate
- `address.geohash(precision)` - Geohash with optional precision

#### Name

- `name.firstName` - First name
- `name.lastName` - Last name
- `name.title` - Name title (Mr., Mrs., etc.)
- `name.suffix` - Name suffix (Jr., Sr., etc.)
- `name.name` - Full name
- `name.nameWithTitle` - Full name with title

#### Internet

- `internet.freeEmailProvider` - Email provider (gmail.com, etc.)
- `internet.domainSuffix` - Domain suffix (.com, .org, etc.)
- `internet.freeEmail` - Free email address
- `internet.safeEmail` - Safe email address
- `internet.username` - Username
- `internet.password(length)` - Password with optional length
- `internet.IPv4` - IPv4 address
- `internet.IPv6` - IPv6 address
- `internet.IP` - IP address (v4 or v6)
- `internet.MACAddress` - MAC address
- `internet.userAgent` - User agent string

#### Company

- `company.companySuffix` - Company suffix (Inc, LLC, etc.)
- `company.companyName` - Company name
- `company.buzzword` - Business buzzword
- `company.buzzwordMiddle` - Middle buzzword
- `company.buzzwordTail` - Tail buzzword
- `company.catchPhrase` - Catch phrase
- `company.bsVerb` - Business verb
- `company.bsAdj` - Business adjective
- `company.bsNoun` - Business noun
- `company.bs` - Business speak
- `company.profession` - Profession
- `company.industry` - Industry

#### Lorem

- `lorem.word` - Single word
- `lorem.words(count)` - Multiple words
- `lorem.sentence(min,max)` - Sentence with word count range
- `lorem.sentences(count)` - Multiple sentences
- `lorem.paragraph(min,max)` - Paragraph with sentence count range
- `lorem.paragraphs(count)` - Multiple paragraphs

#### Time & Date

- `chrono.time` - Time
- `chrono.date` - Date
- `chrono.dateTime` - Date and time
- `chrono.duration` - Duration
- `chrono.dateTimeBefore(date)` - Date before specified date
- `chrono.dateTimeAfter(date)` - Date after specified date
- `chrono.dateTimeBetween(start,end)` - Date between two dates

**Time Aliases (same as chrono.\*):**

- `time.time` - Time (alias for chrono.time)
- `time.date` - Date (alias for chrono.date)
- `time.dateTime` - Date and time (alias for chrono.dateTime)
- `time.duration` - Duration (alias for chrono.duration)
- `time.dateTimeBefore(date)` - Date before specified date (alias for chrono.dateTimeBefore)
- `time.dateTimeAfter(date)` - Date after specified date (alias for chrono.dateTimeAfter)
- `time.dateTimeBetween(start,end)` - Date between two dates (alias for chrono.dateTimeBetween)

#### Numbers & Identifiers

- `number.digit` - Single digit
- `number.numberWithFormat(format)` - Number with custom format
- `ulid` - ULID identifier
- `uuid.v4` - UUID v4

#### Colors

- `color.hexColor` - Hex color (#ffffff)
- `color.rgbColor` - RGB color
- `color.rgbaColor` - RGBA color
- `color.hslColor` - HSL color
- `color.hslaColor` - HSLA color
- `color.color` - Color name

#### Phone & Contact

- `phone_number.phoneNumber` - Phone number
- `phone_number.cellNumber` - Cell phone number

#### Finance

- `creditcard.creditCardNumber` - Credit card number
- `finance.bic` - Bank Identifier Code
- `finance.isin` - International Securities Identification Number
- `currency.currencyCode` - Currency code (USD, EUR, etc.)
- `currency.currencyName` - Currency name
- `currency.currencySymbol` - Currency symbol

#### Filesystem

- `filesystem.filePath` - File path
- `filesystem.fileName` - File name
- `filesystem.fileExtension` - File extension
- `filesystem.dirPath` - Directory path
- `filesystem.mimeType` - MIME type
- `filesystem.semver` - Semantic version
- `filesystem.semverStable` - Stable semantic version
- `filesystem.semverUnstable` - Unstable semantic version

#### Markdown

- `markdown.italicWord` - Italic formatted word
- `markdown.boldWord` - Bold formatted word
- `markdown.link` - Markdown link
- `markdown.bulletPoints` - Bullet point list
- `markdown.listItems` - List items
- `markdown.blockQuoteSingleLine` - Single line block quote
- `markdown.blockQuoteMultiLine` - Multi-line block quote
- `markdown.code` - Code block

#### Administrative

- `administrative.healthInsuranceCode` - Health insurance code

#### Automotive

- `automotive.licencePlate` - License plate number

#### Other Categories

- `barcode.isbn` - ISBN barcode
- `barcode.isbn10` - ISBN-10
- `barcode.isbn13` - ISBN-13
- `boolean.boolean(ratio)` - Boolean with optional true ratio
- `job.seniority` - Job seniority level
- `job.field` - Job field
- `job.position` - Job position
- `job.title` - Job title
- `http.rfcStatusCode` - HTTP status code
- `http.validStatusCode` - Valid HTTP status code

### Method Arguments

Many faker methods accept arguments:

```json
{
  "description": "${lorem.sentence(5,10)}", // 5-10 words
  "tags": "${lorem.words(3)}", // exactly 3 words
  "isActive": "${boolean.boolean(80)}", // 80% chance of true
  "password": "${internet.password(12)}", // 12 character password
  "precision": "${address.geohash(8)}", // 8-character geohash
  "phone": "${number.numberWithFormat(###-###-####)}" // Custom format
}
```

### Localization

Set the `defaultLocale` field to generate locale-specific data:

```json
{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "defaultLocale": "FR_FR",
  "root": {
    "fields": {
      "name": "${name.name}",
      "city": "${address.cityName}"
    }
  }
}
```

**Supported Locales:**

- `EN` - English (default)
- `FR_FR` - French (France)
- `DE_DE` - German (Germany)
- `IT_IT` - Italian (Italy)
- `PT_BR` - Portuguese (Brazil)
- `JA_JP` - Japanese (Japan)
- `AR_SA` - Arabic (Saudi Arabia)
- `CY_GB` - Welsh (Great Britain)

## Count Specifications

Control how many items to generate:

### Fixed Count

```json
{
  "users": {
    "count": 5,
    "fields": { "name": "${name.name}" }
  }
}
```

### Range Count

```json
{
  "posts": {
    "count": [10, 20],
    "fields": { "title": "${lorem.sentence}" }
  }
}
```

### Default Count

If no count is specified, generates a single item.

## Cross-References

Link data between entities using the `ref` field:

```json
{
  "entities": {
    "users": {
      "count": 3,
      "fields": {
        "id": "${ulid}",
        "name": "${name.name}"
      }
    },
    "posts": {
      "count": 10,
      "fields": {
        "id": "${uuid.v4}",
        "authorId": {
          "ref": "users.id"
        },
        "title": "${lorem.sentence}"
      }
    }
  }
}
```

The `ref` field format is `"entityName.fieldName"`. The library will randomly select from the generated entity data.

## Deterministic Generation

Use seeds for reproducible output:

```rust
let schema_with_seed = r#"{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "seed": 42,
  "root": {
    "fields": {
      "random_number": {
        "number": {
          "min": 1,
          "max": 100,
          "integer": true
        }
      }
    }
  }
}"#;

let jgd1 = Jgd::from(schema_with_seed);
let jgd2 = Jgd::from(schema_with_seed);

let result1 = jgd1.generate();
let result2 = jgd2.generate();

// result1 == result2 (same seed produces identical output)
```

## API Reference

### Library Functions

#### `generate_jgd_from_str(schema: &str) -> Value`

Generate JSON data from a schema string.

#### `generate_jgd_from_file(path: &PathBuf) -> Value`

Generate JSON data from a schema file.

### Jgd Struct

#### `Jgd::from(schema: &str) -> Jgd`

Parse a schema from a string.

#### `Jgd::from(schema: String) -> Jgd`

Parse a schema from an owned string.

#### `Jgd::from(schema: Value) -> Jgd`

Parse a schema from a JSON value.

#### `Jgd::from_file(path: &PathBuf) -> Jgd`

Load a schema from a file.

#### `jgd.generate() -> Value`

Generate JSON data according to the schema.

#### `jgd.create_config() -> GeneratorConfig`

Create a generator configuration from the schema settings.

### Schema Fields

#### `format: String`

Schema format identifier (e.g., "jgd/v1").

#### `version: String`

User-defined schema version.

#### `seed: Option<u64>`

Optional random seed for deterministic generation.

#### `default_locale: String`

Locale for fake data generation (default: "EN").

#### `entities: Option<IndexMap<String, Entity>>`

Named entity definitions for entities mode.

#### `root: Option<Entity>`

Root entity definition for root mode.

## Examples

### Simple User Profile

```json
{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "root": {
    "fields": {
      "id": "${ulid}",
      "name": "${name.name}",
      "email": "${internet.safeEmail}",
      "age": {
        "number": {
          "min": 18,
          "max": 65,
          "integer": true
        }
      },
      "city": "${address.cityName}",
      "active": true
    }
  }
}
```

### Blog System with Relationships

```json
{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "seed": 12345,
  "entities": {
    "users": {
      "count": 5,
      "fields": {
        "id": "${ulid}",
        "name": "${name.name}",
        "email": "${internet.safeEmail}",
        "bio": {
          "optional": {
            "of": "${lorem.paragraph(1,3)}",
            "prob": 0.7
          }
        }
      }
    },
    "categories": {
      "count": 3,
      "fields": {
        "id": "${uuid.v4}",
        "name": "${lorem.word}",
        "description": "${lorem.sentence(5,10)}"
      }
    },
    "posts": {
      "count": [15, 25],
      "fields": {
        "id": "${uuid.v4}",
        "authorId": {
          "ref": "users.id"
        },
        "categoryId": {
          "ref": "categories.id"
        },
        "title": "${lorem.sentence(3,8)}",
        "content": "${lorem.paragraphs(2,5)}",
        "tags": {
          "array": {
            "count": [1, 5],
            "of": "${lorem.word}"
          }
        },
        "published": "${boolean.boolean(85)}",
        "createdAt": "${chrono.dateTimeBetween(2023-01-01T00:00:00Z,2024-12-31T23:59:59Z)}"
      }
    }
  }
}
```

### E-commerce Product Catalog

```json
{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "defaultLocale": "EN",
  "root": {
    "count": 100,
    "fields": {
      "id": "${uuid.v4}",
      "sku": "${number.numberWithFormat(ABC-####-###)}",
      "name": "${lorem.words(2,4)}",
      "description": "${lorem.sentence(8,15)}",
      "price": {
        "number": {
          "min": 9.99,
          "max": 999.99,
          "integer": false
        }
      },
      "inStock": "${boolean.boolean(90)}",
      "categories": {
        "array": {
          "count": [1, 3],
          "of": "${lorem.word}"
        }
      },
      "metadata": {
        "fields": {
          "weight": {
            "number": {
              "min": 0.1,
              "max": 50.0,
              "integer": false
            }
          },
          "dimensions": {
            "fields": {
              "length": {
                "number": {
                  "min": 1,
                  "max": 100,
                  "integer": true
                }
              },
              "width": {
                "number": {
                  "min": 1,
                  "max": 100,
                  "integer": true
                }
              },
              "height": {
                "number": {
                  "min": 1,
                  "max": 100,
                  "integer": true
                }
              }
            }
          }
        }
      }
    }
  }
}
```

## Error Handling

The library will panic on invalid schemas. Ensure your schemas:

- Include required fields (`$format`, `version`)
- Have either `root` OR `entities` (not both)
- Use valid JSON syntax
- Reference existing entities in cross-references
- Use supported faker patterns

## Performance

- Use seeds for deterministic generation when testing
- Consider count ranges for more realistic data distribution
- Cross-references are resolved efficiently using internal caching
- Large datasets are generated lazily where possible

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
