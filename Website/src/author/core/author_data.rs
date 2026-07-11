use serde::{Serialize, Deserialize};

use crate::author::core::author;

#[derive(Serialize, Deserialize)]
pub struct AuthorData {
    pub _id: Option<i32>,
    pub name: Option<String>,
}

impl From<author::Author> for AuthorData{
    fn from(value: author::Author) -> Self {
        AuthorData{
            _id: Some(value._id),
            name: Some(value.name),
        }
    }
}