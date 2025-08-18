# JGD.js - JSON Generator Definition for TypeScript

[![npm version](https://img.shields.io/npm/v/jgd.js.svg)](https://www.npmjs.com/package/jgd.js)
[![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=flat&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A TypeScript library for generating realistic JSON data using declarative schema definitions. Based on the Rust JGD library architecture with full TypeScript support and modern JavaScript best practices.

## 🚀 Features

- 🎯 **Declarative Schema**: Define complex data structures using JSON schemas
- 🌍 **Multi-locale Support**: Generate data in different languages (EN, FR_FR, IT_IT, JA_JP, DE_DE, PT_BR, AR_SA, CY_GB)
- 🔄 **Cross-references**: Link data between entities with automatic relationship management
- 🔢 **Deterministic Generation**: Use seeds for reproducible results
- 📊 **Rich Data Types**: Support for arrays, objects, numbers, booleans, and complex nested structures
- 🌐 **Faker Integration**: Built-in support for 100+ realistic fake data patterns
- ⚙️ **Flexible Counts**: Generate fixed or random counts of data with range specifications
- 🔧 **Optional Fields**: Probability-based field generation
- 🔑 **Custom Keys**: Register your own data generation functions
- 📊 **Context-Aware Keys**: Built-in support for index, count, entity.name, and field.name keys
- ⚡ **TypeScript Native**: Full type safety and IntelliSense support
- 🧪 **Well Tested**: Comprehensive test suite with 30+ tests covering all example schemas

## 📦 Installation

```bash
npm install jgd.js
```

## 🚀 Quick Start

### Basic Usage

```typescript
import { Jgd } from "jgd.js";

const schema = {
  $format: "jgd/v1",
  version: "1.0.0",
  root: {
    fields: {
      id: "${ulid}",
      name: "${name.name}",
      email: "${internet.safeEmail}",
      age: {
        number: {
          min: 18,
          max: 65,
          integer: true,
        },
      },
      active: true,
    },
  },
};

const jgd = Jgd.fromObject(schema);
const result = jgd.generate();

if (result.success) {
  console.log(result.data);
  // Output:
  // {
  //   id: "01HZ8X2QR3EXAMPLE123456789",
  //   name: "John Smith",
  //   email: "john.smith@example.com",
  //   age: 32,
  //   active: true
  // }
}
```

### Multi-Entity Schema with Relationships

```typescript
const schema = {
  $format: "jgd/v1",
  version: "1.0.0",
  seed: 12345,
  entities: {
    users: {
      count: 3,
      fields: {
        id: "${ulid}",
        name: "${name.name}",
        email: "${internet.safeEmail}",
        summary: "User ${index} of ${count}",
        context: "${field.name} in ${entity.name}",
      },
    },
    posts: {
      count: [5, 10],
      fields: {
        id: "${uuid.v4}",
        userId: { ref: "users.id" },
        title: "${lorem.sentence}",
        content: "${lorem.paragraphs}",
        tags: {
          array: {
            count: [1, 5],
            of: "${lorem.word}",
          },
        },
      },
    },
  },
};
```

### From File

```typescript
import { Jgd, generateFromFile } from "jgd.js";

// Load and generate from a .jgd file
const result = generateFromFile("./schemas/user-posts.jgd");
if (result.success) {
  console.log(result.data);
}

// Or create instance first
const jgd = Jgd.fromFile("./schemas/user-posts.jgd");
const data = jgd.generate();
```

## 📋 Schema Structure

JGD schemas follow a structured format that defines how your fake data should be generated:

### Basic Schema Components

```typescript
{
  "$format": "jgd/v1",           // Required: Schema format version
  "version": "1.0.0",           // Required: Your schema version
  "seed": 42,                   // Optional: Deterministic generation seed
  "defaultLocale": "EN",        // Optional: Faker locale (default: EN)
  "root": { ... }               // Single entity root
  // OR
  "entities": { ... }           // Multiple named entities
}
```

### Entity Structure

```typescript
{
  "count": 5,                   // Fixed count
  // OR
  "count": [1, 10],            // Random count range

  "fields": {
    "fieldName": "value",       // Static value (string, number, boolean, null)
    "fieldName": "${faker.pattern}", // Faker pattern string
    "fieldName": {              // Entity (nested object)
      "fields": { ... }
    },
    "fieldName": {              // Array
      "array": { "count": 3, "of": "..." }
    },
    "fieldName": {              // Number specification
      "number": { "min": 1, "max": 10 }
    },
    "fieldName": {              // Optional field wrapper
      "optional": {
        "of": "${faker.pattern}",
        "prob": 0.7             // 70% chance field exists
      }
    },
    "fieldName": {              // Reference to another entity
      "ref": "entity.field"
    }
  }
}
```

## 🎯 Field Types & Patterns

### Static Values

```typescript
{
  "name": "John Doe",           // String literal
  "age": 25,                   // Number literal
  "active": true,              // Boolean literal
  "metadata": null             // Null value
}
```

### Number Ranges

```typescript
{
  "age": {
    "number": {
      "min": 18,
      "max": 65
    }
  },
  "rating": {
    "number": {
      "min": 1.0,
      "max": 5.0,
      "integer": false
    }
  },
  "score": {
    "number": {
      "min": 0,
      "max": 100,
      "integer": true
    }
  }
}
```

````

### Arrays
```typescript
{
  "tags": {
    "array": {
      "count": [1, 5],          // Random count of items
      "of": "${lorem.word}"     // Pattern for each item
    }
  },
  "categories": {
    "array": {
      "count": 3,
      "of": {                   // Complex objects in array
        "fields": {             // Must be an Entity with fields
          "id": "${index}",
          "name": "${company.companyName}"
        }
      }
    }
  }
}
````

### Optional Fields

```typescript
{
  "profile": {
    "optional": {               // Optional field wrapper
      "of": {
        "fields": {             // Must be an Entity with fields
          "bio": "${lorem.paragraph}",
          "avatar": "${internet.userAgent}"
        }
      },
      "prob": 0.8              // 80% chance this field exists
    }
  }
}
```

### Nested Objects

```typescript
{
  "profile": {
    "fields": {
      "bio": "${lorem.paragraph}",
      "settings": {
        "fields": {
          "theme": "dark",
          "notifications": true
        }
      }
    }
  }
}
```

## 🌍 Faker Patterns

JGD supports 100+ faker patterns across multiple categories:

### Person & Identity

```typescript
"${name.firstName}"; // First name
"${name.lastName}"; // Last name
"${name.name}"; // Full name
"${name.title}"; // Title prefix (Mr., Dr., etc.)
"${name.suffix}"; // Name suffix (Jr., PhD, etc.)
"${name.nameWithTitle}"; // Name with title
```

### Internet & Communication

```typescript
"${internet.freeEmail}"; // Free email address
"${internet.freeEmailProvider}"; // Free email provider domain
"${internet.safeEmail}"; // Safe email (example.com domain)
"${internet.domainSuffix}"; // Domain suffix (.com, .org, etc.)
"${internet.username}"; // Username
"${internet.password}"; // Password
"${internet.IPv4}"; // IPv4 address
"${internet.IPv6}"; // IPv6 address
"${internet.IP}"; // IP address
"${internet.MACAddress}"; // MAC address
"${internet.userAgent}"; // User agent string
"${phone_number.phoneNumber}"; // Phone number
"${phone_number.cellNumber}"; // Cell phone number
```

### Location & Geography

```typescript
"${address.cityPrefix}"; // City prefix
"${address.citySuffix}"; // City suffix
"${address.cityName}"; // City name
"${address.countryName}"; // Country name
"${address.countryCode}"; // Country code
"${address.streetSuffix}"; // Street suffix
"${address.streetName}"; // Street name
"${address.timeZone}"; // Time zone
"${address.stateName}"; // State/province name
"${address.stateAbbr}"; // State abbreviation
"${address.secondaryAddressType}"; // Secondary address type
"${address.secondaryAddress}"; // Secondary address
"${address.zipCode}"; // ZIP/postal code
"${address.postCode}"; // Postal code
"${address.buildingNumber}"; // Building number
"${address.latitude}"; // Latitude coordinate
"${address.longitude}"; // Longitude coordinate
"${address.geohash}"; // Geohash
```

### Commerce & Business

```typescript
"${company.companyName}"; // Company name
"${company.companySuffix}"; // Company suffix
"${company.buzzword}"; // Business buzzword
"${company.buzzwordMiddle}"; // Business buzzword (middle)
"${company.buzzwordTail}"; // Business buzzword (tail)
"${company.catchPhrase}"; // Company slogan
"${company.bsVerb}"; // Business speak verb
"${company.bsAdj}"; // Business speak adjective
"${company.bsNoun}"; // Business speak noun
"${company.bs}"; // Business speak phrase
"${company.profession}"; // Profession
"${company.industry}"; // Industry
```

### Finance & Currency

```typescript
"${finance.bic}"; // Bank Identifier Code
"${finance.isin}"; // ISIN code
"${currency.currencyCode}"; // Currency code (USD, EUR, etc.)
"${currency.currencyName}"; // Currency name
"${currency.currencySymbol}"; // Currency symbol ($, €, etc.)
"${creditcard.creditCardNumber}"; // Credit card number
```

### Text & Content

```typescript
"${lorem.word}"; // Single word
"${lorem.words}"; // Multiple words
"${lorem.sentence}"; // Sentence
"${lorem.sentences}"; // Multiple sentences
"${lorem.paragraph}"; // Paragraph
"${lorem.paragraphs}"; // Multiple paragraphs
```

### Markdown & Formatting

```typescript
"${markdown.italicWord}"; // *italic* word
"${markdown.boldWord}"; // **bold** word
"${markdown.link}"; // [link](url)
"${markdown.bulletPoints}"; // - bullet point
"${markdown.listItems}"; // 1. numbered item
"${markdown.blockQuoteSingleLine}"; // > single line quote
"${markdown.blockQuoteMultiLine}"; // > multi line quote
"${markdown.code}"; // `code`
```

### Dates & Time

```typescript
"${chrono.time}"; // Time string
"${chrono.date}"; // Date string
"${chrono.dateTime}"; // Date/time string
"${chrono.duration}"; // Duration
"${chrono.dateTimeBefore}"; // Date/time before specified date
"${chrono.dateTimeAfter}"; // Date/time after specified date
"${chrono.dateTimeBetween}"; // Date/time between two dates
"${time.time}"; // Time string
"${time.date}"; // Date string
"${time.dateTime}"; // Date/time string
"${time.duration}"; // Duration
"${time.dateTimeBefore}"; // Date/time before specified date
"${time.dateTimeAfter}"; // Date/time after specified date
"${time.dateTimeBetween}"; // Date/time between two dates
```

### Colors

```typescript
"${color.hexColor}"; // Hex color (#ff0000)
"${color.rgbColor}"; // RGB color
"${color.rgbaColor}"; // RGBA color
"${color.hslColor}"; // HSL color
"${color.hslaColor}"; // HSLA color
"${color.color}"; // Human-readable color name
```

### Numbers & Booleans

```typescript
"${number.digit}"; // Single digit (0-9)
"${number.numberWithFormat}"; // Number with custom format
"${boolean.boolean}"; // Boolean value
```

### Job & Career

```typescript
"${job.seniority}"; // Job seniority level
"${job.field}"; // Job field/area
"${job.position}"; // Job position
"${job.title}"; // Job title
```

### HTTP & Technical

```typescript
"${http.rfcStatusCode}"; // RFC HTTP status code
"${http.validStatusCode}"; // Valid HTTP status code
"${barcode.isbn}"; // ISBN barcode
"${barcode.isbn10}"; // ISBN-10
"${barcode.isbn13}"; // ISBN-13
```

### Filesystem & Development

```typescript
"${filesystem.filePath}"; // File path
"${filesystem.fileName}"; // File name
"${filesystem.fileExtension}"; // File extension
"${filesystem.dirPath}"; // Directory path
"${filesystem.mimeType}"; // MIME type
"${filesystem.semver}"; // Semantic version
"${filesystem.semverStable}"; // Stable semantic version
"${filesystem.semverUnstable}"; // Unstable semantic version
```

### Administrative & Automotive

```typescript
"${administrative.healthInsuranceCode}"; // Health insurance code
"${automotive.licencePlate}"; // Vehicle license plate
```

### Unique Identifiers

```typescript
"${ulid}"; // ULID
"${uuid.v4}"; // UUID v4
```

## 🔗 Context-Aware Keys

JGD provides special context-aware patterns that reference generation state:

### Index & Counting

```typescript
"${index}"; // Current item index (1-based)
"${index(2)}"; // Parent entity index (2 levels up, 1-based)
"${count}"; // Total count for current entity
```

### Entity & Field References

```typescript
"${entity.name}"; // Current entity name
"${field.name}"; // Current field name
```

### Practical Examples

```typescript
{
  "entities": {
    "users": {
      "count": 3,
      "fields": {
        "id": "${index}",                     // 1, 2, 3
        "name": "${name.name}"
        "summary": "User ${index} of ${count}", // "User 1 of 3"
        "posts": {
          "count": [1, 3],
          "fields": {
            "id": "${uuid.v4}",
            "userId": "${index(2)}",          // Parent user index
            "title": "${lorem.sentence}",
            "meta": "Post in ${entity.name}"  // "Post in users"
          }
        }
      }
    }
  }
}
```

## 🌐 Multi-Locale Support

JGD supports multiple locales for generating localized fake data:

```typescript
const schema = {
  $format: "jgd/v1",
  version: "1.0.0",
  defaultLocale: "DE_DE", // German locale
  root: {
    fields: {
      name: "${name.name}", // German names
      address: "${address.streetName}", // German addresses
      phone: "${phone_number.phoneNumber}", // German phone format
    },
  },
};
```

**Supported Locales**: `EN`, `FR_FR`, `IT_IT`, `JA_JP`, `DE_DE`, `PT_BR`, `AR_SA`, `CY_GB`.

## 🎲 Deterministic Generation

Use seeds for reproducible fake data:

```typescript
const schema = {
  $format: "jgd/v1",
  version: "1.0.0",
  seed: 42, // Same seed = same data
  root: {
    fields: {
      name: "${name.name}",
    },
  },
};

// Multiple runs with same seed produce identical results
const jgd = Jgd.fromObject(schema);
const result1 = jgd.generate();
const result2 = jgd.generate();
// result1.data === result2.data
```

## 📚 API Reference

### Core Classes

#### `Jgd`

Main class for JGD schema processing and data generation.

```typescript
class Jgd {
  // Static factory methods
  static fromObject(schema: object): Jgd;
  static fromFile(filePath: string): Jgd;
  static fromString(content: string): Jgd;

  // Instance methods
  generate(): GenerationResult<any>;
  generateMany(count: number): GenerationResult<any[]>;

  // Properties
  readonly schema: JgdSchema;
}
```

#### `GenerationResult<T>`

Result wrapper with success/error handling.

```typescript
interface GenerationResult<T> {
  success: boolean;
  data?: T;
  error?: string;
}
```

### Utility Functions

```typescript
// Generate directly from file
function generateFromFile(filePath: string): GenerationResult<any>;

// Generate directly from object
function generateFromObject(schema: object): GenerationResult<any>;

// Generate directly from string
function generateFromString(content: string): GenerationResult<any>;

// Generate multiple from any source
function generateManyFromFile(
  filePath: string,
  count: number
): GenerationResult<any[]>;
function generateManyFromObject(
  schema: object,
  count: number
): GenerationResult<any[]>;
```

### Error Handling

```typescript
const result = jgd.generate();

if (result.success) {
  console.log("Generated data:", result.data);
} else {
  console.error("Generation failed:", result.error);
}
```

## 💡 Advanced Examples

### E-commerce Product Catalog

```typescript
const ecommerceSchema = {
  $format: "jgd/v1",
  version: "1.0.0",
  seed: 123,
  entities: {
    categories: {
      count: 5,
      fields: {
        id: "${index}",
        name: "${company.companyName}",
        slug: "${lorem.word}",
        description: "${lorem.paragraph}",
      },
    },
    products: {
      count: [20, 50],
      fields: {
        id: "${uuid.v4}",
        categoryId: "${index(1)}", // Reference category
        name: "${lorem.word}",
        description: "${lorem.sentence}",
        price: {
          number: {
            min: 9.99,
            max: 999.99,
            integer: false,
          },
        },
        sku: "${number.numberWithFormat}",
        inStock: "${boolean.boolean}",
        tags: {
          array: {
            count: [1, 5],
            of: "${lorem.word}",
          },
        },
        metadata: {
          optional: {
            of: {
              fields: {
                // Must be an Entity with fields
                weight: {
                  number: {
                    min: 0.1,
                    max: 50.0,
                  },
                },
                dimensions: "${lorem.words}",
              },
            },
            prob: 0.7,
          },
        },
      },
    },
  },
};
```

### User Management System

```typescript
const userSystemSchema = {
  $format: "jgd/v1",
  version: "2.0.0",
  defaultLocale: "EN",
  entities: {
    roles: {
      count: 3,
      fields: {
        id: "${index}",
        name: "role_${index}",
        permissions: {
          array: {
            count: [1, 5],
            of: "${lorem.word}",
          },
        },
      },
    },
    users: {
      count: 100,
      fields: {
        id: "${ulid}",
        roleId: "${index(1)}", // Reference role
        profile: {
          firstName: "${name.firstName}",
          lastName: "${name.lastName}",
          email: "${internet.safeEmail}",
          avatar: "${internet.userAgent}",
          bio: {
            optional: {
              of: "${lorem.sentence}",
              prob: 0.6,
            },
          },
        },
        account: {
          username: "${internet.username}",
          createdAt: "${chrono.dateTime}",
          lastLoginAt: {
            optional: {
              of: "${chrono.dateTime}",
              prob: 0.8,
            },
          },
          isActive: "${boolean.boolean}",
          preferences: {
            theme: "light",
            notifications: "${boolean.boolean}",
            language: "${address.countryCode}",
          },
        },
      },
    },
  },
};
```

### Social Media Posts

```typescript
const socialMediaSchema = {
  $format: "jgd/v1",
  version: "1.0.0",
  seed: 456,
  entities: {
    users: {
      count: 25,
      fields: {
        id: "${index}",
        username: "${internet.username}",
        displayName: "${name.name}",
        followerCount: {
          number: {
            min: 0,
            max: 10000,
            integer: true,
          },
        },
      },
    },
    posts: {
      count: [50, 200],
      fields: {
        id: "${uuid.v4}",
        authorId: "${index(1)}",
        content: "${lorem.paragraphs}",
        hashtags: {
          array: {
            count: [0, 5],
            of: "#${lorem.word}",
          },
        },
        metrics: {
          likes: {
            number: {
              min: 0,
              max: 1000,
              integer: true,
            },
          },
          shares: {
            number: {
              min: 0,
              max: 100,
              integer: true,
            },
          },
          comments: {
            number: {
              min: 0,
              max: 50,
              integer: true,
            },
          },
        },
        publishedAt: "${chrono.dateTime}",
        media: {
          optional: {
            of: {
              array: {
                count: [1, 3],
                of: {
                  fields: {
                    // Must be an Entity with fields
                    url: "${internet.safeEmail}",
                    type: "image",
                  },
                },
              },
            },
            prob: 0.4,
          },
        },
      },
    },
  },
};
```

## 🔧 Best Practices

### Schema Organization

- Use semantic versioning for your schemas
- Add meaningful descriptions in comments
- Keep entity names descriptive and consistent
- Group related fields logically

### Performance Tips

- Use seeds for reproducible data in tests
- Prefer fixed counts over ranges for performance
- Limit deep nesting for better generation speed
- Use optional fields sparingly in large datasets

### Data Quality

- Choose appropriate faker patterns for realistic data
- Use constraints (min/max) to ensure valid ranges
- Test your schemas with small counts first
- Validate generated data structure matches expectations

### Maintainability

- Document custom patterns and context usage
- Use consistent naming conventions
- Version your schemas when making breaking changes
- Keep schemas focused on single domain concepts

Register your own data generation functions:

```typescript
import { addCustomKey } from 'jgd.js';

addCustomKey('custom.greeting', (args) => {
  const name = args.type === 'fixed' ? args.value : 'World';
  return { success: true, data: `Hello, ${name}!` };
});

// Use in schema
{
  "message": "${custom.greeting(TypeScript)}"
}
```

## API Reference

### Main Classes

```typescript
// Create JGD instance
const jgd = Jgd.from(schemaString);
const jgd = Jgd.fromFile("./schema.jgd");
const jgd = Jgd.fromObject(schemaObject);

// Generate data
const result = jgd.generate();

// Check result
if (result.success) {
  console.log(result.data);
} else {
  console.error(result.error);
}
```

### Utility Functions

```typescript
import {
  generateFromString,
  generateFromFile,
  isSuccess,
  isError,
  addCustomKey,
} from "jgd.js";

// Quick generation
const result = generateFromString(schemaString);

// Type guards
if (isSuccess(result)) {
  // result.data is available
}
if (isError(result)) {
  // result.error is available
}
```

### Configuration

```typescript
import { createGeneratorConfig } from "jgd.js";

const config = createGeneratorConfig("EN", 42);
// Locale: 'EN', Seed: 42
```

## Examples

Check the [`examples/`](../examples/) directory for complete schema examples:

- `single-user.jgd` - Simple root mode example
- `users-and-posts.jgd` - Complex entities with relationships

## Error Handling

All generation methods return `GenerationResult<T>`:

```typescript
type GenerationResult<T> =
  | { success: true; data: T }
  | { success: false; error: string };

const result = jgd.generate();
if (!result.success) {
  console.error("Generation failed:", result.error);
  return;
}

const data = result.data; // Safe to use
```

## 🔍 Troubleshooting & FAQ

### Common Issues

**Q: My schema generates different data each time**
A: Use the `seed` property for deterministic generation:

```typescript
{ "$format": "jgd/v1", "seed": 42, ... }
```

**Q: Context keys like `${index(2)}` return undefined**
A: Context depth is only created by entities with `count` property. Ensure parent entities have proper count specifications.

**Q: Optional fields appear too often/rarely**
A: Check your probability value - `0.7` means 70% chance:

```typescript
{ "optional": { "of": "...", "prob": 0.3 } } // 30% chance
```

**Q: Array generation fails with complex objects**
A: Arrays are designed for primitive values. For multiple complex objects, use entities with count:

```typescript
// ✅ Good - arrays for primitives
"tags": { "array": { "count": 3, "of": "${lorem.word}" } }
"scores": { "array": { "count": 5, "of": { "number": { "min": 1, "max": 100 } } } }

// ✅ Good - entities with count for multiple complex objects
"items": {
  "count": [2, 5],              // Generate 2-5 items
  "fields": {
    "id": "${index}",
    "name": "${name.name}",
    "email": "${internet.safeEmail}"
  }
}
```

**Q: Faker patterns not working**
A: Ensure proper syntax with `${}` and check pattern exists:

```typescript
"${name.firstName}"; // ✅ Correct
"name.firstName"; // ❌ Missing ${}
"${name.invalidPattern}"; // ❌ Pattern doesn't exist
```

### Performance Tips

- Use fixed counts instead of ranges for better performance
- Limit deep nesting levels (< 5 recommended)
- Cache `Jgd` instances for repeated generation
- Use smaller datasets for development/testing

### Debugging

Enable detailed error messages:

```typescript
const result = jgd.generate();
if (!result.success) {
  console.error("Generation failed:", result.error);
  // Check schema structure and field specifications
}
```

## TypeScript Support

Full TypeScript support with proper type definitions:

```typescript
import type {
  JgdSchema,
  EntitySpec,
  FieldSpec,
  NumberSpec,
  ArraySpec,
  CustomKeyFunction,
} from "jgd.js";
```

## Development

```bash
# Install dependencies
npm install

# Build
npm run build

# Test
npm test

# Watch mode
npm run build:watch
npm run test:watch
```

## License

MIT - See LICENSE file for details.

## Related Projects

- [jgd-rs](https://github.com/lvendrame/jgd-rs) - Original Rust implementation
- [@faker-js/faker](https://github.com/faker-js/faker) - Fake data generation
