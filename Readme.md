
# Zaporozhets JSON Parser

https://docs.rs/my_main_parser_kma_zaporozhetss/latest/my_main_parser_kma_zaporozhetss/

https://crates.io/crates/my_main_parser_kma_zaporozhetss


## Overview
Zaporozhets JSON Parser is a JSON processing tool written in Rust, capable of parsing JSON files, validating schemas, and converting JSON to various formats like YAML and XML. Built with Pest for grammar parsing, this parser supports the manipulation of JSON files with a command-line interface (CLI).

## Features
- Parse and validate JSON files against a provided schema.
- Extract specific JSON sections by key.
- Edit JSON data and save changes back to the file.
- Convert JSON to YAML or XML.
- Handle large JSON files in chunks.
- Search for values in JSON data.
- Access JSON elements by path.
- Minify JSON by removing whitespace.
- Display the structural hierarchy of JSON data.

## Technical Description
The parser utilizes a custom-defined Pest grammar file (`json.pest`) to interpret JSON structures. Key parsing rules are defined for JSON objects, arrays, strings, numbers, booleans, and null values, enabling support for typical JSON formats with whitespace tolerance and escape sequences in strings.

### Grammar Rules
1. **WHITESPACE**: Defines whitespace characters (space, tab, newline) and ignores them across the JSON structure.
2. **value**: The primary rule that includes objects, arrays, strings, numbers, booleans, and null values.
3. **object**: Defines a JSON object structure with key-value pairs, with optional commas between pairs.
4. **array**: Defines a JSON array structure with optional commas between values.

## JSON Structure Diagram
This diagram illustrates the general structure of JSON supported by this parser:
```
json
├── object
│   ├── pair
│   │   ├── string (key)
│   │   ├── value
│   │   │   ├── object
│   │   │   ├── array
│   │   │   ├── string
│   │   │   ├── number
│   │   │   ├── boolean
│   │   │   └── null
│   │   └── ...
├── array
│   ├── value
│   └── ...
└── other basic JSON values (string, number, boolean, null)
```
### Parsing Logic
1. **Objects**: Parsed as collections of key-value pairs enclosed in braces `{}`, where keys are strings and values can be any JSON type.
2. **Arrays**: Parsed as ordered collections of values enclosed in square brackets `[]`, allowing mixed types.
3. **Strings**: Parsed with support for escape sequences (e.g., `\n`, `\t`, Unicode).
4. **Numbers**: Parsed to support integer, float, and scientific notation formats.
5. **Booleans and Null**: Parsed as literals `true`, `false`, and `null`.

## CLI Usage
Run the parser through CLI commands, such as:
```bash
$ zaporozhets-json-parser validate <input> <schema>
$ zaporozhets-json-parser parse-partial <input> <key>
$ zaporozhets-json-parser edit <input> <key> <value>
```
Use `--help` for full command options.

### Example Commands
- `validate`: Validates JSON against a schema.
- `parse-partial`: Extracts a specified key's value.
- `edit`: Updates a key in the JSON.
- `convert`: Converts JSON to YAML or XML.
- `large-file`: Parses large JSON files in chunks.

## Setup
1. Ensure Rust is installed: [Rust Installation](https://www.rust-lang.org/tools/install)
2. Clone the repository and run:
   ```bash
   cargo build
   ```
3. Use CLI commands as described.

## Error Handling
Error handling is implemented with `anyhow` for flexible context-based error reporting, and `thiserror` for custom error types like `JsonParseError` and `SchemaValidationError`.

## Testing and Quality Assurance
- **Unit Tests**: Located in the `tests` directory, covering each grammar rule.
- **Formatting and Linting**: Run `cargo fmt` and `cargo clippy` to maintain code quality.

## Makefile
The `Makefile` includes commands to simplify running, testing, formatting, and linting the project:
```bash
run:
	cargo run
release:
	cargo run --release
build:
	cargo build
build-release:
	cargo build --release
test:
	cargo test
format:
	cargo fmt
lint:
	cargo clippy -- -D warnings
precommit: format lint test
clean:
	cargo clean
update:
	cargo update
```

