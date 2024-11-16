use std::fs::File;
use pest::Parser;
use json_parser_with_pest::parser::{JSONParser, Rule};
use anyhow::{Context, Result};
use serde_json::{json, Value};
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use std::fs::File;
    use std::io::Write;
    use pest::Parser;
    use tempfile::tempdir;
    /// Tests basic JSON parsing functionality.
    #[test]
    fn test_parse_simple_json() -> Result<()> {
        let json_data = r#"{ "name": "John", "age": 30, "city": "New York" }"#;
        let result = json_parser_with_pest::parse_json(json_data)
            .context("Failed to parse valid JSON")?;
        assert!(result.is_object(), "Expected parsed JSON to be an object.");
        Ok(())
    }

    /// Tests JSON schema validation functionality.
    #[test]
    fn test_validate_json_schema() -> Result<()> {
        let json_data = json!({ "name": "John", "age": 30 });
        let schema = json!({ "name": "", "age": 0 });
        json_parser_with_pest::validate_json_schema(&json_data, &schema)
            .context("Schema validation failed for valid JSON and schema")?;
        Ok(())
    }

    /// Tests partial JSON parsing by a specific key.
    #[test]
    fn test_parse_partial_json() -> Result<()> {
        let json_data = json!({ "name": "John", "age": 30, "city": "New York" });
        let result = json_parser_with_pest::parse_partial_json(&json_data, "city")
            .context("Failed to retrieve value for the key 'city'")?;
        assert_eq!(result, json!("New York"), "Incorrect value retrieved for 'city'.");
        Ok(())
    }

    /// Tests editing a JSON object by modifying a specific key's value.
    #[test]
    fn test_edit_json() -> Result<()> {
        let mut json_data = json!({ "name": "John", "age": 30 });
        json_parser_with_pest::edit_json(&mut json_data, "age", json!(35))
            .context("Failed to edit JSON key 'age'")?;
        assert_eq!(
            json_data["age"], 35,
            "Value of key 'age' did not update correctly."
        );
        Ok(())
    }

    /// Tests conversion of JSON to YAML format.
    #[test]
    fn test_convert_to_yaml() -> Result<()> {
        let json_data = json!({ "name": "John", "age": 30 });
        let result = json_parser_with_pest::convert_to_format(&json_data, "yaml")
            .context("Failed to convert JSON to YAML")?;
        assert!(
            result.contains("name: John"),
            "YAML conversion did not contain expected data."
        );
        Ok(())
    }

    /// Tests conversion of JSON to XML format.
    #[test]
    fn test_convert_to_xml() -> Result<()> {
        let json_data = json!({ "name": "John", "age": 30 });
        let result = json_parser_with_pest::convert_to_format(&json_data, "xml")
            .context("Failed to convert JSON to XML")?;
        assert!(
            result.contains("<name>John</name>"),
            "XML conversion did not contain expected data."
        );
        Ok(())
    }

    /// Tests handling of large JSON files by parsing them in chunks.
    #[test]
    fn test_handle_large_json() -> Result<()> {
        let dir = tempdir().context("Failed to create temporary directory")?;
        let file_path = dir.path().join("large_test.json");
        let mut file = File::create(&file_path).context("Failed to create temporary file")?;
        write!(
            file,
            "[{}]",
            (0..1000)
                .map(|_| "{\"key\": \"value\"}")
                .collect::<Vec<_>>()
                .join(",")
        )
            .context("Failed to write large JSON data to temporary file")?;

        json_parser_with_pest::handle_large_json(&file_path)
            .context("Failed to handle large JSON file")?;
        Ok(())
    }

    /// Tests minifying JSON by removing whitespace.
    #[test]
    fn test_minify_json() -> Result<()> {
        let json_data = json!({ "name": "John", "age": 30, "city": "New York" });
        let minified = json_parser_with_pest::parser::minify_json(&json_data);
        let parsed_minified: Value =
            serde_json::from_str(&minified).context("Failed to parse minified JSON")?;
        assert_eq!(
            parsed_minified, json_data,
            "Minified JSON does not match the original structure."
        );
        Ok(())
    }

    /// Tests retrieving a value by JSON path.
    #[test]
    fn test_get_by_path() -> Result<()> {
        let json_data = json!({
            "data": {
                "items": [
                    { "name": "Item1" },
                    { "name": "Item2" }
                ]
            }
        });
        let result =
            json_parser_with_pest::parser::get_by_path(&json_data, "data.items[1].name")
                .context("Failed to retrieve value by JSON path")?;
        assert_eq!(result, json!("Item2"), "Incorrect value retrieved by JSON path.");
        Ok(())
    }

    /// Tests searching JSON by a specific value, retrieving paths where the value is found.
    #[test]
    fn test_search_by_value() -> Result<()> {
        let json_data = json!({
            "name": "John",
            "details": {
                "age": 30,
                "location": "New York",
                "nickname": "John"
            }
        });
        let mut results = json_parser_with_pest::parser::search_by_value(&json_data, "John");
        results.sort();
        assert_eq!(
            results,
            vec!["details.nickname", "name"],
            "Failed to find correct paths for value 'John'."
        );
        Ok(())
    }

    /// Tests invalid JSON input parsing.
    #[test]
    fn test_invalid_json_parse() -> Result<()> {
        let invalid_json_data = "{ name: John, age: 30 }"; // Invalid JSON
        let result = json_parser_with_pest::parse_json(invalid_json_data);
        assert!(
            result.is_err(),
            "Expected failure for invalid JSON input, but parsing succeeded."
        );
        Ok(())
    }

    /// Tests recursive JSON object parsing.
    #[test]
    fn test_recursive_object_parsing() -> Result<()> {
        let input = r#"
            {
                "key1": {
                    "nested_key": {
                        "deep_nested_key": "value"
                    }
                }
            }
        "#;
        JSONParser::parse(Rule::json, input).context("Failed to parse nested JSON object")?;
        Ok(())
    }

    /// Tests edge cases for empty objects and arrays.
    #[test]
    fn test_empty_structures() -> Result<()> {
        let valid_inputs = vec!["{}", "[]"];
        for input in valid_inputs {
            JSONParser::parse(Rule::json, input)
                .with_context(|| format!("Failed to parse empty structure: {}", input))?;
        }
        Ok(())
    }
}
    /// Tests basic JSON parsing functionality.
    #[test]
    fn test_parse_simple_json() {
        let json_data = r#"{ "name": "John", "age": 30, "city": "New York" }"#;
        let result = json_parser_with_pest::parse_json(json_data);
        assert!(
            result.is_ok(),
            "Failed to parse valid JSON: {:?}",
            result.err()
        );
    }

    /// Tests JSON schema validation functionality.
    #[test]
    fn test_validate_json_schema() {
        let json_data = json!({ "name": "John", "age": 30 });
        let schema = json!({ "name": "", "age": 0 });
        let result = json_parser_with_pest::validate_json_schema(&json_data, &schema);
        assert!(
            result.is_ok(),
            "Schema validation failed for valid JSON and schema."
        );
    }

    /// Tests partial JSON parsing by a specific key.
    #[test]
    fn test_parse_partial_json() {
        let json_data = json!({ "name": "John", "age": 30, "city": "New York" });
        let result = json_parser_with_pest::parse_partial_json(&json_data, "city");
        assert_eq!(
            result,
            Some(json!("New York")),
            "Failed to retrieve the correct value for the key 'city'."
        );
    }

    /// Tests editing a JSON object by modifying a specific key's value.
    #[test]
    fn test_edit_json() {
        let mut json_data = json!({ "name": "John", "age": 30 });
        let result = json_parser_with_pest::edit_json(&mut json_data, "age", json!(35));
        assert!(result.is_ok(), "Failed to edit JSON key 'age'.");
        assert_eq!(
            json_data["age"], 35,
            "Value of key 'age' did not update correctly."
        );
    }

    /// Tests conversion of JSON to YAML format.
    #[test]
    fn test_convert_to_yaml() {
        let json_data = json!({ "name": "John", "age": 30 });
        let result = json_parser_with_pest::convert_to_format(&json_data, "yaml");
        assert!(
            result.is_ok(),
            "Failed to convert JSON to YAML: {:?}",
            result.err()
        );
        assert!(
            result.unwrap().contains("name: John"),
            "YAML conversion did not contain expected data."
        );
    }

    /// Tests conversion of JSON to XML format.
    #[test]
    fn test_convert_to_xml() {
        let json_data = json!({ "name": "John", "age": 30 });
        let result = json_parser_with_pest::convert_to_format(&json_data, "xml");
        assert!(
            result.is_ok(),
            "Failed to convert JSON to XML: {:?}",
            result.err()
        );
        assert!(
            result.unwrap().contains("<name>John</name>"),
            "XML conversion did not contain expected data."
        );
    }

