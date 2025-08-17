# JGD-rs - JSON Generator Definition Rust Library

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A simple Rust library for generating realistic JSON data using declarative schema definitions. JGD (JSON Generator Definition) provides a flexible way to create fake data with complex relationships, cross-references, and customizable generation rules.

## Features

- 🎯 **Declarative Schema**: Define data structure using JSON schemas
- 🌍 **Multi-locale Support**: Generate data in different languages (EN, FR_FR, DE_DE, IT_IT, PT_BR, JA_JP, AR_SA, CY_GB)
- 🔄 **Cross-references**: Link data between entities with automatic relationship management
- 🎲 **Deterministic Generation**: Use seeds for reproducible data generation
- 📊 **Rich Data Types**: Support for arrays, objects, numbers, booleans, and complex nested structures
- 🏭 **Faker Integration**: Built-in fake data generation using faker patterns
- ⚙️ **Flexible Counts**: Generate fixed or random counts of data
- 🔧 **Optional Fields**: Probability-based field generation
- 🔑 **Custom Keys**: User-defined custom key functions for specialized data generation
- 📊 **Context-Aware Keys**: Built-in support for index, count, entity.name, and field.name keys

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
      "id": "${ulid}",
      "name": "${name.firstName} ${name.lastName}",
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
}
```

### Multi-Entity Schema with Proper Structure

```json
{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "seed": 12345,
  "entities": {
    "users": {
      "count": 3,
      "fields": {
        "id": "${index}",
        "name": "${name.fullName}",
        "email": "${internet.safeEmail}",
        "summary": "User ${index} of ${count}",
        "details": "${field.name} of ${entity.name}"
      }
    },
    "posts": {
      "count": 10,
      "fields": {
        "id": "${uuid.v4}",
        "userId": {
          "ref": "users.id"
        },
        "title": "${lorem.sentence(3,7)}",
        "content": "${lorem.paragraphs(2,4)}",
        "summary": "${index} of ${count}",
        "tags": {
          "array": {
            "count": [1, 5],
            "of": "${lorem.word}"
          }
        }
      }
    }
  }
}
```

### Using Custom Keys

````rust
use jgd_rs::{Jgd, Arguments};
use serde_json::Value;
use std::sync::Arc;

// Register custom key functions
Jgd::add_custom_key("custom.uuid", Arc::new(|_args: Arguments| {
    Ok(Value::String(uuid::Uuid::new_v4().to_string()))
}));

Jgd::add_custom_key("custom.status", Arc::new(|args: Arguments| {
    let statuses = ["active", "inactive", "pending"];
    let index = match args {
        Arguments::Fixed(n) => n.parse::<usize>().unwrap_or(0) % statuses.len(),
        _ => 0
    };
    Ok(Value::String(statuses[index].to_string()))
}));

let schema = r#"{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "entities": {
    "users": {
      "count": 5,
      "fields": {
        "id": "${custom.uuid}",
        "status": "${custom.status(${index})}",
        "position": "${index} of ${count}"
      }
    }
  }
}"#;

let result = Jgd::from(schema).generate().unwrap();
println!("{}", serde_json::to_string_pretty(&result).unwrap());
```"#;

let json_data = generate_jgd_from_str(schema).unwrap();
println!("{}", serde_json::to_string_pretty(&json_data).unwrap());

// Generate from file
let file_path = PathBuf::from("schema.jgd");
let json_data = generate_jgd_from_file(&file_path).unwrap();
````

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
let result = jgd.generate().unwrap();
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

Arrays are **only for primitive values** (strings, numbers, booleans):

```json
{
  "tags": {
    "array": {
      "count": [1, 5],
      "of": "${lorem.word}"
    }
  },
  "scores": {
    "array": {
      "count": 3,
      "of": {
        "number": {
          "min": 0,
          "max": 100,
          "integer": true
        }
      }
    }
  },
  "flags": {
    "array": {
      "count": 2,
      "of": "${boolean.boolean(70)}"
    }
  }
}
```

**Note:** For objects or lists of objects, use entities with `fields` and `count` properties instead of arrays.

#### Entity Generation

Entities are for objects (single or multiple):

**Single Object:**

```json
{
  "profile": {
    "fields": {
      "bio": "${lorem.paragraph(1,3)}",
      "avatar": "${internet.url}",
      "settings": {
        "fields": {
          "theme": "${lorem.word}",
          "notifications": true
        }
      }
    }
  }
}
```

**List of Objects (use count):**

