use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct JSONHindranceDefinition {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub summary: String,

    pub book_id: u64,
    #[serde(default)]
    pub page: String,

    pub created_on: String,
    pub updated_on: String,
    pub deleted_on: String,
    pub deleted: bool,
}