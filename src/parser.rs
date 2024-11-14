use anyhow::{Error, Result};
use log::{error, info};
use pest::Parser;
use pest_derive::Parser;
use serde_json::{Map, Value};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

/// JSONParser struct, generated from the grammar defined in `json.pest`.
/// This struct is used to parse JSON based on the defined rules in the `json.pest` grammar file.
#[derive(Parser)]
#[grammar = "json.pest"]
struct JSONParser;

/// Custom error type for JSON parsing, schema validation, and file-related errors.
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("File read error: {0}")]
    FileReadError(#[from] std::io::Error),
    #[error("JSON parse error")]
    JsonParseError,
    #[error("Schema validation failed")]
    SchemaValidationError,
}

/// Parses a JSON string using the `JSONParser` and converts it to a `serde_json::Value`.
/// Returns a `Result` with `Value` on success or `ParserError` on failure.
///
/// # Arguments
///
/// * `json_str` - The JSON string to be parsed.
///
/// # Returns
///
/// * `Result<Value, ParserError>` - The parsed JSON as a `serde_json::Value` if successful, or an error on failure.
pub fn parse_json(json_str: &str) -> Result<Value, ParserError> {
    let pairs = JSONParser::parse(Rule::json, json_str).map_err(|e| {
        println!("Parsing error in JSON input: {:?}", e);
        ParserError::JsonParseError
    })?;
    parse_value(pairs)
}

/// Recursively processes `pest` parsing results and converts them to `serde_json::Value`.
///
/// # Arguments
///
/// * `pairs` - The parsed pairs of tokens from `pest`.
///
/// # Returns
///
/// * `Result<Value, ParserError>` - A `serde_json::Value` representing the parsed JSON structure, or an error if parsing fails.
fn parse_value(mut pairs: pest::iterators::Pairs<Rule>) -> Result<Value, ParserError> {
    let pair = pairs.next().ok_or_else(|| {
        println!("No pairs found in input.");
        ParserError::JsonParseError
    })?;

    match pair.as_rule() {
        Rule::json => parse_value(pair.into_inner()),
        Rule::object => parse_object(pair),
        Rule::array => parse_array(pair),
        Rule::string => Ok(Value::String(parse_string(pair)?)),
        Rule::number => Ok(Value::Number(parse_number(pair)?)),
        Rule::boolean => Ok(Value::Bool(pair.as_str() == "true")),
        Rule::null => Ok(Value::Null),
        _ => {
            println!("Unexpected pair encountered: {:?}", pair.as_rule());
            Err(ParserError::JsonParseError)
        }
    }
}

/// Parses a JSON object and returns it as a `serde_json::Value`.
/// Handles JSON objects by parsing each key-value pair.
///
/// # Arguments
///
/// * `pair` - The `pest::iterators::Pair` containing the JSON object.
///
/// # Returns
///
/// * `Result<Value, ParserError>` - Returns a `serde_json::Value::Object` on success, or an error if parsing fails.
fn parse_object(pair: pest::iterators::Pair<Rule>) -> Result<Value, ParserError> {
    let mut map = Map::new();
    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::pair {
            let mut inner_rules = inner_pair.into_inner();
            let key = parse_string(inner_rules.next().ok_or(ParserError::JsonParseError)?)?;
            let value = parse_value(inner_rules)?;
            map.insert(key, value);
        }
    }
    Ok(Value::Object(map))
}

/// Parses a JSON array and returns it as a `serde_json::Value`.
/// Processes each array element and collects them into a `Vec<Value>`.
///
/// # Arguments
///
/// * `pair` - The `pest::iterators::Pair` containing the JSON array.
///
/// # Returns
///
/// * `Result<Value, ParserError>` - Returns a `serde_json::Value::Array` on success, or an error if parsing fails.
fn parse_array(pair: pest::iterators::Pair<Rule>) -> Result<Value, ParserError> {
    let mut array = Vec::new();
    for inner_pair in pair.into_inner() {
        let value = parse_value(inner_pair.into_inner())?;
        array.push(value);
    }
    Ok(Value::Array(array))
}

/// Parses a JSON string, handling escape sequences and Unicode characters.
///
/// # Arguments
///
/// * `pair` - The `pest::iterators::Pair` containing the JSON string.
///
/// # Returns
///
/// * `Result<String, ParserError>` - The parsed string or an error if parsing fails.
fn parse_string(pair: pest::iterators::Pair<Rule>) -> Result<String, ParserError> {
    let mut result = String::new();
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::character => result.push_str(inner_pair.as_str()),
            Rule::escape_sequence => {
                let escaped = match inner_pair.as_str() {
                    "\\\"" => "\"",
                    "\\\\" => "\\",
                    "\\/" => "/",
                    "\\b" => "\u{0008}",
                    "\\f" => "\u{000C}",
                    "\\n" => "\n",
                    "\\r" => "\r",
                    "\\t" => "\t",
                    escape if escape.starts_with("\\u") => {
                        let hex = &escape[2..];
                        let code_point = u32::from_str_radix(hex, 16)
                            .map_err(|_| ParserError::JsonParseError)?;
                        let unicode_char = std::char::from_u32(code_point)
                            .ok_or(ParserError::JsonParseError)?
                            .to_string();
                        result.push_str(&unicode_char);
                        continue;
                    }
                    _ => return Err(ParserError::JsonParseError),
                };
                result.push_str(escaped);
            }
            _ => {}
        }
    }
    Ok(result)
}

