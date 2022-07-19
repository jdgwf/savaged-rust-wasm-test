use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct JSONGearDefinition {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub summary: String,

    pub book_id: u64,
    #[serde(default)]
    pub page: String,
}