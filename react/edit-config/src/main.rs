use std::{error::Error, fs};

use toml_edit::{value, Document};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let toml = r#"
"hello" = 'toml!' # comment
['a'.b] # my
"#;
    let mut doc = toml.parse::<Document>().expect("invalid doc");
    assert_eq!(doc.to_string(), toml);
    // let's add a new key/value pair inside a.b: c = {d = "hello"}
    doc["a"]["b"]["c"]["d"] = value("hello");
    let mut val = doc["hello"].as_value_mut().unwrap();
    let decor = doc["hello"].as_value().unwrap().decor();
    println!("{:?}", decor);
    // toml_edit::decorated(value, prefix, suffix)

    // toml_edit::
    // decor.()
    // toml_edit::Decor::

    // doc["hello"] = value("toml2");
    let val_raw = doc["hello"].as_value_mut().unwrap();
    // value

    let val_decor = val_raw.decor();
    *val_raw = toml_edit::decorated(
        toml_edit::Value::from("toml2!"),
        val_decor.prefix(),
        val_decor.suffix(),
    );
    doc["hello"].as_inline_table_mut().map(|t| t.fmt());
    // val_raw.as_inline_table_mut().map(|e| e.fmt());
    // val.as_str().unwrap() = "Heh";
    // autoformat inline table a.b.c: { d = "hello" }
    doc["a"]["b"]["c"].as_inline_table_mut().map(|t| t.fmt());
    let expected = r#"
"hello" = 'toml2!' # comment
['a'.b] # my
c = { d = "hello" }
"#;
    assert_eq!(doc.to_string(), expected);
    println!("{}", doc.to_string());

    edit_config()?;

    Ok(())
}

fn edit_config() -> Result<(), Box<dyn Error>> {
    let config = fs::read_to_string("package/Config.toml")?;
    let mut config = config.parse::<Document>()?;

    let orsen = &config["data"]["name"];
    let orsen = orsen.as_value();
    // config.
    if let Some(orsen) = orsen {
        println!("{}", orsen);
        println!("{}", orsen.as_str().unwrap());
    }

    // config["data"]["name"].as_array()
    config["data"]["name"] = value("Orsen2");
    config["data"].as_inline_table_mut().map(|t| t.fmt());

    let result = config.to_string();
    println!("{}", result);

    Ok(())
}
