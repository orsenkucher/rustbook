use std::{error::Error, fs};

use toml_edit::{value, Document};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let toml = r#"
"hello" = 'toml!' # comment
['a'.b]
"#;
    let mut doc = toml.parse::<Document>().expect("invalid doc");
    assert_eq!(doc.to_string(), toml);
    // let's add a new key/value pair inside a.b: c = {d = "hello"}
    doc["a"]["b"]["c"]["d"] = value("hello");
    // autoformat inline table a.b.c: { d = "hello" }
    doc["a"]["b"]["c"].as_inline_table_mut().map(|t| t.fmt());
    let expected = r#"
"hello" = 'toml!' # comment
['a'.b]
c = { d = "hello" }
"#;
    assert_eq!(doc.to_string(), expected);

    edit_config()?;

    Ok(())
}

fn edit_config() -> Result<(), Box<dyn Error>> {
    let config = fs::read_to_string("package/Config.toml")?;
    let mut config = config.parse::<Document>()?;

    let orsen = &config["data"]["name"];
    let orsen = orsen.as_value();
    if let Some(orsen) = orsen {
        println!("{}", orsen);
        println!("{}", orsen.as_str().unwrap());
    }
    Ok(())
}
