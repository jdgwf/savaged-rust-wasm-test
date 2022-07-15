use wasm_bindgen::prelude::*;
use uuid::{Uuid};
use crate::player_character::PlayerCharacter;
use std::collections::HashMap;
use crate::json_data::json_chargen_data::JSONChargenData;

pub struct EdgeVars {

}

pub struct EdgeDef {
    pub id: u64,
    pub is_custom: bool,
    name: String,
    uuid: String,
}

#[wasm_bindgen]
pub struct Edge {
    pub id: u64,
    pub is_custom: bool,
    #[wasm_bindgen(skip)]
    pub name: String,
    #[wasm_bindgen(skip)]
    pub uuid: Uuid,

}

#[wasm_bindgen]
impl Edge {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Edge {
        //use the . operator to fetch the value of a field via the self keyword
        Edge{
            id: 0,
            is_custom: false,
            name: "".to_string(),
            uuid: Uuid::new_v4(),
        }
    }

    pub fn apply( mut char_obj: &PlayerCharacter ) {

    }
}

// WASM Bindgen Getters/Setters
#[wasm_bindgen]
impl Edge {


    #[wasm_bindgen(setter)]
    pub fn set_name( &mut self, new_name: String) {
         self.name = new_name;
    }

    #[wasm_bindgen(getter)]
    pub fn name( &self ) -> String {
        self.name.clone()
    }


    #[wasm_bindgen(setter)]
    pub fn set_uuid( &mut self, new_value: String) {
        // self.uuid = uuid!( new_value[..] );
        self.uuid = Uuid::parse_str( &new_value ).unwrap();
    }

    #[wasm_bindgen(getter)]
    pub fn uuid( &self ) -> String {
        self.uuid.to_string()
    }



}

impl Edge {
    pub fn import_def(
        &mut self,
        id: i32,
        def: EdgeDef,
        available_data: JSONChargenData,
    ) {
        if id == 0 {
            self.set_name( def.name );
            self.is_custom = def.is_custom;
            self.uuid = Uuid::parse_str( &def.uuid ).unwrap();
            self.id = 0
        } else {
            for edge in available_data.edges.iter() {
                self.name = edge.name.clone();
                self.id = edge.id;
            }

        }


    }

    pub fn import_vars(
        &mut self
    ) {

    }
}