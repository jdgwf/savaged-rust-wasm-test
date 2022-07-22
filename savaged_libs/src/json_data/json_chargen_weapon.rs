use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct JSONWeaponDefinition {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub summary: String,

    pub book_id: u64,
    #[serde(default)]
    pub page: String,


    #[serde(default)]
    pub created_on: String,
    #[serde(default)]
    pub updated_on: String,
    #[serde(default)]
    pub deleted_on: String,
    #[serde(default)]
    pub deleted: bool,

}