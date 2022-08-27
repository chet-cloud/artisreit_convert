use serde_json::{Map,Value};
use std::fs;
use std::fs::{File};
use std::io;
use std::path::Path;
use std::io::prelude::*;
// https://github.com/dtolnay/serde-yaml
// https://github.com/serde-rs/json
// enum Value {
//     Null,
//     Bool(bool),
//     Number(Number),
//     String(String),
//     Array(Vec<Value>),
//     Object(Map<String, Value>),
// }

fn create_dir_by_array(vec_value:&Vec<Value>,path: &Path){
    for v in vec_value.iter() {
        if let Value::String(path_name) = v {
            let current_path = &path.join(path_name);
            if let Ok(_) = fs::create_dir(current_path) {
                for v in vec_value.iter(){
                    match v {
                        Value::Array(vec) => {
                            create_dir_by_array(vec, current_path.as_path())
                        },
                        Value::Object(map) => {
                            create_md_by_object(map, current_path.as_path())
                        },
                        _ =>{},
                    }
                }
            }
        }
    }
}

fn create_md_by_object(object_value:&Map<String, Value>, path: &Path){
    if let Some(Value::String(file_name)) =  object_value.get("file_name") {
        if let Ok(file_content) = serde_json::to_string(&object_value){
            let current_path = path.join(file_name);
            echo_with_path(&file_content,&current_path);
        }
    }
}


pub fn create_files_by_json(jsonstr: &str,file_path_str:&str){
    let path = Path::new(file_path_str);
    if let Ok(Value::Array(vec)) = serde_json::from_str(jsonstr) {
        create_dir_by_array(&vec, path);
    }
}


fn json_to_yaml(jsonstr: &str) -> Option<String>{
    if let Ok(Value::Object(map)) = serde_json::from_str(jsonstr) {
         if let Ok(yaml) = serde_yaml::to_string(&map){
            return Some(yaml)
         }
    }
    eprint!("There are problem to parse the json string and convert to yaml string");
    return None
}

fn yaml_to_md(yamlstr: &str) -> Option<String>{
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

// A simple implementation of `% cat path`
fn cat(file_path_str: &str) -> io::Result<String> {
    let path = Path::new(file_path_str);
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// A simple implementation of `% echo s > path`
fn echo(file_content:&str,file_path_str:&str) -> io::Result<()> {
    let path = Path::new(file_path_str);
    let mut f = File::create(path)?;
    f.write_all(file_content.as_bytes())
}

fn echo_with_path(file_content:&str,path:&Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(file_content.as_bytes())
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_files_by_json(){
        let content = fs::read_to_string("test/create_files.json").unwrap();
        create_files_by_json(&content,".");
        //assert!(fs::try_exists(Path::new("")).is_ok())
    }

    #[test]
    fn test_create_file(){
        if let Ok(_) = echo("hello world","hello.txt") {
            assert_eq!(cat("hello.txt").unwrap().to_string(),"hello world");
            assert!(fs::remove_file("hello.txt").is_ok());
        }
    }

    #[test]
    fn test_to_md() {
        let content = fs::read_to_string("test/test.yaml").unwrap();
        let result = yaml_to_md(&content).unwrap();
        let content = fs::read_to_string("test/test.md").unwrap();
        assert_eq!(content,result)
    }

    #[test]
    fn test_json_to_yaml() {
        let content = fs::read_to_string("test/test.json").unwrap();
        let result = json_to_yaml(&content).unwrap();
        let content = fs::read_to_string("test/test.yaml").unwrap();
        assert_eq!(content,result)
    }

}
