use wasm_bindgen::prelude::*;
use uuid::{Uuid};
use crate::player_character::PlayerCharacter;
use chrono::prelude::*;

#[wasm_bindgen]
pub struct Hindrance {
    #[wasm_bindgen(skip)]
    pub name: String,
    #[wasm_bindgen(skip)]
    pub uuid: Uuid,

    created_on:  DateTime<Utc>,
    updated_on:  DateTime<Utc>,
    deleted_on:  DateTime<Utc>,
    pub deleted: bool,
}


#[wasm_bindgen]
impl Hindrance {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Hindrance {
        //use the . operator to fetch the value of a field via the self keyword
        Hindrance{
            name: "".to_string(),
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
impl Hindrance {

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