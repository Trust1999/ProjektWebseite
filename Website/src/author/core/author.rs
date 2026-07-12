use crate::author::core::author_command;
use crate::author::core::author_id;
use crate::author::core::author_record;

pub struct Author {
    pub author_id: author_id::AuthorID,
    pub name: String,
}

impl From<author_record::AuthorRecord> for Author {
    fn from(value: author_record::AuthorRecord) -> Self {
        Author {
            author_id: value.author_id.into(),
            name: value.name,
        }
    }
}

impl From<author_command::AuthorCommandPost> for Author {
    fn from(value: author_command::AuthorCommandPost) -> Self {
        Author {
            author_id: value.author_id,
            name: value.name,
        }
    }
}
