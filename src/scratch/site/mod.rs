use super::Url;

pub struct User;

impl Url for User {
    type UrlArgs = String;

    fn url(username: Self::UrlArgs) -> String {
        format!("https://scratch.mit.edu/users/{username}")
    }
}

pub struct Project;

impl Url for Project {
    type UrlArgs = i64;

    fn url(id: Self::UrlArgs) -> String {
        format!("https://scratch.mit.edu/projects/{id}")
    }
}