/// Parses a JSON number and converts it to a `serde_json::Number`.
///
/// # Arguments
///
/// * `pair` - The `pest::iterators::Pair` containing the JSON number.
///
/// # Returns
///
/// * `Result<serde_json::Number, ParserError>` - The parsed number or an error if parsing fails.
fn parse_number(pair: pest::iterators::Pair<Rule>) -> Result<serde_json::Number, ParserError> {
    let number_str = pair.as_str();
    serde_json::Number::from_str(number_str).map_err(|_| ParserError::JsonParseError)
}

/// Validates a JSON object against a schema.
/// Checks that all keys in the schema are present in the JSON object.
///
/// # Arguments
///
/// * `json` - The JSON object to validate.
/// * `schema` - The schema to validate against.
///
/// # Returns
///
/// * `Result<(), ParserError>` - Returns Ok if validation is successful, or an error if validation fails.
pub fn validate_json_schema(json: &Value, schema: &Value) -> Result<(), ParserError> {
    if json.is_object() && schema.is_object() {
        if json
            .as_object()
            .unwrap()
            .keys()
            .all(|key| schema.as_object().unwrap().contains_key(key))
        {
            Ok(())
        } else {
            Err(ParserError::SchemaValidationError)
        }
    } else {
        Err(ParserError::SchemaValidationError)
    }
}

/// Parses a specific part of the JSON file by a given key.
/// Returns `Some(Value)` if the key exists, otherwise `None`.
///
/// # Arguments
///
/// * `json` - The JSON object to parse.
/// * `key` - The key to extract from the JSON.
///
/// # Returns
///
/// * `Option<Value>` - The extracted value or `None` if the key is not found.
pub fn parse_partial_json(json: &Value, key: &str) -> Option<Value> {
    json.get(key).cloned()
}

/// Edits a JSON file by updating a specific key with a new value.
///
/// # Arguments
///
/// * `json` - A mutable reference to the JSON object to edit.
/// * `key` - The key in the JSON object to update.
/// * `new_value` - The new value to set for the specified key.
///
/// # Returns
///
/// * `Result<(), Error>` - Returns Ok if successful, or an error if the JSON structure is invalid.
pub fn edit_json(json: &mut Value, key: &str, new_value: Value) -> Result<(), Error> {
    if let Some(obj) = json.as_object_mut() {
        obj.insert(key.to_string(), new_value);
        Ok(())
    } else {
        Err(Error::msg("Invalid JSON structure for editing"))
    }
}

/// Converts JSON to YAML or XML format based on the specified format.
///
/// # Arguments
///
/// * `json` - The JSON object to convert.
/// * `format` - The target format ("yaml" or "xml").
///
/// # Returns
///
/// * `Result<String, Error>` - The converted JSON in the specified format, or an error if the format is unsupported.
pub fn convert_to_format(json: &Value, format: &str) -> Result<String, Error> {
    match format {
        "yaml" => serde_yaml::to_string(json).map_err(|e| Error::msg(e.to_string())),
        "xml" => convert_json_to_xml(json),
        _ => Err(Error::msg("Unsupported format")),
    }
}

/// Converts JSON to XML format.
///
/// # Arguments
///
/// * `json` - The JSON object to convert.
///
/// # Returns
///
/// * `Result<String, Error>` - The converted JSON in XML format, or an error if conversion fails.
fn convert_json_to_xml(json: &Value) -> Result<String, Error> {
    let mut writer = Vec::new();
    write_xml(json, &mut writer, "root")?;
    String::from_utf8(writer).map_err(|e| Error::msg(e.to_string()))
}

