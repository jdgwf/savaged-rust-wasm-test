use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct JSONBaseAttributes {
    pub agility: u8,
    pub smarts: u8,
    pub spirit: u8,
    pub strength: u8,
    pub vigor: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct JSONRaceOptions {
    pub chosen_race_abilities: Vec<JSONChargenRaceAbility>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct JSONChargenRaceAbility {
    pub adjusted_value: i64,
    pub custom_effects: Vec<String>,
    pub custom_name: String,
    pub custom_summary: String,
    pub custom_value: i64,
    pub effects: Vec<String>,
    pub max: String,
    pub needs_selected_attribute: bool,
    pub needs_selected_edge: bool,
    pub needs_selected_hindrance: bool,
    pub needs_selected_skill: bool,
    pub needs_selected_trait: bool,
    pub needs_selected_super_powers: bool,
    pub needs_selected_power: bool,
    pub positive: bool,
    pub selected_attribute: String,
    pub selected_edge: String,
    pub selected_hindrance: String,
    pub selected_hindrance_major: bool,
    pub selected_skill: String,
    pub selected_skill_specify: String,
    pub selected_trait: String,
    pub selected_trait_specify: String,
    // pub selected_super_power_2021_options: SuperPower2021ObjectVars,
    pub selected_super_power_2021: i64,
    pub selected_super_power: i64,
    // pub selected_super_power_options: SuperPower2014ObjectVars,
    pub selected_power: i64,
    pub value: i64,

}

#[derive(Deserialize, Serialize, Debug)]
pub struct JSONCharacterExport {
    pub id: u64,
    pub name: String,

    pub created_on: String,
    pub updated_on: String,
    pub deleted_on: String,
    pub deleted: bool,

    pub last_save_id: i64,
    pub race_choices: JSONRaceOptions,

    pub attribute_assignments: JSONBaseAttributes,
    pub version: u64,
    pub session_id: u64,
}