```json
{
  "comments": {
    "count": [2, 5],
    "fields": {
      "id": "${uuid.v4}",
      "text": "${lorem.sentence(5,15)}",
      "author": "${name.fullName}",
      "createdAt": "${chrono.dateTime}"
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

### Context-Aware Keys

The library provides built-in context-aware keys that give information about the current generation context:

#### Index Key

- `index` - Current item index (1-based) at the current entity level
- `index(depth)` - Item index at a specific depth level
  - `index(1)` - Current entity level (default)
  - `index(2)` - Parent entity level
  - `index(3)` - Grandparent entity level, etc.

#### Other Context Keys

- `count` - Total count of items being generated for the current entity
- `entity.name` - Name of the current entity being generated
- `field.name` - Name of the current field being generated

#### Context Keys Examples

```json
{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "entities": {
    "users": {
      "count": 3,
      "fields": {
        "id": "${index}", // 1, 2, 3
        "summary": "${index} of ${count}", // "1 of 3", "2 of 3", etc.
        "context": "${field.name} in ${entity.name}", // "context in users"
        "profile": {
          "fields": {
            "bio": "${lorem.paragraph(1,2)}",
            "avatar": "${internet.domainSuffix}",
            "settings": {
              "fields": {
                "theme": "${lorem.word}",
                "notifications": true
              }
            }
          }
        },
        "posts": {
          "count": [2, 4],
          "fields": {
            "id": "${uuid.v4}",
            "userId": "${index(2)}", // References parent user index (depth 2)
            "title": "${lorem.sentence(5,10)}",
            "postNumber": "${index}", // 1, 2, 3, 4 (post level - depth 1)
            "summary": "Post ${index} for user ${index(2)}",
            "comments": {
              "count": [1, 3],
              "fields": {
                "id": "${uuid.v4}",
                "postId": "${index(2)}", // References parent post index (depth 2)
                "userId": "${index(3)}", // References grandparent user index (depth 3)
                "text": "${lorem.sentence(3,10)}",
                "position": "Comment ${index} on post ${index(2)} by user ${index(3)}"
              }
            }
          }
        },
        "tags": {
          "array": {
            "count": [1, 5],
            "of": "${lorem.word}"
          }
        }
      }
    }
  }
}
```

**Important Notes:**

- `${index}` only works when there's a `count` property (generates multiple items)
- `${index(2)}` references the parent entity's index (only when parent has count)
- `${index(3)}` references the grandparent entity's index (only when grandparent has count)
- Single objects without `count` (like `profile`, `settings`) don't create new depth levels
- Arrays are only for primitives - use entities with `count` for objects

### Custom Keys

You can register custom key functions to extend the data generation capabilities:

```rust
use jgd_rs::{Jgd, Arguments};
use serde_json::Value;
use std::sync::Arc;

// Register a custom key function
Jgd::add_custom_key("custom.timestamp", Arc::new(|args: Arguments| {
    let timestamp = match args {
        Arguments::None => chrono::Utc::now().timestamp(),
        Arguments::Fixed(offset) => {
            let offset: i64 = offset.parse().unwrap_or(0);
            chrono::Utc::now().timestamp() + offset
        },
        Arguments::Range(start, end) => {
            let start: i64 = start.parse().unwrap_or(0);
            let end: i64 = end.parse().unwrap_or(0);
            start + (end - start) / 2  // Simple midpoint
        }
    };
    Ok(Value::Number(timestamp.into()))
}));

// Use the custom key in your schema
let schema = r#"{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "root": {
    "fields": {
      "createdAt": "${custom.timestamp}",
      "scheduledFor": "${custom.timestamp(3600)}"
    }
  }
}"#;

let result = Jgd::from(schema).generate().unwrap();
```

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

let result1 = jgd1.generate().unwrap();
let result2 = jgd2.generate().unwrap();

// result1 == result2 (same seed produces identical output)
```

## API Reference

### Library Functions

#### `generate_jgd_from_str(schema: &str) -> Result<Value, JgdGeneratorError>`

Generate JSON data from a schema string. Returns a `Result` containing the generated JSON data or an error if generation fails.

#### `generate_jgd_from_file(path: &PathBuf) -> Result<Value, JgdGeneratorError>`

Generate JSON data from a schema file. Returns a `Result` containing the generated JSON data or an error if the file cannot be read or generation fails.

### Jgd Struct

#### `Jgd::from(schema: &str) -> Jgd`

Parse a schema from a string.

#### `Jgd::from(schema: String) -> Jgd`

Parse a schema from an owned string.

#### `Jgd::from(schema: Value) -> Jgd`

Parse a schema from a JSON value.

#### `Jgd::from_file(path: &PathBuf) -> Jgd`

Load a schema from a file.

#### `jgd.generate() -> Result<Value, JgdGeneratorError>`

Generate JSON data according to the schema. Returns a `Result` containing the generated JSON data or an error if generation fails.

#### `jgd.create_config() -> GeneratorConfig`

Create a generator configuration from the schema settings.

#### `Jgd::add_custom_key(key: &str, function: Arc<CustomKeyFunction>)`

Register a custom key function that can be used in templates. The function receives parsed arguments and returns a `Result<Value, String>`.

### Custom Key Functions

Custom key functions have the signature:

```rust
Arc<dyn Fn(Arguments) -> Result<Value, String> + Send + Sync>
```

Where `Arguments` can be:

- `Arguments::None` - No arguments provided
- `Arguments::Fixed(String)` - Single argument value
- `Arguments::Range(String, String)` - Two arguments (start, end)

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
