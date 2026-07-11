use crate::author::core::author_data;
pub struct AuthorCommandPost {
    pub _id: i32,
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

impl From<author_data::AuthorData> for AuthorCommandPost{

    fn from(value: author_data::AuthorData) -> Self {
        AuthorCommandPost{
            _id: 5,
            name: match value.name {
                Some(n) => n,
                None => "test".to_string(),
            }
        }
    }
}