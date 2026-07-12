use crate::author::core::author_data;
use crate::author::core::author_id;
pub struct AuthorCommandPost {
    pub author_id: author_id::AuthorID,
    pub name: String,
}

/*impl TryFrom<author_data::AuthorData> for AuthorCommandPost{
    type Error = String;

    fn try_from(value: author_data::AuthorData) -> Result<Self, Self::Error> {
        Ok(AuthorCommandPost{
            _id: 5,
            name: match value.name {
                Some(n) => n,
                None => "test".to_string(),
            }
        })
    }
}*/

impl From<author_data::AuthorData> for AuthorCommandPost {
    fn from(author_data: author_data::AuthorData) -> Self {
        AuthorCommandPost {
            author_id: author_id::AuthorID::new(),
            name: match author_data.name {
                Some(n) => n,
                None => "test".to_string(),
            },
        }
    }
}

pub struct AuthorCommandPatch {
    pub author_id: author_id::AuthorID,
    pub name: String,
}

impl AuthorCommandPatch {
    pub fn create(
        author_data: author_data::AuthorData,
        author_data_id: author_id::AuthorIDData,
    ) -> Self {
        AuthorCommandPatch {
            author_id: author_data_id.into(),
            name: match author_data.name {
                Some(n) => n,
                None => "test".to_string(),
            },
        }
    }
}
