use wasm_bindgen::prelude::*;
pub mod attributes;
pub mod edge;
pub mod hindrance;
pub mod exports;
pub mod imports;

use crate::json_data;

use uuid::{Uuid};
use attributes::Attributes;
use edge::Edge;
use hindrance::Hindrance;
use chrono::prelude::*;
use json_data::json_chargen_data::JSONChargenData;
use json_data::json_chargen_book::JSONBookDefinition;
use json_data::json_chargen_edge::JSONEdgeDefinition;
use json_data::json_chargen_hindrance::JSONHindranceDefinition;
use json_data::json_chargen_weapon::JSONWeaponDefinition;
use json_data::json_chargen_armor::JSONArmorDefinition;
use json_data::json_chargen_gear::JSONGearDefinition;
use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[wasm_bindgen]
pub struct PlayerCharacter {
    #[wasm_bindgen(skip)]
    pub name: String,

    #[wasm_bindgen(skip)]
    pub uuid: Uuid,

    #[wasm_bindgen(skip)]
    pub attributes: Attributes,

    #[wasm_bindgen(skip)]
    pub selected_edges: Vec< Edge >,

    #[wasm_bindgen(skip)]
    pub selected_hindrances: Vec< Hindrance >,

    #[wasm_bindgen(skip)]
    pub added_edges: Vec< Edge >,

    #[wasm_bindgen(skip)]
    pub added_hindrances: Vec< Hindrance >,
    created_on:  DateTime<Utc>,
    updated_on:  DateTime<Utc>,
    deleted_on:  DateTime<Utc>,
    pub deleted: bool,

    available_data: JSONChargenData,
}

#[wasm_bindgen]
impl PlayerCharacter {

    pub fn set_attribute_selected_agility( &mut self, new_val: u8 ) {
        self.attributes.selected_agility = new_val;
    }
    pub fn set_attribute_selected_smarts( &mut self, new_val: u8 ) {
        self.attributes.selected_smarts = new_val;
    }
    pub fn set_attribute_selected_spirit( &mut self, new_val: u8 ) {
        self.attributes.selected_spirit = new_val;
    }
    pub fn set_attribute_selected_strength( &mut self, new_val: u8 ) {
        self.attributes.selected_strength = new_val;
    }
    pub fn set_attribute_selected_vigor( &mut self, new_val: u8 ) {
        self.attributes.selected_vigor = new_val;
    }

    pub fn calc( &mut self ) {
        self._calc_reset();
    }

    fn _calc_reset( &mut self ) {
        self.attributes.reset();

        self.added_edges = Vec::new();
        self.added_hindrances = Vec::new();
    }
}

// WASM Bindgen Getters/Setters
#[wasm_bindgen]
impl PlayerCharacter {
    #[wasm_bindgen(constructor)]
    pub fn new(
        available_data: String,
    ) -> PlayerCharacter {
        //use the . operator to fetch the value of a field via the self keyword
        let mut pc = PlayerCharacter{
            name: "".to_string(),
            uuid: Uuid::new_v4(),
            attributes: Attributes::new(),
            selected_edges: Vec::new(),
            selected_hindrances: Vec::new(),
            added_edges: Vec::new(),
            added_hindrances: Vec::new(),
            created_on: Utc::now(),
            updated_on: Utc::now(),
            deleted_on: Utc::now(),
            deleted: false,
            available_data: serde_json::from_str(&available_data).unwrap(),
        };

        pc.calc();
        pc
    }

    #[wasm_bindgen(setter)]
    pub fn set_name( &mut self, new_name: String) {
         self.name = new_name;
    }

    #[wasm_bindgen(getter)]
    pub fn name( &self ) -> String {
        self.name.clone()
    }

    pub fn get_available_edges_count( &self ) -> usize {
        self.available_data.edges.len()
    }

    pub fn get_available_books_count( &self ) -> usize {
        self.available_data.books.len()
    }

    pub fn get_available_books_json( &self ) -> String {
        serde_json::to_string( &self.available_data.books ).unwrap()
    }

    pub fn get_available_hindrances_json( &self ) -> String {
        serde_json::to_string( &self.available_data.hindrances ).unwrap()
    }
    pub fn get_available_edges_json( &self ) -> String {
        serde_json::to_string( &self.available_data.edges ).unwrap()
    }

    pub fn get_available_weapons_json( &self ) -> String {
        serde_json::to_string( &self.available_data.weapons ).unwrap()
    }
    pub fn get_available_armor_json( &self ) -> String {
        serde_json::to_string( &self.available_data.armor ).unwrap()
    }
    pub fn get_available_gear_json( &self ) -> String {
        serde_json::to_string( &self.available_data.gear ).unwrap()
    }


    pub fn get_available_hindrances_count( &self ) -> usize {
        self.available_data.hindrances.len()
    }


    pub fn get_available_weapon_count( &self ) -> usize {
        self.available_data.weapons.len()
    }

    pub fn get_available_armor_count( &self ) -> usize {
        self.available_data.armor.len()
    }

    pub fn get_available_gear_count( &self ) -> usize {
        self.available_data.gear.len()
    }
    // #[wasm_bindgen(getter)]
    // pub fn created_on( &self ) -> DateTime<Utc> {
    //     self.created_on.clone()
    // }

    // #[wasm_bindgen(getter)]
    // pub fn updated_on( &self ) -> DateTime<Utc> {
    //     self.updated_on.clone()
    // }

    // #[wasm_bindgen(getter)]
    // pub fn deleted_on( &self ) -> DateTime<Utc> {
    //     self.deleted_on.clone()
    // }

    #[wasm_bindgen(setter)]
    pub fn set_uuid( &mut self, new_value: String) {
        // self.uuid = uuid!( new_value[..] );
        self.uuid = Uuid::parse_str( &new_value ).unwrap();
    }

    #[wasm_bindgen(getter)]
    pub fn uuid( &self ) -> String {
        self.uuid.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn attributes( &self ) -> Attributes {
        self.attributes.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_attributes( &mut self, new_value: Attributes) {
        // self.uuid = uuid!( new_value[..] );
        self.attributes = new_value.clone();
    }

    pub fn reset( &mut self ) {
        self.name = "".to_string();
        self.uuid = Uuid::new_v4();
        self.attributes = Attributes::new();
        self.selected_edges = Vec::new();
        self.selected_hindrances = Vec::new();
        self.added_edges = Vec::new();
        self.added_hindrances = Vec::new();
    }

}

// non WASM functions
impl PlayerCharacter {

    pub fn get_available_books( &self ) -> &Vec< JSONBookDefinition > {
        &self.available_data.books
    }

    pub fn get_available_hindrances( &self ) -> &Vec< JSONHindranceDefinition > {
        &self.available_data.hindrances
    }
    pub fn get_available_edges( &self ) -> &Vec< JSONEdgeDefinition > {
        &self.available_data.edges
    }

    pub fn get_available_weapons( &self ) -> &Vec< JSONWeaponDefinition > {
        &self.available_data.weapons
    }
    pub fn get_available_armor( &self ) -> &Vec< JSONArmorDefinition > {
        &self.available_data.armor
    }
    pub fn get_available_gear( &self ) -> &Vec< JSONGearDefinition > {
        &self.available_data.gear
    }
}