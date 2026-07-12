use crate::author::core::author;
use crate::author::core::author_id;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthorData {
    pub author_id: Option<author_id::AuthorIDData>,
    pub name: Option<String>,
}

impl From<author::Author> for AuthorData {
    fn from(value: author::Author) -> Self {
        AuthorData {
            author_id: Some(value.author_id.into()),
            name: Some(value.name),
        }
    }
}
