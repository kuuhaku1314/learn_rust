use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn build(mut env: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        // skip program name
        env.next();
        let filename = match env.next() {
            Some(filename) => filename,
            None => return Err("Didn't get a filename string"),
        };
        let query = match env.next() {
            Some(query) => query,
            None => return Err("Didn't get a query string"),
        };
        Ok(Config { query, filename })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(&config.filename)?;
    let result = search(&text, &config.query);
    result.iter().for_each(move |line| println!("{}", line));
    Ok(())
}

fn search<'a>(text: &'a str, query: &str) -> Vec<&'a str> {
    text.lines().filter(|line| line.contains(query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "fast";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(contents, query));
    }
}