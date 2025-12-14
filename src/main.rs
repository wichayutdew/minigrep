use minigrep::{search_case_insensitive, search_case_sensitive};
use std::env;
use std::error::Error;
use std::process;

mod config;
use crate::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(e) = run(&args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    let config = Config::build(args)?;
    let contents = config.file_contents()?;

    let result = match config.get_case_insensitive() {
        true => search_case_insensitive(config.get_query(), &contents),
        false => search_case_sensitive(config.get_query(), &contents),
    };

    result.iter().for_each(|line| println!("{}", line));

    Ok(())
}
