
pub struct Config {
    pub from: String,
    pub to: String,
    pub url: String,
}


impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let from = match args.next() {
            Some(arg) => arg,
            None => "pre_content".to_string(),
        };

        let url = match args.next() {
            Some(arg) => arg,
            None => "".to_string(),
        };

        let to = match args.next() {
            Some(arg) => arg,
            None => "content".to_string(),
        };

        Ok(Config {
            from,
            to,
            url,
        })
    }
}