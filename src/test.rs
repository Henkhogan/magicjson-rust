#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_file() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("tests").join("test0.json");

        // Write some JSON content to the file
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, r#"{{"key": "value1"}}"#).unwrap();

        // Call load_file function
        let result = load_file(file_path.to_str().unwrap().to_string());

        // Check the result
        match result {
            Ok(dict) => {
                assert_eq!(dict.len(), 1);
                assert_eq!(dict.get("key").unwrap(), "value1");
            },
            Err(_) => panic!("load_file failed"),
        }

        // Close the temporary directory
        dir.close().unwrap();
    }

    #[test]
    fn test_serialize_any() {
        // Create a HashMap
        let mut map = HashMap::new();
        map.insert("key".to_string(), "value1".to_string());

        // Call serialize_any function
        let result = serialize_any(&map);

        // Check the result
        assert_eq!(result, r#"{"key":"value1"}"#);
        println!("{}", result);
    }
}