use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct JSONEdgeDefinition {
    pub id: u64,
    pub name: String,
}