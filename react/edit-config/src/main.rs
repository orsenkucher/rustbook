use std::{error::Error, fs};

use toml_edit::Document;

fn main() -> Result<(), Box<dyn Error>> {
    edit_config()?;
    Ok(())
}

fn edit_config() -> Result<(), Box<dyn Error>> {
    let config = fs::read_to_string("package/Config.toml")?;
    let mut config = config.parse::<Document>()?;

    let val_raw = config["data"]["name"].as_value_mut().unwrap();
    let val_decor = val_raw.decor();
    *val_raw = toml_edit::decorated(
        "Orsen2 -> \"orsenkucher2\"".into(),
        val_decor.prefix(),
        val_decor.suffix(),
    );

    // config["data"].as_inline_table_mut().map(|t| t.fmt());

    let result = config.to_string();
    println!("{}", result);

    fs::write("package/Duplicate.toml", result)?;

    Ok(())
}
