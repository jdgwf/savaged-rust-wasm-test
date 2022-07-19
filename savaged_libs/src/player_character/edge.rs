use wasm_bindgen::prelude::*;
use uuid::{Uuid};
use crate::player_character::PlayerCharacter;
// use std::collections::HashMap;
use crate::json_data::json_chargen_data::JSONChargenData;
use crate::json_data::json_chargen_edge::JSONEdgeDefinition;
use crate::json_data::json_chargen_edge::JSONEdgeVars;
use chrono::prelude::*;


#[wasm_bindgen]
pub struct Edge {
    pub id: u64,
    pub is_custom: bool,
    #[wasm_bindgen(skip)]
    pub name: String,
    #[wasm_bindgen(skip)]
    pub custom_name: String,
    #[wasm_bindgen(skip)]
    pub uuid: Uuid,

    created_on:  DateTime<Utc>,
    updated_on:  DateTime<Utc>,
    deleted_on:  DateTime<Utc>,
    pub deleted: bool,

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
            custom_name: "".to_string(),
            uuid: Uuid::new_v4(),
            created_on: Utc::now(),
            updated_on: Utc::now(),
            deleted_on: Utc::now(),
            deleted: false,
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
         self.name = new_name.clone();
    }

    #[wasm_bindgen(getter)]
    pub fn name( &self ) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn custom_name( &self ) -> String {
        self.custom_name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_custom_name( &mut self, new_name: String) {
         self.custom_name = new_name.clone();
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
    pub fn import(
        &mut self,
        id: u64,
        def: JSONEdgeDefinition,
        available_data: JSONChargenData,
    ) {
        if id == 0 {
            self._import_definition( id, &def );
        } else {
            for edge in available_data.edges.iter() {
                self._import_definition( edge.id, &edge );
            }

        }
    }

    fn _import_definition(
        &mut self,
        id: u64,
        def: &JSONEdgeDefinition,
    ) {
        self.set_name( def.name.clone() );
        if id == 0 {
            self.is_custom = true;
        } else {
            self.is_custom = false;
        }
        self.id = id;

        self.created_on = DateTime::from_utc(DateTime::parse_from_rfc3339( &def.created_on ).unwrap().naive_utc(), Utc);
        self.updated_on = DateTime::from_utc(DateTime::parse_from_rfc3339( &def.updated_on ).unwrap().naive_utc(), Utc);
        self.deleted_on = DateTime::from_utc(DateTime::parse_from_rfc3339( &def.deleted_on ).unwrap().naive_utc(), Utc);

    }

    pub fn import_vars(
        &mut self,
        vars: &JSONEdgeVars,
    ) {
        self.uuid = Uuid::parse_str( &vars.uuid ).unwrap();
        self.set_custom_name( vars.custom_name.clone() );
    }

}