/// Tests handling of large JSON files by parsing them in chunks.
#[test]
fn test_handle_large_json() {
    use std::io::Write; // Імпорт для запису у файл

    // Створення тимчасового каталогу
    let dir = tempdir().expect("Failed to create temp directory");
    let file_path = dir.path().join("large_test.json");

    // Створення тимчасового файлу
    let mut file = File::create(&file_path).expect("Failed to create temp file");

    // Запис великих даних у файл
    let json_content = format!(
        "[{}]",
        (0..1000)
            .map(|_| "{\"key\": \"value\"}")
            .collect::<Vec<_>>()
            .join(",")
    );
    file.write_all(json_content.as_bytes())
        .expect("Failed to write to file");

    // Тестування функції handle_large_json
    let result = json_parser_with_pest::handle_large_json(&file_path);
    assert!(
        result.is_ok(),
        "Failed to handle large JSON file: {:?}",
        result.err()
    );
}


    /// Tests minifying JSON by removing whitespace.
    #[test]
    fn test_minify_json() {
        let json_data = json!({ "name": "John", "age": 30, "city": "New York" });
        let minified = json_parser_with_pest::parser::minify_json(&json_data);

        let parsed_minified: Value = serde_json::from_str(&minified).unwrap();
        assert_eq!(
            parsed_minified, json_data,
            "Minified JSON does not match the original structure."
        );
    }


    /// Tests retrieving a value by JSON path.
    #[test]
    fn test_get_by_path() {
        let json_data = json!({
            "data": {
                "items": [
                    { "name": "Item1" },
                    { "name": "Item2" }
                ]
            }
        });
        let result =
            json_parser_with_pest::parser::get_by_path(&json_data, "data.items[1].name");
        assert_eq!(
            result,
            Some(json!("Item2")),
            "Failed to retrieve value by JSON path."
        );
    }

    /// Tests searching JSON by a specific value, retrieving paths where the value is found.
    #[test]
    fn test_search_by_value() {
        let json_data = json!({
            "name": "John",
            "details": {
                "age": 30,
                "location": "New York",
                "nickname": "John"
            }
        });
        let mut results =
            json_parser_with_pest::parser::search_by_value(&json_data, "John");
        results.sort();
        assert_eq!(
            results,
            vec!["details.nickname", "name"],
            "Failed to find correct paths for value 'John'."
        );
    }

    /// Tests handling of invalid JSON input.
    /// Tests handling of invalid JSON input.
    #[test]
    fn test_invalid_json_parse() {
        let invalid_json_data = "{ name: John, age: 30 }"; // Invalid JSON due to missing quotes
        let result = json_parser_with_pest::parse_json(invalid_json_data);

        // Перевіряємо, чи повертається помилка
        assert!(
            result.is_err(),
            "Expected failure for invalid JSON input, but parsing succeeded."
        );

        // Перевіряємо, чи повернена помилка є саме ParsingError
        if let Err(err) = result {
            assert!(
                matches!(err, json_parser_with_pest::ParserError::JsonParseError),
                "Expected JsonParseError, but got: {:?}",
                err
            );
        }
    }


    /// Tests validation of JSON schema with extra keys in the schema.
    #[test]
    fn test_invalid_json_schema() {
        let json_data = json!({ "name": "John", "age": 30 });
        let schema = json!({ "name": "", "age": 0, "extra_key": "" });
/*
        // Якщо дозволяємо надлишкові ключі в схемі
        let result = json_parser_with_pest::validate_json_schema(&json_data, &schema);
        assert!(
            result.is_ok(),
            "Expected schema validation to succeed, but it failed: {:?}",
            result.err()
        );
*/
        // Якщо надлишкові ключі мають викликати помилку

        let result = json_parser_with_pest::validate_json_schema(&json_data, &schema);
        assert!(
            result.is_err(),
            "Expected schema validation to fail due to extra key in schema."
        );

    }

    /// Tests parsing invalid key-value arrays.
    #[test]
    fn test_key_value_array_invalid() {
        let inputs = vec![
            r#"
            [
                { "key1": "value1" },
                { "key2" "value2" }  // Missing colon
            ]
            "#,
            r#"
            [
                { "key1": "value1" },
                "not_an_object"
            ]
            "#,
            r#"
            [
                { "key1": "value1", "key2": "value2" }
            "# // Missing closing bracket
        ];

        for input in inputs {
            let result = JSONParser::parse(Rule::key_value_array, input);
            assert!(
                result.is_err(),
                "Expected failure for invalid key-value array: {}. Result: {:?}",
                input,
                result
            );
        }
    }
/// Test parsing of JSON date in format YYYY-MM-DD.
#[test]
fn test_date_parsing() {
    let valid_dates = vec!["2023-11-15", "2000-01-01", "1999-12-31"];
    for date in valid_dates {
        let result = JSONParser::parse(Rule::date, date);
        assert!(
            result.is_ok(),
            "Failed to parse valid date: {}. Error: {:?}",
            date,
            result.err()
        );
    }

    let invalid_dates = vec!["2023-13-01", "202-11-15", "2023-11", "abcd-11-15"];
    for date in invalid_dates {
        let result = JSONParser::parse(Rule::date, date);
        assert!(
            result.is_err(),
            "Parsed invalid date: {}. Result: {:?}",
            date,
            result
        );
    }
}

/// Test parsing of JSON identifier.
#[test]
fn test_identifier_parsing() {
    let valid_identifiers = vec!["key", "key1", "_key", "key_name", "Key123"];
    for identifier in valid_identifiers {
        let result = JSONParser::parse(Rule::identifier, identifier);
        assert!(
            result.is_ok(),
            "Failed to parse valid identifier: {}. Error: {:?}",
            identifier,
            result.err()
        );
    }

    let invalid_identifiers = vec!["1key", "-key", "key@", "key name"];
    for identifier in invalid_identifiers {
        let result = JSONParser::parse(Rule::identifier, identifier);
        assert!(
            result.is_err(),
            "Parsed invalid identifier: {}. Result: {:?}",
            identifier,
            result
        );
    }
}

/// Test parsing of SemVer version strings.
#[test]
fn test_version_parsing() {
    let valid_versions = vec![
        "1.0.0",
        "2.1.3-alpha",
        "3.2.1-beta.1",
        "4.5.6+build.123",
        "0.1.0",
        "10.20.30-pre.4+metadata.12345",
    ];
    for version in valid_versions {
        let result = JSONParser::parse(Rule::version, version);
        assert!(
            result.is_ok(),
            "Failed to parse valid version: {}. Error: {:?}",
            version,
            result.err()
        );
    }

    let invalid_versions = vec![
        "1.0",
        "v1.0.0",
        "1.0.0-",
        "1.0.0+",
        "1.0.0-beta..1",
        "1.0.0-alpha+build@123",
    ];
    for version in invalid_versions {
        let result = JSONParser::parse(Rule::version, version);
        assert!(
            result.is_err(),
            "Parsed invalid version: {}. Result: {:?}",
            version,
            result
        );
    }
}

/// Test parsing of key-value arrays.
#[test]
fn test_key_value_array_parsing() {
    let valid_arrays = vec![
        r#"[{ "key1": "value1" }, { "key2": "value2" }]"#,
        r#"[{ "key": "value" }]"#,
        r#"[]"#,
    ];
    for array in valid_arrays {
        let result = JSONParser::parse(Rule::key_value_array, array);
        assert!(
            result.is_ok(),
            "Failed to parse valid key-value array: {}. Error: {:?}",
            array,
            result.err()
        );
    }

    let invalid_arrays = vec![
        r#"[{ "key1": "value1" }, { "key2" "value2" }]"#, // Missing colon
        r#"[{ "key": "value" }, "string"]"#,              // Non-object element
        r#"[{ "key": "value""#,                           // Missing closing bracket
    ];
    for array in invalid_arrays {
        let result = JSONParser::parse(Rule::key_value_array, array);
        assert!(
            result.is_err(),
            "Parsed invalid key-value array: {}. Result: {:?}",
            array,
            result
        );
    }
}

/// Test recursive JSON object parsing.
#[test]
fn test_recursive_object_parsing() {
    let input = r#"
        {
            "key1": {
                "nested_key": {
                    "deep_nested_key": "value"
                }
            }
        }
        "#;
    let result = JSONParser::parse(Rule::json, input);
    assert!(
        result.is_ok(),
        "Failed to parse nested JSON object. Error: {:?}",
        result.err()
    );
}

/// Test parsing of JSON arrays with mixed types.
#[test]
fn test_mixed_type_array_parsing() {
    let input = r#"[123, "string", true, null, { "key": "value" }, [1, 2, 3]]"#;
    let result = JSONParser::parse(Rule::json, input);
    assert!(
        result.is_ok(),
        "Failed to parse mixed-type array. Error: {:?}",
        result.err()
    );
}

/// Test edge cases for empty objects and arrays.
#[test]
fn test_empty_structures() {
    let valid_inputs = vec!["{}", "[]"];
    for input in valid_inputs {
        let result = JSONParser::parse(Rule::json, input);
        assert!(
            result.is_ok(),
            "Failed to parse empty structure: {}. Error: {:?}",
            input,
            result.err()
        );
    }
}