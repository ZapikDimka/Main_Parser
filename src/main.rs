use clap::{Arg, Command};
use anyhow::{Context, Result};
use json_parser_with_pest::parser::{
    display_structure, get_by_path, minify_json, search_by_value,
};
use json_parser_with_pest::{
    convert_to_format, edit_json, handle_large_json, parse_partial_json, validate_json_schema,
};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Reads and parses a JSON file into a `serde_json::Value` structure.
fn read_and_parse_json(file_path: &str) -> Result<Value> {
    let json_str = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read JSON file at path: {}", file_path))?;
    serde_json::from_str(&json_str).with_context(|| "Failed to parse JSON".to_string())
}

/// Writes the given content to the output file `output.txt`.
fn write_to_file(content: &str) -> Result<()> {
    fs::write("output.txt", content).with_context(|| "Failed to write to output.txt")
}

/// CLI-supported main function.
fn main() -> Result<()> {
    // Initialize the logger for displaying information and error messages.
    env_logger::init();

    // Define the CLI commands and arguments
    let matches = Command::new("JSON Parser")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A tool for parsing and manipulating JSON files")
        .subcommand(
            Command::new("help")
                .about("Displays help information for available commands"),
        )
        .subcommand(
            Command::new("validate")
                .about("Validates a JSON file against a schema")
                .arg(Arg::new("input").required(true).help("Input JSON file path"))
                .arg(Arg::new("schema").required(true).help("Schema JSON file path")),
        )
        .subcommand(
            Command::new("minify")
                .about("Minifies a JSON file by removing whitespace")
                .arg(Arg::new("input").required(true).help("Input JSON file path")),
        )
        .subcommand(
            Command::new("structure")
                .about("Displays the structure of a JSON file")
                .arg(Arg::new("input").required(true).help("Input JSON file path")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("help", _)) => {
            println!(
                "Available commands:\n\
                 validate: Validates a JSON file against a schema\n\
                 minify: Minifies a JSON file by removing whitespace\n\
                 structure: Displays the structure of a JSON file"
            );
        }
        Some(("validate", args)) => {
            let input_path = args.get_one::<String>("input").unwrap();
            let schema_path = args.get_one::<String>("schema").unwrap();
            let json = read_and_parse_json(input_path)?;
            let schema = read_and_parse_json(schema_path)?;
            let validate_result = match validate_json_schema(&json, &schema) {
                Ok(_) => "JSON is valid against the schema.".to_string(),
                Err(e) => format!("Validation error: {}", e),
            };
            write_to_file(&validate_result)?;
        }
        Some(("minify", args)) => {
            let input_path = args.get_one::<String>("input").unwrap();
            let json = read_and_parse_json(input_path)?;
            let minified = minify_json(&json);
            write_to_file(&minified)?;
        }
        Some(("structure", args)) => {
            let input_path = args.get_one::<String>("input").unwrap();
            let json = read_and_parse_json(input_path)?;
            display_structure(&json);
        }
        _ => {
            println!("Invalid command. Use `help` for the list of available commands.");
        }
    }

    Ok(())
}
