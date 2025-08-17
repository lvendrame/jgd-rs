# JGD - JSON Generator Definition

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

An ecosystem for generating realistic JSON data using declarative schema definitions. This repository contains both the core library and command-line tool for working with JGD (JSON Generator Definition) schemas.

## 📦 Components

### [JGD Rust Library](./jgd-rs/)

The core Rust library that powers JSON data generation. See [jgd-rs README](./jgd-rs/README.md) for detailed documentation, API reference, and usage examples.

### [JGD CLI Tool](./jgd-rs-cli/)

A command-line interface for generating JSON data from JGD schema files.

### [JGD JavaScript Library](./jgd.js/)

The JavaScript library that powers JSON data generation. See [jgd.js README](./jgd.js/README.md) for detailed documentation, API reference, and usage examples.

## 🎯 What is JGD?

JGD (JSON Generator Definition) is a declarative schema format for generating realistic fake JSON data. It allows you to:

- Define complex data structures with relationships
- Generate deterministic or random data sets
- Create cross-references between entities
- Use faker patterns for realistic data
- Support multiple locales and languages
- Generate arrays with flexible counts
- Create optional fields with probability control

## 📋 Schema Overview

JGD schemas are JSON documents that describe how to generate data. Here's the basic structure:

```json
{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "seed": 42,
  "defaultLocale": "EN",
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
      }
    }
  }
}
```

### Schema Validation

The repository includes a comprehensive JSON Schema for validation:

📄 **[JGD Schema Definition](./schema/jgd.schema.json)**

This schema can be used with any JSON Schema validator or IDE that supports schema validation to ensure your JGD files are correctly formatted.

### Schema URL for IDE Integration

You can reference the schema directly in your JGD files for IDE support:

```json
{
  "$schema": "https://raw.githubusercontent.com/lvendrame/jgd-rs/refs/heads/main/schema/jgd.schema.json",
  "$format": "jgd/v1",
  "version": "1.0.0",
  ...
}
```

## 🚀 Quick Start

### Using the Library

Add to your `Cargo.toml`:

```toml
[dependencies]
jgd-rs = "0.1.0"
```

Generate data in your Rust code:

```rust
use jgd_rs::generate_jgd_from_str;

let schema = r#"{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "root": {
    "fields": {
      "name": "${name.firstName}",
      "email": "${internet.safeEmail}"
    }
  }
}"#;

let json_data = generate_jgd_from_str(schema);
println!("{}", serde_json::to_string_pretty(&json_data).unwrap());
```

### Using the CLI Tool

Build and run the CLI tool:

```bash
# Build the project
cargo build --release

# Run with a schema file
./target/release/jgd-rs-cli path/to/schema.jgd
```

## 📖 Schema Documentation

### Core Concepts

#### 1. **Root Mode vs Entities Mode**

- **Root Mode**: Generate a single entity (object or array)
- **Entities Mode**: Generate multiple named entities with relationships

#### 2. **Field Types**

- **Primitives**: Strings, numbers, booleans, null
- **Template Strings**: `"${faker.pattern}"` for dynamic data
- **Number Generation**: Min/max ranges with integer/float support
- **Arrays**: Variable or fixed-count collections
- **Optional Fields**: Probability-based field inclusion
- **Cross-references**: Link data between entities
- **Nested Objects**: Complex hierarchical structures

#### 3. **Faker Integration**

Over 100+ faker patterns across categories:

- **Address**: Cities, countries, coordinates, postal codes
- **Names**: First/last names, titles, full names
- **Internet**: Emails, domains, IPs, user agents
- **Company**: Business names, buzzwords, industries
- **Lorem**: Words, sentences, paragraphs
- **Time/Date**: Timestamps, durations, date ranges
- **Finance**: Credit cards, currencies, banking codes
- **And many more...**

### Schema Structure

#### Required Fields

- `$format`: Always "jgd/v1"
- `version`: User-defined schema version
- Either `root` OR `entities` (mutually exclusive)

#### Optional Fields

- `seed`: Random seed for deterministic generation
- `defaultLocale`: Locale for faker data (EN, FR_FR, DE_DE, etc.)

### Example Schemas

#### Simple User Profile

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
      "active": true
    }
  }
}
```

#### Multi-Entity Blog System

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

## 📂 Example Files

The repository includes several example schema files:

- [`single-object-root.jgd`](./examples/single-object-root.jgd) - Simple single object generation
- [`array-object-root.jgd`](./examples/array-object-root.jgd) - Array of objects generation
- [`ranged-array-object-root.jgd`](./examples/ranged-array-object-root.jgd) - Variable count arrays
- [`user-post-entities.jgd`](./examples/user-post-entities.jgd) - Multi-entity relationships

## 🔧 Development

### Building the Project

```bash
# Build all components
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test

# Run specific component
cargo run --bin jgd-rs-cli -- examples/single-object-root.jgd
```

### Project Structure

```
jgd-rs/
├── jgd-rs/              # Core library
│   ├── src/              # Library source code
│   ├── examples/         # Example JGD schema files
│   ├── schema/           # JSON Schema definition
│   └── README.md         # Detailed library documentation
├── jgd-rs-cli/           # Command-line tool
│   └── src/              # CLI source code
└── README.md             # This file
```

## 🌍 Supported Locales

Generate locale-specific data with these supported locales:

- `EN` - English (default)
- `FR_FR` - French (France)
- `DE_DE` - German (Germany)
- `IT_IT` - Italian (Italy)
- `PT_BR` - Portuguese (Brazil)
- `JA_JP` - Japanese (Japan)
- `AR_SA` - Arabic (Saudi Arabia)
- `CY_GB` - Welsh (Great Britain)

## 📚 Documentation

- **[Library Documentation](./jgd-rs/README.md)** - Complete API reference and usage guide
- **[JSON Schema](./jgd-rs/schema/jgd.schema.json)** - Formal schema definition for validation
- **[Examples](./jgd-rs/examples/)** - Sample JGD schema files

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Related Tools

### IDE Integration

For the best development experience, use an IDE that supports JSON Schema validation:

- **VS Code**: Install a JSON Schema extension
- **IntelliJ IDEA**: Built-in JSON Schema support
- **Vim/Neovim**: Use a JSON Schema plugin

Add the schema reference to your JGD files:

```json
{
  "$schema": "https://raw.githubusercontent.com/lvendrame/jgd-rs/refs/heads/main/jgd-rs/schema/jgd.schema.json",
  "$format": "jgd/v1",
  ...
}
```

### Online Validators

You can validate your JGD schemas using online JSON Schema validators:

- [JSONSchemaLint](https://jsonschemalint.com/)
- [JSON Schema Validator](https://www.jsonschemavalidator.net/)

Just paste the [schema URL](https://raw.githubusercontent.com/lvendrame/jgd-rs/refs/heads/main/jgd-rs/schema/jgd.schema.json) and your JGD content.
