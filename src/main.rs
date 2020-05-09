use releaser::config;
use std::env;
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = config::Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing args: {}", err);
    process::exit(1);
  });

  if let Err(e) = releaser::run(config) {
    println!("Application error: {}", e);
    process::exit(1);
  }
}
