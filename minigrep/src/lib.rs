use std::error::Error;
use std::fs;

// here we use trait object, so it needs to be Boxed
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line)
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut res = vec![];
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn not_enough_arguments() {
        // assert_eq!(Config::new(&vec![]), Err("not enough arguments"))
        assert_eq!(Config::new(&vec![]).err(), Some("not enough arguments"))
    }

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        let data: Vec<_> = ["", "query", "filename"]
            .iter()
            .map(|s| String::from(*s))
            .collect();

        let cfg = Config::new(&data)?;

        assert_eq!(cfg.query, "query");
        assert_eq!(cfg.filename, "filename");

        Ok(())
    }
}
