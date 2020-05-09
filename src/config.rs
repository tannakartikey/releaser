use std::str;

pub struct Config {
  pub version: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 2 {
      return Err("No release version provided!");
    }

    let version = args[1].clone();

    Ok(Config { version })
  }
}
