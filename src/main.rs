use anyhow::{Context, Result};
use my_main_parser_kma_zaporozhetss::parser::{
    display_structure, get_by_path, minify_json, search_by_value,
};
use my_main_parser_kma_zaporozhetss::{
    convert_to_format, edit_json, handle_large_json, parse_partial_json, validate_json_schema,
};
use serde_json::Value;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
/// Reads and parses a JSON file into a `serde_json::Value` structure.
///
/// # Arguments
///
/// * `file_path` - A string slice holding the path to the JSON file to be read and parsed.
///
/// # Returns
///
/// * `Result<Value>` - Returns the parsed JSON as a `serde_json::Value` if successful,
///    or an error if reading or parsing fails.
///
/// # Example
///
/// ```
/// let json = read_and_parse_json("path/to/file.json").unwrap();
/// ```
fn read_and_parse_json(file_path: &str) -> Result<Value> {
    let json_str = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read JSON file at path: {}", file_path))?;
    serde_json::from_str(&json_str).with_context(|| "Failed to parse JSON".to_string())
}

/// Writes the given content to the output file `output.txt`.
///
/// # Arguments
///
/// * `content` - A string slice containing the content to write to the file.
///
/// # Returns
///
/// * `Result<()>` - Returns `Ok(())` if the content is written successfully, or an error if writing fails.
///
/// # Example
///
/// ```
/// write_to_file("Some content to write").unwrap();
/// ```
fn write_to_file(content: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("output.txt")
        .with_context(|| "Failed to open or create output.txt")?;
    file.write_all(content.as_bytes())
        .with_context(|| "Failed to write to output.txt")?;
    Ok(())
}

/// Main function that performs various operations on a JSON file, including validation,
/// parsing, editing, conversion, and displaying structure.
///
/// Each operation writes its output to `output.txt`.
///
/// # Returns
///
/// * `Result<()>` - Returns `Ok(())` if all operations succeed, or an error if any operation fails.
fn main() -> Result<()> {
    // Initialize the logger for displaying information and error messages.
    env_logger::init();

    // Path to the input JSON file
    let input_path = "src/grammar/test.json";

    // Read and parse the JSON file once
    let json = read_and_parse_json(input_path)?;

    // Sequentially execute all commands

    // Validate command: validates JSON against a provided schema.
    let schema_path = "src/grammar/schema.json";
    let schema = read_and_parse_json(schema_path)?;
    let validate_result = match validate_json_schema(&json, &schema) {
        Ok(_) => "JSON is valid against the schema.".to_string(),
        Err(e) => format!("Validation error: {}", e),
    };
    write_to_file(&validate_result)?;

    // Parse-partial command: parses a specific part of the JSON by a given key.
    let key = "some_key"; // Specify the required key here
    let parse_partial_result = if let Some(part) = parse_partial_json(&json, key) {
        format!("Parsed part: {:#?}", part)
    } else {
        "Key not found in JSON.".to_string()
    };
    write_to_file(&parse_partial_result)?;

    // Edit command: modifies a specific key in the JSON with a new value.
    let key = "some_key_to_edit";
    let new_value: Value = serde_json::json!("new_value");
    let mut json_clone = json.clone();
    edit_json(&mut json_clone, key, new_value)?;
    let edit_result = serde_json::to_string_pretty(&json_clone)?;
    fs::write(input_path, &edit_result)?;
    write_to_file(&edit_result)?;

    // Convert command: converts JSON to another format (YAML).
    let format = "yaml";
    let convert_result = match convert_to_format(&json, format) {
        Ok(output) => output,
        Err(e) => format!("Conversion error: {}", e),
    };
    write_to_file(&convert_result)?;

    // Large-file command: handles large JSON files by parsing them in chunks.
    handle_large_json(Path::new(input_path))?;
    write_to_file("Finished parsing large JSON file.")?;

    // Search command: searches for keys with a specific value in the JSON.
    let search_value = "value_to_search";
    let search_results = search_by_value(&json, search_value);
    let search_result = format!("Search results: {:?}", search_results);
    write_to_file(&search_result)?;

    // Path command: retrieves a JSON value by a given path.
    let json_path = "data.items[0].name";
    let path_result = match get_by_path(&json, json_path) {
        Some(value) => format!("Value at path {}: {:?}", json_path, value),
        None => "Path not found in JSON.".to_string(),
    };
    write_to_file(&path_result)?;

    // Minify command: minifies the JSON by removing unnecessary whitespace.
    let minified = minify_json(&json);
    write_to_file(&minified)?;

    // Structure command: displays the structure of the JSON file.
    let structure_output: Vec<String> = Vec::new();
    display_structure(&json);
    write_to_file(&structure_output.join("\n"))?;

    // Pretty-print command: formats the JSON in a human-readable way.
    let pretty_json = serde_json::to_string_pretty(&json)?;
    write_to_file(&pretty_json)?;

    // Print command: prints the raw parsed JSON.
    let print_result = format!("{:#?}", json);
    write_to_file(&print_result)?;

    Ok(())
}
