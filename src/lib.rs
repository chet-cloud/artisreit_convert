use serde_json::{Result, Value};
use std::collections::BTreeMap;
use std::fs;


fn convert_array( map: &mut BTreeMap, v:&Vec<Value>){

}

fn convert_object( map: &mut BTreeMap, v:&Map<String, Value>){
    
}

fn json_2_yaml(jsonstr: &str) -> Result<()> {
    let mut map = BTreeMap::new();
    let v: Value = serde_json::from_str(jsonstr).unwrap();

    // enum Value {
    //     Null,
    //     Bool(bool),
    //     Number(Number),
    //     String(String),
    //     Array(Vec<Value>),
    //     Object(Map<String, Value>),
    // }

    match v {
        Array(vec_Value>) =>{

        },
        Object(map_String_Value) =>{
            map.insert(key, value)
        },
        Null => 
        Bool(bool),
        Number(Number),
        String(String),
    }




    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_yaml() {
        // You have some type.
        let mut map = BTreeMap::new();
        map.insert("x".to_string(), 1.0);
        map.insert("y".to_string(), 2.0);

        // Serialize it to a YAML string.
        let yaml = serde_yaml::to_string(&map).unwrap();
        assert_eq!(yaml, "x: 1.0\ny: 2.0\n");

        // Deserialize it back to a Rust type.
        let deserialized_map: BTreeMap<String, f64> = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(map, deserialized_map);
    }

    #[test]
    fn test_load_json_file() {
        // Some JSON input data as a &str. Maybe this comes from the user.
        let content = fs::read_to_string("test.json").unwrap();
        // let data = r#"
        //     {
        //         "name": "John Doe",
        //         "age": 43,
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
            format!("Please call {} at the number {}", v["name"], v["phones"][0]),
            "Please call \"John Doe\" at the number \"+44 1234567\""
        );
    }
}
