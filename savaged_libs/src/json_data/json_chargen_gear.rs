use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct JSONGearDefinition {
    pub id: u64,
    pub name: String,
    pub summary: String,
}