# JGD.js - Project Structure

## Overview

A complete TypeScript implementation of the JGD (JSON Generator Definition) library, mirroring the Rust architecture with TypeScript best practices.

## Project Structure

```
jgd.js/
├── src/
│   ├── index.ts              # Main exports and public API
│   ├── types.ts              # TypeScript type definitions
│   ├── jgd.ts               # Main JGD class (like jgd.rs)
│   ├── config.ts            # Configuration management
│   ├── utils.ts             # Utility functions
│   ├── replacer.ts          # Pattern replacement logic
│   ├── template.ts          # Template string processing
│   ├── number-spec.ts       # Number generation (like number_spec.rs)
│   ├── array-spec.ts        # Array generation (like array_spec.rs)
│   ├── optional-spec.ts     # Optional field wrapper
│   ├── field-generator.ts   # Field generation dispatcher
│   └── entity-generator.ts  # Entity generation (like entity.rs)
├── __tests__/
│   └── jgd.test.ts         # Comprehensive test suite
├── examples/
│   ├── single-user.jgd     # Simple root mode example
│   └── users-and-posts.jgd # Complex entities example
├── dist/                   # Compiled JavaScript output
├── package.json           # NPM package configuration
├── tsconfig.json          # TypeScript configuration
├── jest.config.js         # Jest test configuration
├── README.md              # Documentation
└── demo.ts               # Demo script

```

## Architecture Mapping from Rust

| Rust Module                      | TypeScript Module                     | Purpose                  |
| -------------------------------- | ------------------------------------- | ------------------------ |
| `src/lib.rs`                     | `src/index.ts`                        | Main library exports     |
| `src/type_spec/jgd.rs`           | `src/jgd.ts`                          | Core JGD schema handling |
| `src/type_spec/mod.rs`           | `src/types.ts`                        | Type definitions         |
| `src/type_spec/entity.rs`        | `src/entity-generator.ts`             | Entity generation        |
| `src/type_spec/field.rs`         | `src/field-generator.ts`              | Field generation         |
| `src/type_spec/number_spec.rs`   | `src/number-spec.ts`                  | Number generation        |
| `src/type_spec/array_spec.rs`    | `src/array-spec.ts`                   | Array generation         |
| `src/type_spec/optional_spec.rs` | `src/optional-spec.ts`                | Optional fields          |
| `src/type_spec/utils/`           | `src/utils.ts`                        | Utility functions        |
| `src/fake/`                      | `src/replacer.ts` + `src/template.ts` | Fake data generation     |

## Key Features Implemented

### ✅ Core Functionality

- [x] JGD schema parsing and validation
- [x] Root mode and entities mode support
- [x] Deterministic generation with seeds
- [x] Error handling with Result types
- [x] Full TypeScript type safety

### ✅ Data Generation

- [x] Number specifications (integer/float, ranges)
- [x] Array specifications (primitives only)
- [x] Optional field specifications
- [x] Nested entity structures
- [x] Field and entity naming

### ✅ Context-Aware Keys

- [x] `${index}` - Current item index
- [x] `${index(n)}` - Parent depth index
- [x] `${count}` - Current count
- [x] `${entity.name}` - Current entity name
- [x] `${field.name}` - Current field name

### ✅ Faker Integration

- [x] Faker.js v9.9.0 integration for realistic data
- [x] Pattern parsing: `${category.method}`
- [x] Argument support: `${method(arg1,arg2)}`
- [x] Template string processing
- [x] Updated to use modern API (`string.uuid` instead of deprecated `datatype.uuid`)

### ✅ Custom Keys

- [x] Global custom key registration
- [x] Custom function support
- [x] Arguments handling

### ✅ Developer Experience

- [x] Comprehensive test suite (16 tests passing)
- [x] TypeScript declarations
- [x] Documentation and examples
- [x] Demo script
- [x] Error handling and validation

## Usage Examples

### Basic Schema

```typescript
import { Jgd } from "jgd.js";

const schema = {
  $format: "jgd/v1",
  version: "1.0.0",
  root: {
    fields: {
      name: "${person.firstName}",
      age: { min: 18, max: 65, integer: true },
    },
  },
};

const jgd = Jgd.fromObject(schema);
const result = jgd.generate();
```

### Context-Aware Generation

```typescript
const schema = {
  $format: "jgd/v1",
  version: "1.0.0",
  entities: {
    users: {
      count: 3,
      fields: {
        id: "${index}",
        summary: "User ${index} of ${count}",
        posts: {
          count: 2,
          fields: {
            userId: "${index(2)}", // References parent user index
            title: "${lorem.sentence}",
          },
        },
      },
    },
  },
};
```

### Custom Keys

```typescript
import { addCustomKey } from "jgd.js";

addCustomKey("custom.status", (args) => {
  const statuses = ["active", "inactive"];
  return { success: true, data: statuses[Math.floor(Math.random() * 2)] };
});
```

## TypeScript Best Practices Applied

1. **Strong Typing**: Full type safety with proper interfaces
2. **Result Types**: Error handling with `GenerationResult<T>`
3. **Modular Architecture**: Clean separation of concerns
4. **Type Guards**: Runtime type checking with `isSuccess`/`isError`
5. **Generic Types**: Flexible type parameters
6. **Readonly Properties**: Immutable data structures
7. **Optional Chaining**: Safe property access
8. **Union Types**: Flexible type definitions

## Performance Considerations

- Lazy evaluation where possible
- Efficient template processing
- Minimal object creation
- Proper memory management
- Configurable depth limits

## Testing

- 16 comprehensive unit tests
- 100% core functionality coverage
- Error case testing
- Type safety validation
- Integration testing

This TypeScript implementation successfully replicates all the core functionality of the Rust JGD library while following TypeScript and Node.js best practices.
