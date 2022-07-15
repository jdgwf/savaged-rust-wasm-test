use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct JSONHindranceDefinition {
    pub id: u64,
    pub name: String,
}