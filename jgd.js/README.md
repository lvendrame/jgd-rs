# JGD.js - JSON Generator Definition for TypeScript

A TypeScript library for generating realistic JSON data using declarative schema definitions. Based on the Rust JGD library architecture with TypeScript best practices.

## Features

- 🎯 **Declarative Schema**: Define data generation rules in JSON format
- 🔢 **Deterministic Generation**: Use seeds for reproducible results
- 🌐 **Faker Integration**: Built-in support for realistic fake data
- 🔧 **Custom Keys**: Register your own data generation functions
- 📊 **Context-Aware**: Access index, count, entity names in generation
- ⚡ **TypeScript Native**: Full type safety and IntelliSense support
- 🧪 **Well Tested**: Comprehensive test suite

## Installation

```bash
npm install jgd.js
```

## Quick Start

### Basic Usage

```typescript
import { Jgd } from "jgd.js";

const schema = {
  $format: "jgd/v1",
  version: "1.0.0",
  root: {
    fields: {
      name: "${person.firstName}",
      age: { min: 18, max: 65, integer: true },
      email: "${internet.email}",
    },
  },
};

const jgd = Jgd.fromObject(schema);
const result = jgd.generate();

if (result.success) {
  console.log(result.data);
  // Output: { name: "John", age: 32, email: "john@example.com" }
}
```

### Multiple Entities

```typescript
const schema = {
  $format: "jgd/v1",
  version: "1.0.0",
  entities: {
    users: {
      count: 3,
      fields: {
        id: "${index}",
        name: "${person.fullName}",
        posts: {
          count: { range: [1, 3] },
          fields: {
            id: "${string.uuid}",
            userId: "${index(2)}", // Reference parent user index
            title: "${lorem.sentence}",
          },
        },
      },
    },
  },
};
```

## Schema Structure

### Root Mode

Single entity generation:

```json
{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "seed": 42,
  "root": {
    "count": 5,
    "fields": {
      "id": "${index}",
      "name": "Item ${index}"
    }
  }
}
```

### Entities Mode

Multiple named entities:

```json
{
  "$format": "jgd/v1",
  "version": "1.0.0",
  "entities": {
    "users": {
      "count": 10,
      "fields": {
        "name": "${person.firstName}"
      }
    },
    "posts": {
      "count": 20,
      "fields": {
        "title": "${lorem.sentence}"
      }
    }
  }
}
```

## Field Types

### Primitive Values

```typescript
{
  "name": "John Doe",        // String literal
  "age": 30,                 // Number literal
  "active": true             // Boolean literal
}
```

### Number Specifications

```typescript
{
  "integer_field": {
    "min": 1,
    "max": 100,
    "integer": true
  },
  "float_field": {
    "min": 0.0,
    "max": 1.0
  }
}
```

### Array Specifications

Arrays are for primitive values only:

```typescript
{
  "tags": {
    "array": {
      "count": 3,
      "of": "${lorem.word}"
    }
  },
  "scores": {
    "array": {
      "count": { "range": [1, 5] },
      "of": { "min": 0, "max": 100, "integer": true }
    }
  }
}
```

### Optional Fields

```typescript
{
  "optional_field": {
    "optional": "${lorem.sentence}",
    "probability": 0.7  // 70% chance of being present
  }
}
```

### Nested Objects

Use entity specifications for complex objects:

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

## Faker Patterns

JGD.js integrates with [@faker-js/faker](https://github.com/faker-js/faker) for realistic data:

```typescript
{
  "person_name": "${person.firstName}",
  "full_name": "${person.fullName}",
  "email": "${internet.email}",
  "phone": "${phone.number}",
  "address": "${location.streetAddress}",
  "company": "${company.name}",
  "sentence": "${lorem.sentence(5,10)",    // 5-10 words
  "number": "${number.int(1,100)}",        // 1-100
  "date": "${date.recent}",
  "uuid": "${string.uuid}"
}
```

## Context-Aware Keys

### Index Keys

```typescript
{
  "id": "${index}",          // Current item index: 1, 2, 3...
  "parent_id": "${index(2)}", // Parent entity index (depth 2)
  "summary": "${index} of ${count}"
}
```

### Entity and Field Context

```typescript
{
  "entity_name": "${entity.name}",  // Current entity name
  "field_name": "${field.name}",    // Current field name
  "context": "Field ${field.name} in ${entity.name}"
}
```

**Important**: Depth levels are only created by entities with `count` property.

## Custom Keys

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

Check the `examples/` directory for complete schema examples:

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
