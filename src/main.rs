use std::env;
use std::error::Error;
use std::path::Path;
use std::process;
use artisreit_convert::config::Config;
use artisreit_convert::generator;
use artisreit_convert::copy;
/// 1. copy directory  
/// https://docs.rs/fs_extra/1.2.0/fs_extra/dir/fn.copy.html
/// 2. command line
/// 3. http get json
/// 4. sql api  

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    println!(
        "Generate files by:
         From:\t[{}] 
         To  :\t[{}]
         URL :\t[{}]",
        config.from, config.to, config.url
    );

    let root_path = &config.to;
    let data_json_path = Path::new(&config.from).join(Path::new("data.json"));
    if data_json_path.is_file() {
        if let Ok(data_json) = generator::cat(data_json_path.to_str().unwrap()){
            generator::create_files_by_json(&data_json, ".", root_path);
            copy::dir_mirror(&config.from,&config.to).expect("dir_mirror error")
        }
    }else{
        //todo url get json;
    }

    Ok(())
}