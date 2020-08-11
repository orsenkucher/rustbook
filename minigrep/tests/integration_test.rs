use minigrep::{self, Config};
use std::error::Error;

#[test]
fn it_runs() -> Result<(), Box<dyn Error>> {
    let args: &Vec<_> = &["", "to", "tests/test_poem.txt", "true"]
        .iter()
        .map(|s| *s)
        .map(String::from)
        .collect();

    let config = Config::new(args)?;

    minigrep::run(config)?;

    Ok(())
}
