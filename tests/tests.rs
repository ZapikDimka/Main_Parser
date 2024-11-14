#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    /// Tests basic JSON parsing functionality.
    #[test]
    fn test_parse_simple_json() {
        let json_data = r#"{ "name": "John", "age": 30, "city": "New York" }"#;
        let result = my_main_parser_kma_zaporozhetss::parse_json(json_data);

        match &result {
            Ok(parsed_value) => println!("Parsed JSON successfully: {:?}", parsed_value),
            Err(e) => println!("Failed to parse JSON: {:?}", e),
        }

        assert!(
            result.is_ok(),
            "Failed to parse JSON with error: {:?}",
            result
        );
    }

    /// Tests JSON schema validation functionality.
    #[test]
    fn test_validate_json_schema() {
        let json_data = json!({ "name": "John", "age": 30 });
        let schema = json!({ "name": "", "age": 0 });
        let result = my_main_parser_kma_zaporozhetss::validate_json_schema(&json_data, &schema);
        assert!(result.is_ok());
    }

    /// Tests partial JSON parsing by a specific key.
    #[test]
    fn test_parse_partial_json() {
        let json_data = json!({ "name": "John", "age": 30, "city": "New York" });
        let result = my_main_parser_kma_zaporozhetss::parse_partial_json(&json_data, "city");
        assert_eq!(result, Some(json!("New York")));
    }

    /// Tests editing a JSON object by modifying a specific key's value.
    #[test]
    fn test_edit_json() {
        let mut json_data = json!({ "name": "John", "age": 30 });
        let result = my_main_parser_kma_zaporozhetss::edit_json(&mut json_data, "age", json!(35));
        assert!(result.is_ok());
        assert_eq!(json_data["age"], 35);
    }

    /// Tests conversion of JSON to YAML format.
    #[test]
    fn test_convert_to_yaml() {
        let json_data = json!({ "name": "John", "age": 30 });
        let result = my_main_parser_kma_zaporozhetss::convert_to_format(&json_data, "yaml");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("name: John"));
    }

    /// Tests conversion of JSON to XML format.
    #[test]
    fn test_convert_to_xml() {
        let json_data = json!({ "name": "John", "age": 30 });
        let result = my_main_parser_kma_zaporozhetss::convert_to_format(&json_data, "xml");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("<name>John</name>"));
    }

    /// Tests handling of large JSON files by parsing them in chunks.
    #[test]
    fn test_handle_large_json() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("large_test.json");
        let mut file = File::create(&file_path).unwrap();
        write!(
            file,
            "[{}]",
            (0..1000)
                .map(|_| "{\"key\": \"value\"}")
                .collect::<Vec<_>>()
                .join(",")
        )
        .unwrap();

        let result = my_main_parser_kma_zaporozhetss::handle_large_json(&file_path);
        assert!(result.is_ok());
    }

    /// Tests minifying JSON by removing whitespace.
    #[test]
    fn test_minify_json() {
        let json_data = json!({ "name": "John", "age": 30, "city": "New York" });
        let expected: Value =
            serde_json::from_str(r#"{"name":"John","age":30,"city":"New York"}"#).unwrap();
        assert_eq!(
            my_main_parser_kma_zaporozhetss::parser::minify_json(&json_data),
            expected.to_string()
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
            my_main_parser_kma_zaporozhetss::parser::get_by_path(&json_data, "data.items[1].name");
        assert_eq!(result, Some(json!("Item2")));
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
            my_main_parser_kma_zaporozhetss::parser::search_by_value(&json_data, "John");
        results.sort();
        assert_eq!(results, vec!["details.nickname", "name"]);
    }

    /// Tests handling of invalid JSON input.
    #[test]
    fn test_invalid_json_parse() {
        let invalid_json_data = "{ name: John, age: 30 }"; // Invalid JSON due to missing quotes
        let result = my_main_parser_kma_zaporozhetss::parse_json(invalid_json_data);
        assert!(result.is_err());
    }

    /// Tests validation of JSON schema with extra keys in the schema.
    #[test]
    fn test_invalid_json_schema() {
        let json_data = json!({ "name": "John", "age": 30 });
        let schema = json!({ "name": "", "age": 0, "extra_key": "" });
        let result = my_main_parser_kma_zaporozhetss::validate_json_schema(&json_data, &schema);
        assert!(result.is_err());
    }

    /// Tests editing a JSON with an invalid key, ensuring an error is returned.
    #[test]
    fn test_edit_json_invalid_key() {
        let mut json_data = json!({ "name": "John", "age": 30 });
        let result = my_main_parser_kma_zaporozhetss::edit_json(
            &mut json_data,
            "nonexistent_key",
            json!(100),
        );
        assert!(result.is_err());
    }
}
