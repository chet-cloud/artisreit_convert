use serde_json::{Value, Map, Number};
use std::{fs, error::Error, result};


fn json_2_yaml(jsonstr: &str) -> Option<String>{
    if let Ok(Value::Object(map)) = serde_json::from_str(jsonstr) {
         if let Ok(yaml) = serde_yaml::to_string(&map){
            return Some(yaml)
         }
    }
    eprint!("There are problem to parse the json string and convert to yaml string");
    return None
}

fn yaml_2_md(yamlstr: &str) -> Option<String>{
    let mut result = String::new();
    if let Ok(Value::Object(map)) = serde_yaml::from_str(yamlstr) {
         let mut yaml_map = map;
         if let Some(Value::String(con)) = yaml_map.get("content") {
            result.push_str(con);
            yaml_map.remove("content");
         }
         result.insert_str(0,"+++\n");
         if let Ok(yaml) = serde_yaml::to_string(&yaml_map){
            result.insert_str(0, &yaml);
         }
    }else{
        result.insert_str(0,"+++\n");
    }
    result.insert_str(0,"+++\n");
    return Some(result);
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_toMD() {
        let content = fs::read_to_string("test/test.yaml").unwrap();
        let result = yaml_2_md(&content).unwrap();
        let content = fs::read_to_string("test/test.md").unwrap();
        assert_eq!(content,result)
    }


    #[test]
    fn test_json_2_yaml() {
        let content = fs::read_to_string("test/test.json").unwrap();
        let result = json_2_yaml(&content).unwrap();
        let content = fs::read_to_string("test/test.yaml").unwrap();
        assert_eq!(content,result)
    }



    // #[test]
    fn test_yaml() {
        // You have some type.
        let mut map = Map::new();
        map.insert("x".to_string(), Value::Number(Number::from(1)));
        map.insert("y".to_string(), Value::Number(Number::from(2)));

        // Serialize it to a YAML string.
        let yaml = serde_yaml::to_string(&map).unwrap();
        assert_eq!(yaml, "x: 1\ny: 2\n");

        // Deserialize it back to a Rust type.
        let deserialized_map: Map<String, Value> = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(map, deserialized_map);
    }

    // #[test]
    fn test_load_json_file() {
        // Some JSON input data as a &str. Maybe this comes from the user.
        let content = fs::read_to_string("test.json").unwrap();
        // let data = r#"
        //     {
        //         "name": "John Doe",
        //         "age": 43,
        //         "empty": null,
        //         "phones": [
        //             "+44 1234567",
        //             "+44 2345678"
        //         ]
        //     }"#;
        let data = &content;

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(data).unwrap();

        // Access parts of the data by indexing with square brackets.
        println!("Please call {} at the number {}", v["name"], v["phones"][0]);

        assert_eq!(
            format!("Please call {} at the number {}, is {}", v["name"], v["phones"][0],v["empty"]),
            "Please call \"John Doe\" at the number \"+44 1234567\", is null"
        );
    }
}
