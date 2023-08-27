pub fn project_link(id: i64) -> String {
    format!("[{id}](https://scratch.mit.edu/projects/{id})")
}

pub fn user_link(username: &str) -> String {
    format!("[{username}](https://scratch.mit.edu/users/{username})")
}

pub fn extract_project_id(value: &str) -> Option<i64> {
    let paths = [
        "https://scratch.mit.edu/projects/",
        "https://api.scratch.mit.edu/projects/",
        "https://scratchdb.lefty.one/v3/project/info/",
    ];
    extract(value, &paths).parse().ok()
}

pub fn extract_username(value: &str) -> Option<String> {
    static PATHS: [&str; 4] = [
        "https://scratch.mit.edu/users/",
        "https://api.scratch.mit.edu/users/",
        "https://scratchdb.lefty.one/v3/user/info/",
        "https://scratchstats.com/",
    ];
    let value = extract(value, &PATHS);
    if username_is_valid(value) {
        Some(value.to_string())
    } else {
        None
    }
}

fn extract<'a>(value: &'a str, paths: &[&str]) -> &'a str {
    const CHARS: [char; 3] = ['/', '?', '#'];
    paths
        .iter()
        .find_map(|path| value.strip_prefix(path))
        .map(|v| &v[..v.find(CHARS).unwrap_or(v.len())])
        .unwrap_or(value)
}

fn username_is_valid(username: &str) -> bool {
    username.len() <= 20
        && username
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

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

    #[test]
    fn passthrough_project_id() {
        assert_eq!(extract_project_id("499038694"), Some(499038694));
    }

    #[test]
    fn extract_project_id_valid() {
        let id = 499038694;
        let paths = [
            "https://scratch.mit.edu/projects/",
            "https://api.scratch.mit.edu/projects/",
            "https://scratchdb.lefty.one/v3/project/info/",
        ];
        let subpaths = ["", "/", "/foo/bar"];
        let queries = ["", "?foo=bar&bar=foo"];
        let hashes = ["", "#hash"];

        let mut failures = Vec::new();

        for path in paths {
            for subpath in subpaths {
                for query in queries {
                    for hash in hashes {
                        let case = format!("{path}{id}{subpath}{query}{hash}");
                        let result = extract_project_id(&case);
                        if result != Some(id.into()) {
                            failures.push((result, case));
                        }
                    }
                }
            }
        }

        if failures.len() > 0 {
            let mut message = format!("{} failures:\n", failures.len());
            for (result, case) in failures {
                writeln!(message, "{result:?} | {case}").unwrap();
            }
            None::<()>.expect(&message);
        }
    }

    #[test]
    fn passthrough_username() {
        assert_eq!(extract_username("PMJ_Studio"), Some("PMJ_Studio".into()));
    }

    #[test]
    fn extract_username_valid() {
        let username = "PMJ_Studio";
        let paths = [
            "https://scratch.mit.edu/users/",
            "https://api.scratch.mit.edu/users/",
            "https://scratchdb.lefty.one/v3/user/info/",
            "https://scratchstats.com/",
        ];
        let subpaths = ["", "/", "/foo/bar"];
        let queries = ["", "?foo=bar&bar=foo"];
        let hashes = ["", "#hash"];

        let mut failures = Vec::new();

        for path in paths {
            for subpath in subpaths {
                for query in queries {
                    for hash in hashes {
                        let case = format!("{path}{username}{subpath}{query}{hash}");
                        let result = extract_username(&case);
                        if result != Some(username.into()) {
                            failures.push((result, case));
                        }
                    }
                }
            }
        }

        if failures.len() > 0 {
            let mut message = format!("{} failures:\n", failures.len());
            for (result, case) in failures {
                writeln!(message, "{result:?} | {case}").unwrap();
            }
            None::<()>.expect(&message);
        }
    }
}