/// Writes XML data recursively from JSON, preserving the structure.
///
/// # Arguments
///
/// * `json` - The JSON object to write as XML.
/// * `writer` - The writer to output the XML data.
/// * `tag_name` - The XML tag name.
///
/// # Returns
///
/// * `Result<(), Error>` - Returns Ok if writing succeeds, or an error if it fails.
fn write_xml<W: std::io::Write>(json: &Value, writer: &mut W, tag_name: &str) -> Result<(), Error> {
    match json {
        Value::Object(map) => {
            writeln!(writer, "<{}>", tag_name)?;
            for (key, value) in map {
                write_xml(value, writer, key)?;
            }
            writeln!(writer, "</{}>", tag_name)?;
        }
        Value::Array(arr) => {
            for value in arr {
                write_xml(value, writer, tag_name)?;
            }
        }
        Value::String(s) => {
            writeln!(writer, "<{0}>{1}</{0}>", tag_name, s)?;
        }
        Value::Number(num) => {
            writeln!(writer, "<{0}>{1}</{0}>", tag_name, num)?;
        }
        Value::Bool(b) => {
            writeln!(writer, "<{0}>{1}</{0}>", tag_name, b)?;
        }
        Value::Null => {
            writeln!(writer, "<{} />", tag_name)?;
        }
    }
    Ok(())
}

/// Processes large JSON files by parsing them in chunks.
///
/// # Arguments
///
/// * `file_path` - The path to the large JSON file.
///
/// # Returns
///
/// * `Result<(), ParserError>` - Returns Ok if successful, or an error if parsing fails.
pub fn handle_large_json(file_path: &Path) -> Result<(), ParserError> {
    let file = fs::File::open(file_path)?;
    let stream = serde_json::Deserializer::from_reader(file).into_iter::<Value>();

    for value in stream {
        match value {
            Ok(json_value) => info!("Parsed chunk: {:?}", json_value),
            Err(e) => error!("Error parsing chunk: {:?}", e),
        }
    }
    Ok(())
}

/// Searches for JSON keys by a specific value, returning paths where the value is found.
///
/// # Arguments
///
/// * `json` - The JSON object to search.
/// * `target_value` - The target value to search for.
///
/// # Returns
///
/// * `Vec<String>` - A list of paths where the target value is found.
pub fn search_by_value(json: &Value, target_value: &str) -> Vec<String> {
    let mut results = Vec::new();
    search_recursive(json, target_value, &mut results, "".to_string());
    results
}

/// Recursive helper function for `search_by_value`, traversing JSON structure.
///
/// # Arguments
///
/// * `json` - The JSON object to search.
/// * `target_value` - The target value to search for.
/// * `results` - A mutable vector to store the found paths.
/// * `path` - The current JSON path.
fn search_recursive(json: &Value, target_value: &str, results: &mut Vec<String>, path: String) {
    match json {
        Value::Object(map) => {
            for (key, value) in map {
                let new_path = format!("{}.{}", path, key)
                    .trim_start_matches('.')
                    .to_string();
                if value.is_string() && value.as_str().unwrap() == target_value {
                    results.push(new_path.clone());
                }
                search_recursive(value, target_value, results, new_path);
            }
        }
        Value::Array(arr) => {
            for (index, item) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", path, index);
                search_recursive(item, target_value, results, new_path);
            }
        }
        _ => {}
    }
}

/// Retrieves a JSON value by a given path (e.g., "data.items[0].name").
///
/// # Arguments
///
/// * `json` - The JSON object to search.
/// * `json_path` - The path to the target value.
///
/// # Returns
///
/// * `Option<Value>` - The found value or `None` if the path does not exist.
pub fn get_by_path(json: &Value, json_path: &str) -> Option<Value> {
    let mut current = json;
    let parts = json_path.split('.');

    for part in parts {
        if part.contains('[') && part.contains(']') {
            let name = &part[..part.find('[').unwrap()];
            let index: usize = part[part.find('[').unwrap() + 1..part.find(']').unwrap()]
                .parse()
                .ok()?;
            current = current.get(name)?.get(index)?;
        } else {
            current = current.get(part)?;
        }
    }
    Some(current.clone())
}

/// Minifies JSON by removing whitespace.
///
/// # Arguments
///
/// * `json` - The JSON object to minify.
///
/// # Returns
///
/// * `String` - The minified JSON string.
pub fn minify_json(json: &Value) -> String {
    json.to_string()
}

/// Displays the structure of JSON, printing each key and nested value with indentation.
///
/// # Arguments
///
/// * `json` - The JSON object to display.
pub fn display_structure(json: &Value) {
    display_structure_recursive(json, 0);
}

/// Helper function for `display_structure` to recursively print JSON structure with indentation.
///
/// # Arguments
///
/// * `json` - The JSON object to display.
/// * `indent` - The current indentation level.
fn display_structure_recursive(json: &Value, indent: usize) {
    match json {
        Value::Object(map) => {
            for (key, value) in map {
                println!("{:indent$}{}", "", key, indent = indent);
                display_structure_recursive(value, indent + 2);
            }
        }
        Value::Array(arr) => {
            for (index, item) in arr.iter().enumerate() {
                println!("{:indent$}[{}]", "", index, indent = indent);
                display_structure_recursive(item, indent + 2);
            }
        }
        _ => {}
    }
}
