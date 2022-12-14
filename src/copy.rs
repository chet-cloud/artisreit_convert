use std::env;
use std::fs;
use std::{
    io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

fn absolute_path(path: impl AsRef<Path>) -> Result<PathBuf, std::io::Error> {
    let path = path.as_ref();
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    };
    return Ok(absolute_path);
}

pub fn dir_mirror(from: &str, to: &str) -> Result<(), io::Error> {
    for e in WalkDir::new(from).into_iter() {
        if let Ok(entry) = e {
            let path_str = entry.path().to_str().unwrap();
            //println!("=>{}\t {}\t {}",path_str,from,to);
            if let Some(new_path_str) = update_path(path_str, from, to) {
                // println!("copy file: {}", new_path_str);
                if let Some(ex) = Path::new(&new_path_str).extension() {
                    //println!("ex:\t{}", ex.to_str().unwrap());
                    if ex == "md" || ex == "json" || ex == "yaml" {
                        println!("Copy_file:\t{} => {}", &path_str, &new_path_str);
                        if let Err(e) = fs::copy(Path::new(&path_str), Path::new(&new_path_str)) {
                            return Err(e);
                        }
                    }
                } else {
                    println!("Create_dir:\t{}", &new_path_str);
                    if let Err(e) = fs::create_dir_all(Path::new(&new_path_str)) {
                        return Err(e);
                    }
                }
            }
        }
    }
    Ok(())
}

fn update_path<'a>(sub_path: &'a str, from: &'a str, to: &'a str) -> Option<String> {
    if let Ok(from_path) = absolute_path(Path::new(from)) {
        if let Ok(to_path) = absolute_path(Path::new(to)) {
            if let Ok(path) = absolute_path(Path::new(sub_path)) {
                let from_path_str = from_path.as_path().to_str().unwrap();
                let to_path_str = to_path.as_path().to_str().unwrap();
                let path = path.as_path().to_str().unwrap();
                // println!("{}----{}---->{}",path,from_path_str,to_path_str);
                return Some(path.replacen(from_path_str, to_path_str, 1));
            }
        }
    }
    return None;
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::generator::cat;
    use std::path::Component;

    #[test]
    fn test_path_mirror_copy() {
        dir_mirror("test", "test1").unwrap();
        assert!(Path::new("./test1").is_dir());
        assert!(cat("./test/files.json").unwrap() == cat("./test1/files.json").unwrap());
        assert!(cat("./test/test.json").unwrap() == cat("./test1/test.json").unwrap());
        assert!(cat("./test/test.md").unwrap() == cat("./test1/test.md").unwrap());
        assert!(cat("./test/test.yaml").unwrap() == cat("./test1/test.yaml").unwrap());
        fs::remove_dir_all("test1").unwrap();
    }

    fn test_path(t: (&str, &str, &str, &str)) {
        if let Some(a) = update_path(t.0, t.1, t.2) {
            if let Ok(to_path) = absolute_path(Path::new(t.3)) {
                let b = to_path.to_str().unwrap();
                assert_eq!(a, b);
                return;
            }
        }
        assert!(false);
    }

    #[test]
    fn test_update_path() {
        let p = vec![("test/test1/ff", "test", "cc", "cc/test1/ff")];
        for v in p.iter() {
            test_path(*v);
        }
    }

    #[test]
    fn testxxx() {
        use std::ffi::OsStr;
        use std::path::{Component, Path};

        let mut components = Path::new("tmp/foo.txt").components();

        // assert_eq!(components.next(), Some(Component::Normal));
        assert_eq!(
            components.next(),
            Some(Component::Normal(OsStr::new("tmp")))
        );
        assert_eq!(
            components.next(),
            Some(Component::Normal(OsStr::new("foo.txt")))
        );
        assert_eq!(components.next(), None);

        // println!("{}",Component::Normal(OsStr::new("tmp")).as_os_str().to_str());

        assert_eq!("rs", Path::new("foo.rs").extension().unwrap());
        assert_eq!("gz", Path::new("foo.tar.gz").extension().unwrap());
        assert_eq!(None, Path::new("foo").extension());

        for e in WalkDir::new("test").into_iter() {
            if let Ok(entry) = e {
                let path_str = entry.path().to_str().unwrap();
                println!("{}", path_str);
            }
        }

        //fs::copy(Path::new("/home/bear/projects/artisreit_convert/test/test.yaml"), Path::new("/home/bear/projects/artisreit_convert/test1/test.yaml"));
    }
}
