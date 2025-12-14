use minigrep::{search_case_insensitive, search_case_sensitive};
use std::env;
use std::error::Error;
use std::process;

mod config;
use crate::config::Config;

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)?;

    let query = config.get_query();
    let contents = config.file_contents()?;
    let case_insensitive = config.get_case_insensitive();

    let result = match case_insensitive {
        true => search_case_insensitive(query, &contents),
        false => search_case_sensitive(query, &contents),
    };

    result.iter().for_each(|line| println!("{}", line));

    Ok(())
}
