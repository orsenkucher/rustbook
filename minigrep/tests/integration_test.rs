use minigrep::{self, Config};
use std::error::Error;

#[test]
fn it_runs() -> Result<(), Box<dyn Error>> {
    let args = ["", "to", "tests/test_poem.txt", "true"]
        .iter()
        .map(|s| *s)
        .map(String::from);

    let config = Config::new(args)?;

    minigrep::run(config)?;

    Ok(())
}
