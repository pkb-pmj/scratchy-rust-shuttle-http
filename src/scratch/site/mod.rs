pub fn user_link(username: &str) -> String {
    format!("[{username}](https://scratch.mit.edu/users/{username})")
}

pub fn username_is_valid(username: &str) -> bool {
    username.len() <= 20
        && username
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_username() {
        assert!(username_is_valid("PMJ_Studio"), "ordinary username");
        assert!(
            username_is_valid("12345678901234567890"),
            "only digits, max length"
        );
        assert!(
            username_is_valid("qwertyuiopasdfghjklz"),
            "only letters, max length"
        );
        assert!(username_is_valid("-_"), "dash and underscore");
        assert!(!username_is_valid("ą"), "non-ASCII letter");
        assert!(!username_is_valid(";"), "invalid ASCII character");
        assert!(!username_is_valid("123456789012345678901"), "too long");
    }
}
