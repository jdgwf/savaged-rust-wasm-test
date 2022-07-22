use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct JSONBookDefinition {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub summary: String,

    #[serde(default)]
    pub publisher: String,

    #[serde(default)]
    pub published: String,


    #[serde(default)]
    pub created_on: String,
    #[serde(default)]
    pub updated_on: String,
    #[serde(default)]
    pub deleted_on: String,
    #[serde(default)]
    pub deleted: bool,
}