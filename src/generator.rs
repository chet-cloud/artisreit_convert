use serde_json::{Map, Value};
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

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

pub fn create_files_by_json(json_str: &str, base_path_str: &str, root_path: &str) {
    let path = Path::new(base_path_str);
    if let Ok(Value::Array(vec)) = serde_json::from_str(json_str) {
        let new_vec = [vec![Value::String(root_path.to_string())], vec].concat();
        create_dir_by_array(&new_vec, path);
    }
}

fn create_dir_by_array(vec_value: &Vec<Value>, path: &Path) {
    for v in vec_value.iter() {
        if let Value::String(path_name) = v {
            // println!("--------->{}",path_name);
            let current_path = &path.join(path_name);
            if let Err(_) = fs::create_dir(current_path.as_path()) {
                //println!("{}-{}",path_name,s.to_string())
            }
            println!("Generate_dir\t{}", current_path.to_str().unwrap());
            for v in vec_value.iter() {
                match v {
                    Value::Array(vec) => create_dir_by_array(vec, current_path.as_path()),
                    Value::Object(map) => create_md_by_object(map, current_path.as_path()),
                    _ => {}
                }
            }
            break;
        }
    }
}

fn create_md_by_object(object_value: &Map<String, Value>, path: &Path) {
    if let Some(Value::String(file_name)) = object_value.get("file_name") {
        if let Ok(file_content) = serde_json::to_string(&object_value) {
            let current_path = path.join(file_name);
            if let Some(file_content) = json_to_toml(&file_content) {
                // println!("-->toml_file_content\t{}", &file_content);
                if let Some(file_content) = toml_to_md(&file_content) {
                    println!("Generate_file\t{}", current_path.to_str().unwrap());
                    // println!("md_file_content\t{}<---", &file_content);
                    echo_with_path(&file_content, &current_path).expect("echo error");
                }
            }
        }
    }
}

fn json_to_yaml(jsonstr: &str) -> Option<String> {
    if let Ok(Value::Object(map)) = serde_json::from_str(jsonstr) {
        if let Ok(yaml) = serde_yaml::to_string(&map) {
            return Some(yaml);
        }
    }
    eprint!("There are problem to parse the json string and convert to yaml string");
    return None;
}

fn json_to_toml(jsonstr: &str) -> Option<String> {
    if let Ok(Value::Object(map)) = serde_json::from_str(jsonstr) {
        if let Ok(toml) = toml::to_string(&map) {
            return Some(toml);
        }
    }
    eprint!("There are problem to parse the json string and convert to toml string");
    return None;
}

fn yaml_to_md(yamlstr: &str) -> Option<String> {
    let mut result = String::new();
    if let Ok(Value::Object(map)) = serde_yaml::from_str(yamlstr) {
        let mut yaml_map = map;
        if let Some(Value::String(con)) = yaml_map.get("content") {
            result.push_str(con);
            yaml_map.remove("content");
        }
        result.insert_str(0, "+++\n");
        if let Ok(yaml) = serde_yaml::to_string(&yaml_map) {
            result.insert_str(0, &yaml);
        }
    } else {
        result.insert_str(0, "+++\n");
    }
    result.insert_str(0, "+++\n");
    return Some(result);
}

fn toml_to_md(yamlstr: &str) -> Option<String> {
    let mut result = String::new();
    if let Ok(Value::Object(map)) = toml::from_str(yamlstr) {
        let mut yaml_map = map;
        if let Some(Value::String(con)) = yaml_map.get("content") {
            result.push_str(con);
            yaml_map.remove("content");
        }
        result.insert_str(0, "+++\n");
        if let Ok(yaml) = serde_yaml::to_string(&yaml_map) {
            result.insert_str(0, &yaml);
        }
    } else {
        result.insert_str(0, "+++\n");
    }
    result.insert_str(0, "+++\n");
    return Some(result);
}

// A simple implementation of `% cat path`
pub fn cat(file_path_str: &str) -> io::Result<String> {
    let path = Path::new(file_path_str);
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// A simple implementation of `% echo s > path`
pub fn echo(file_content: &str, file_path_str: &str) -> io::Result<()> {
    let path = Path::new(file_path_str);
    let mut f = File::create(path)?;
    f.write_all(file_content.as_bytes())
}

fn echo_with_path(file_content: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(file_content.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_files_by_json() {
        assert!(!Path::new("./test/a.md").exists());
        assert!(!Path::new("./test/b.md").exists());
        assert!(!Path::new("./test/subdir/a.md").exists());
        assert!(!Path::new("./test/subdir/b.md").exists());
        assert!(!Path::new("./test/subdir/subsubdir/a.md").exists());
        assert!(!Path::new("./test/subdir/subsubdir/b.md").exists());

        let data = fs::read_to_string("test/files.json").unwrap();
        create_files_by_json(&data, &".", "test");

        assert!(Path::new("./test/a.md").exists());
        assert!(Path::new("./test/b.md").exists());
        assert!(Path::new("./test/subdir/a.md").exists());
        assert!(Path::new("./test/subdir/b.md").exists());
        assert!(Path::new("./test/subdir/subsubdir/a.md").exists());
        assert!(Path::new("./test/subdir/subsubdir/b.md").exists());

        fs::remove_file("./test/a.md").unwrap();
        fs::remove_file("./test/b.md").unwrap();
        fs::remove_dir_all("./test/subdir").unwrap();
    }

    #[test]
    fn test_create_file_by_json() {
        let data = r#"
        {
            "file_name": "a.md",
            "value":"cc"
        }"#;
        assert!(!Path::new("./test/a.md").exists());
        if let Value::Object(map) = serde_json::from_str(data).unwrap() {
            let path = Path::new("./test");
            create_md_by_object(&map, path);
        }
        assert!(Path::new("./test/a.md").exists());
        fs::remove_file("./test/a.md").unwrap();
    }

    #[test]
    fn test_create_dir_by_json() {
        let data = r#"
        [
            "test",
            [
                "subdir"
            ]
        ]"#;
        assert!(!Path::new("./test/subdir").exists());
        if let Value::Array(v) = serde_json::from_str(data).unwrap() {
            let path = Path::new(".");
            create_dir_by_array(&v, path);
        }
        assert!(Path::new("./test/subdir").exists());
        fs::remove_dir_all("./test/subdir").unwrap();
    }

    #[test]
    fn test_create_file() {
        if let Ok(_) = echo("hello world", "hello.txt") {
            assert_eq!(cat("hello.txt").unwrap().to_string(), "hello world");
            assert!(fs::remove_file("hello.txt").is_ok());
        }
    }

    #[test]
    fn test_to_md() {
        let content = fs::read_to_string("test/test.yaml").unwrap();
        let result = yaml_to_md(&content).unwrap();
        let content = fs::read_to_string("test/test.md").unwrap();
        assert_eq!(content, result)
    }

    #[test]
    fn test_json_to_yaml() {
        let content = fs::read_to_string("test/test.json").unwrap();
        let result = json_to_yaml(&content).unwrap();
        let content = fs::read_to_string("test/test.yaml").unwrap();
        assert_eq!(content, result)
    }

    #[test]
    fn test_json_to_toml() {
        let json = fs::read_to_string("test/test.toml.json").unwrap();
        let result = json_to_toml(&json).unwrap();
        let json = fs::read_to_string("test/test.toml").unwrap();
        assert_eq!(json, result)
    }
}
