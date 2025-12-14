pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query_lower = query.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query_lower))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_correctly() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct three.
        ";
        let expected = vec!["safe, fast, productive."];

        let result = search_case_sensitive(query, contents);

        assert_eq!(expected, result);
    }

    #[test]
    fn search_not_found() {
        let query = "to be or not to be";
        let contents = "\
Rust:
safe, fast, productive.
Duct three.
        ";

        let expected: Vec<String> = Vec::new();
        let result = search_case_sensitive(query, contents);

        assert_eq!(expected, result);
    }

    #[test]
    fn search_case_insensitive_correctly() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct three.
        ";
        let expected = vec!["safe, fast, productive.", "Duct three."];

        let result = search_case_insensitive(query, contents);

        assert_eq!(expected, result);
    }

    #[test]
    fn search_case_insensitive_not_found() {
        let query = "to be or not to be";
        let contents = "\
Rust:
safe, fast, productive.
Duct three.
        ";

        let expected: Vec<String> = Vec::new();
        let result = search_case_insensitive(query, contents);

        assert_eq!(expected, result);
    }
}
