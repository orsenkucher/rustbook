use serde::{Deserialize, Serialize};

use std::error::Error;
use std::{fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let config = read_config()?;
    write_config("package/Duplicate.toml", &config)?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
}

fn read_config() -> io::Result<Config> {
    let contents = fs::read_to_string("package/config.toml")?;
    println!("Contents: {}", contents);

    let bytes = fs::read("package/config.toml")?;
    let config: Config = toml::from_slice(&bytes[..])?;
    println!("Config: {:?}", config);

    Ok(config)
}

fn write_config(path: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    let toml = toml::to_string(&config)?;
    fs::write(path, toml)?;
    Ok(())
}
