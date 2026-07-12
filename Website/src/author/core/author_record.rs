use crate::author::core::author;
use crate::author::core::author_id;

pub struct AuthorRecord {
    pub author_id: author_id::AuthorIDRecord,
    pub name: String,
}

impl From<author::Author> for AuthorRecord {
    fn from(value: author::Author) -> Self {
        AuthorRecord {
            author_id: value.author_id.into(),
            name: value.name,
        }
    }
}
