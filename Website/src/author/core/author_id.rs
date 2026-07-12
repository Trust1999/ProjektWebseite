use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorIDData {
    #[serde(rename = "author_id")]
    author_id_data: uuid::Uuid,
}

impl From<uuid::Uuid> for AuthorIDData {
    fn from(value: uuid::Uuid) -> Self {
        AuthorIDData {
            author_id_data: value,
        }
    }
}

impl From<AuthorIDData> for uuid::Uuid {
    fn from(value: AuthorIDData) -> Self {
        value.author_id_data
    }
}

impl From<AuthorID> for AuthorIDData {
    fn from(value: AuthorID) -> Self {
        AuthorIDData {
            author_id_data: value.author_id,
        }
    }
}

pub struct AuthorID {
    author_id: uuid::Uuid,
}

impl AuthorID {
    pub fn new() -> AuthorID {
        AuthorID {
            author_id: uuid::Uuid::new_v4(),
        }
    }
}

impl From<AuthorIDData> for AuthorID {
    fn from(value: AuthorIDData) -> Self {
        AuthorID {
            author_id: value.author_id_data,
        }
    }
}

impl From<AuthorIDRecord> for AuthorID {
    fn from(value: AuthorIDRecord) -> Self {
        AuthorID {
            author_id: value.author_id_record,
        }
    }
}

pub struct AuthorIDRecord {
    author_id_record: uuid::Uuid,
}

impl From<AuthorID> for AuthorIDRecord {
    fn from(value: AuthorID) -> Self {
        AuthorIDRecord {
            author_id_record: value.author_id,
        }
    }
}

impl From<uuid::Uuid> for AuthorIDRecord {
    fn from(value: uuid::Uuid) -> Self {
        AuthorIDRecord {
            author_id_record: value,
        }
    }
}

impl From<AuthorIDRecord> for uuid::Uuid {
    fn from(value: AuthorIDRecord) -> Self {
        value.author_id_record
    }
}
