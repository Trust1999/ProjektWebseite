use crate::author::core::author_record;
use crate::author::core::author_command;

pub struct Author {
    pub _id: i32,
    pub name: String,
}

impl From<author_record::AuthorRecord> for Author {
    fn from(value: author_record::AuthorRecord) -> Self {
        Author {
            _id: value._id, 
            name: value.name, 
        }
    }
}

impl From<author_command::AuthorCommandPost> for  Author{
    fn from(value: author_command::AuthorCommandPost) -> Self {
        Author {
            _id: value._id, 
            name: value.name, 
        }
    }
    
}