use std::{error::Error, fs};

use toml_edit::Document;

fn main() -> Result<(), Box<dyn Error>> {
    edit_config()?;
    comments()?;
    Ok(())
}

fn edit_config() -> Result<(), Box<dyn Error>> {
    let config = fs::read_to_string("package/Config.toml")?;
    let mut config = config.parse::<Document>()?;

    config["data"]["name"]
        .as_value_mut()
        .unwrap()
        .mutate("Orsen3 -> \"orsenkucher3\"".into());

    // let val_raw = config["data"]["name"].as_value_mut().unwrap();
    // let val_decor = val_raw.decor();
    // *val_raw = toml_edit::decorated(
    //     "Orsen2 -> \"orsenkucher2\"".into(),
    //     val_decor.prefix(),
    //     val_decor.suffix(),
    // );

    // config["data"].as_inline_table_mut().map(|t| t.fmt());

    let result = config.to_string();
    println!("{}", result);

    fs::write("package/Duplicate.toml", result)?;

    Ok(())
}

fn comments() -> Result<(), Box<dyn Error>> {
    let config = fs::read_to_string("package/Duplicate.toml")?;
    let doc = config.parse::<Document>()?;
    println!("Doc: \n{}", doc.to_string());

    let (key, value) = doc["data"]
        .as_table()
        .unwrap()
        .get_kv("name")
        .unwrap()
        .decor()
        .unwrap();

    println!(
        "Comments:\n{}key{}:{}value{}",
        key.prefix(),
        key.suffix(),
        value.prefix(),
        value.suffix()
    );

    Ok(())
}
