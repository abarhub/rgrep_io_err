use std::fs::File;
use std::io::{self, BufRead, BufReader};

use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Config {
    fichier: String,
    text: String,
    debug: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;

    let file = File::open(config.fichier)?;
    let reader = BufReader::new(file);

    let mut trouve = false;

    for line in reader.lines() {
        let line = line?;
        if (config.debug) {
            println!("{}", line);
        }
        if line.contains(&config.text) {
            trouve = true;
            break;
        }
    }

    println!("trouve: {}", trouve);

    Ok(())
}
