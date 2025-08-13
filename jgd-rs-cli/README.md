# JGD-rs CLI - Command Line Tool for JSON Generation

A command-line interface for generating JSON data from JGD (JSON Generator Definition) schema files.

## Overview

The JGD-rs CLI tool allows you to generate realistic JSON data from declarative schema definitions directly from the command line. It's built on top of the [jgd-rs](../jgd-rs/) library and provides a simple interface for batch JSON generation, testing, and data seeding.

## Installation

### From Source

Clone the repository and build the CLI tool:

```bash
git clone https://github.com/lvendrame/jgd-rs.git
cd jgd-rs
cargo build --release
```

The binary will be available at `./target/release/jgd-rs-cli`.

### Using Cargo

```bash
cargo install --path jgd-rs-cli
```

## Usage

```bash
jgd-rs-cli [OPTIONS] <INPUT>
```

### Arguments

- `<INPUT>` - Path to the .jgd schema file

### Options

- `-o, --out <FILE>` - Output file (JSON). If omitted, prints to stdout
- `--seed <SEED>` - Seed override for deterministic generation
- `-p, --pretty` - Pretty print the JSON output
- `-h, --help` - Print help information
- `-V, --version` - Print version information

## Examples

### Basic Usage

Generate JSON data and print to stdout:

```bash
jgd-rs-cli schema.jgd
```

### Output to File

Generate JSON data and save to a file:

```bash
jgd-rs-cli schema.jgd -o output.json
```

### Pretty Print Output

Generate formatted JSON with proper indentation:

```bash
jgd-rs-cli schema.jgd --pretty
```

### Using Custom Seed

Generate deterministic data with a specific seed:

```bash
jgd-rs-cli schema.jgd --seed 42 --pretty
```

### Complete Example

```bash
jgd-rs-cli examples/user-post-entities.jgd --seed 12345 --pretty --out generated-data.json
```

This command:

- Uses the `user-post-entities.jgd` schema file
- Sets seed to 12345 for reproducible output
- Formats the JSON with pretty printing
- Saves the result to `generated-data.json`

## Sample Schema Files

The repository includes several example schema files you can use:

```bash
# Generate a single user object
jgd-rs-cli ../examples/single-object-root.jgd --pretty

# Generate an array of objects
jgd-rs-cli ../examples/array-object-root.jgd --pretty

# Generate complex multi-entity data
jgd-rs-cli ../examples/user-post-entities.jgd --pretty
```

## Sample Output

### Input Schema (`user.jgd`)

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

### Command

```bash
jgd-rs-cli user.jgd --pretty
```

### Output

```json
{
  "id": "01HQCR5K2X3QP2M9T4N6B7V8Z0",
  "name": "Alice Johnson",
  "email": "alice.johnson@example.org",
  "age": 29,
  "city": "Springfield",
  "active": true
}
```

## Use Cases

### 1. API Testing

Generate test data for API endpoints:

```bash
# Generate user test data
jgd-rs-cli schemas/user.jgd -o test-data/users.json

# Generate product catalog
jgd-rs-cli schemas/products.jgd --seed 100 -o test-data/products.json
```

### 2. Database Seeding

Create seed data for development databases:

```bash
# Generate 1000 users with relationships
jgd-rs-cli schemas/large-dataset.jgd --seed 42 -o seed-data.json
```

### 3. Mock Data Generation

Create realistic mock data for frontend development:

```bash
# Generate blog posts with authors
jgd-rs-cli schemas/blog-system.jgd --pretty -o frontend/mock-data.json
```

### 4. Data Pipeline Testing

Generate consistent test data for data processing pipelines:

```bash
# Generate reproducible test data
jgd-rs-cli schemas/pipeline-input.jgd --seed 123 -o pipeline-test-input.json
```

## Integration with Build Tools

### Makefile Integration

```makefile
.PHONY: generate-test-data
generate-test-data:
	jgd-rs-cli schemas/users.jgd --seed 42 -o test-data/users.json
	jgd-rs-cli schemas/products.jgd --seed 42 -o test-data/products.json
	jgd-rs-cli schemas/orders.jgd --seed 42 -o test-data/orders.json

.PHONY: clean-test-data
clean-test-data:
	rm -rf test-data/*.json
```

### npm/package.json Scripts

```json
{
  "scripts": {
    "generate:mockdata": "jgd-rs-cli schemas/frontend-data.jgd --pretty -o src/mock-data.json",
    "generate:testdata": "jgd-rs-cli schemas/test-data.jgd --seed 42 -o tests/fixtures/data.json"
  }
}
```

### Docker Integration

```dockerfile
FROM rust:1.70 as builder
COPY . /app
WORKDIR /app
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/jgd-rs-cli /usr/local/bin/
COPY schemas/ /schemas/
CMD ["jgd-rs-cli", "/schemas/default.jgd", "--pretty"]
```

## Performance Considerations

- **Large Datasets**: For generating large amounts of data, consider using range counts in your schemas rather than fixed large numbers
- **Memory Usage**: The tool loads the entire generated dataset into memory before output
- **Deterministic Generation**: Using seeds ensures reproducible output but may be slightly slower than random generation
- **File I/O**: Writing to files is generally faster than stdout for large datasets

## Troubleshooting

### Common Issues

#### Invalid Schema Format

```bash
Error: Schema validation failed
```

**Solution**: Ensure your schema follows the JGD format. See the [schema documentation](../schema/jgd.schema.json).

#### File Not Found

```bash
Error: No such file or directory
```

**Solution**: Check that the input file path is correct and the file exists.

#### Permission Denied

```bash
Error: Permission denied (os error 13)
```

**Solution**: Ensure you have read permissions for the input file and write permissions for the output directory.

### Debug Mode

For troubleshooting, you can use Rust's built-in logging:

```bash
RUST_LOG=debug jgd-rs-cli schema.jgd --pretty
```

## Schema Validation

The CLI tool automatically validates input schemas against the JGD specification. For additional validation, you can use external JSON Schema validators with the [JGD schema definition](../jgd-rs/schema/jgd.schema.json).

## Exit Codes

- `0` - Success
- `1` - Error (invalid schema, file not found, etc.)

## Related Documentation

- **[JGD Library Documentation](../jgd-rs/README.md)** - Complete API reference and schema documentation
- **[JSON Schema Definition](../schema/jgd.schema.json)** - Formal schema specification
- **[Example Schemas](../examples/)** - Sample JGD files to get started

## Contributing

Contributions are welcome! To contribute to the CLI tool:

1. Fork the repository
2. Create a feature branch
3. Make your changes to the CLI code
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
