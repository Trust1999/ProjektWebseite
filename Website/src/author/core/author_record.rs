use crate::author::core::author;

pub struct AuthorRecord {
    pub _id: i32,
    pub name: String,
}

impl From<author::Author> for AuthorRecord{
    fn from(value: author::Author) -> Self {
        AuthorRecord{
            _id: value._id,
            name: value.name,
        }
    }
}