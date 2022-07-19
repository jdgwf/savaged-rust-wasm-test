use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct JSONCharacterExport {
    pub id: u64,
    pub name: String,

    pub created_on: String,
    pub updated_on: String,
    pub deleted_on: String,
    pub deleted: bool,
}