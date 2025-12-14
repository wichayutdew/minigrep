use std::env;
use std::error::Error;
use std::fs;

const IGNORE_CASE_ENV_VAR: &str = "IGNORE_CASE";

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    case_insensitive: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let query = args
            .get(1)
            .ok_or("Please input String you want to find e.g. text")?
            .to_string();

        let filename = args
            .get(2)
            .ok_or("Please input a location to file you wanted to grep e.g. file.txt")?
            .to_string();

        let case_insensitive = env::var(IGNORE_CASE_ENV_VAR).is_ok();

        Ok(Config {
            query,
            filename,
            case_insensitive,
        })
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }

    pub fn get_case_insensitive(&self) -> &bool {
        &self.case_insensitive
    }

    pub fn file_contents(&self) -> Result<String, Box<dyn Error>> {
        Ok(fs::read_to_string(&self.filename)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn config_builds_correctly() {
        let expected_query = "test_query";
        let expected_filename = "test_file.txt";
        let args = vec![
            String::from("minigrep"),
            String::from(expected_query),
            String::from(expected_filename),
        ];

        let config = Config::build(&args).unwrap();

        assert_eq!(expected_query, config.get_query());
        assert_eq!(expected_filename, config.filename);
        assert_eq!(false, *config.get_case_insensitive());
    }

    #[test]
    #[serial]
    fn config_builds_correctly_with_case_insensitive_flag() {
        unsafe {
            env::set_var(IGNORE_CASE_ENV_VAR, "1");
        }

        let expected_query = "test_query";
        let expected_filename = "test_file.txt";
        let args = vec![
            String::from("minigrep"),
            String::from(expected_query),
            String::from(expected_filename),
        ];

        let config = Config::build(&args).unwrap();

        assert_eq!(expected_query, config.get_query());
        assert_eq!(expected_filename, config.filename);
        assert_eq!(true, *config.get_case_insensitive());

        unsafe {
            env::remove_var(IGNORE_CASE_ENV_VAR);
        }
    }
    #[test]
    fn config_builds_fails_with_missing_query() {
        let args = vec![String::from("minigrep")];
        let config_res = Config::build(&args);
        assert!(config_res.is_err());
        assert_eq!(
            "Please input String you want to find e.g. text",
            config_res.unwrap_err(),
        );
    }

    #[test]
    fn config_builds_fails_with_missing_filename() {
        let args = vec![String::from("minigrep"), String::from("test_query")];
        let config_res = Config::build(&args);
        assert!(config_res.is_err());
        assert_eq!(
            "Please input a location to file you wanted to grep e.g. file.txt",
            config_res.unwrap_err(),
        );
    }
}
