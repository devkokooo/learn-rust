//! # Minigrep
//! This is just a comment.
//! 
//! I'm learning Rust from the Rust book.
//! 
//! They forced me to build this exercise.

/// Search contents by query string
/// 
/// # Examples
/// 
/// ```
/// let mut iter = search("boat", "my boat is sinking\nmy brain is thinking");
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(
            &query.to_lowercase()
        ))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents).collect::<Vec<_>>()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
