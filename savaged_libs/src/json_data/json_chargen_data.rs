use serde::{Serialize, Deserialize};
use super::json_chargen_edge::JSONEdgeDefinition;
use super::json_chargen_hindrance::JSONHindranceDefinition;

#[derive(Deserialize, Serialize, Debug)]
pub struct JSONChargenData {
    pub edges: Vec<JSONEdgeDefinition>,
    pub hindrances: Vec<JSONHindranceDefinition>,
}

