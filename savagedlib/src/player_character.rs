use wasm_bindgen::prelude::*;
pub mod attributes;
pub mod edge;
pub mod hindrance;

use uuid::{Uuid};
use attributes::Attributes;
use edge::Edge;
use hindrance::Hindrance;
use chrono::prelude::*;

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
    // date_created:  DateTime<Utc>,
    // date_modified:  DateTime<Utc>,
    // date_deleted:  DateTime<Utc>,
    pub deleted: bool,
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

    pub fn set_attribute_boosted_agility( &mut self, new_val: u8 ) {
        self.attributes.min_agility = new_val + 1;
        self.attributes.max_agility = new_val + 5;
        self.attributes.boosted_agility = new_val;
    }
    pub fn set_attribute_boosted_smarts( &mut self, new_val: u8 ) {
        self.attributes.min_smarts = new_val + 1;
        self.attributes.max_smarts = new_val + 5;
        self.attributes.boosted_smarts = new_val;
    }
    pub fn set_attribute_boosted_spirit( &mut self, new_val: u8 ) {
        self.attributes.min_spirit = new_val + 1;
        self.attributes.max_spirit = new_val + 5;
        self.attributes.boosted_spirit = new_val;
    }
    pub fn set_attribute_boosted_strength( &mut self, new_val: u8 ) {
        self.attributes.min_strength = new_val + 1;
        self.attributes.max_strength = new_val + 5;
        self.attributes.boosted_strength = new_val;
    }
    pub fn set_attribute_boosted_vigor( &mut self, new_val: u8 ) {
        self.attributes.min_vigor = new_val + 1;
        self.attributes.max_vigor = new_val + 5;
        self.attributes.boosted_vigor = new_val;
    }

    pub fn set_attribute_bonus_agility( &mut self, new_val: i8 ) {
        self.attributes.bonus_agility = new_val;
    }
    pub fn set_attribute_bonus_smarts( &mut self, new_val: i8 ) {
        self.attributes.bonus_smarts = new_val;
    }
    pub fn set_attribute_bonus_spirit( &mut self, new_val: i8 ) {
        self.attributes.bonus_spirit = new_val;
    }
    pub fn set_attribute_bonus_strength( &mut self, new_val: i8 ) {
        self.attributes.bonus_strength = new_val;
    }
    pub fn set_attribute_bonus_vigor( &mut self, new_val: i8 ) {
        self.attributes.bonus_vigor = new_val;
    }
}

// WASM Bindgen Getters/Setters
#[wasm_bindgen]
impl PlayerCharacter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PlayerCharacter {
        //use the . operator to fetch the value of a field via the self keyword
        PlayerCharacter{
            name: "".to_string(),
            uuid: Uuid::new_v4(),
            attributes: Attributes::new(),
            selected_edges: Vec::new(),
            selected_hindrances: Vec::new(),
            // date_created: Utc::now(),
            // date_modified: Utc::now(),
            // date_deleted: Utc::now(),
            deleted: false,
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_name( &mut self, new_name: String) {
         self.name = new_name;
    }

    #[wasm_bindgen(getter)]
    pub fn name( &self ) -> String {
        self.name.clone()
    }


    // #[wasm_bindgen(getter)]
    // pub fn date_created( &self ) -> DateTime<Utc> {
    //     self.date_created.clone()
    // }

    // #[wasm_bindgen(getter)]
    // pub fn date_modified( &self ) -> DateTime<Utc> {
    //     self.date_modified.clone()
    // }

    // #[wasm_bindgen(getter)]
    // pub fn date_deleted( &self ) -> DateTime<Utc> {
    //     self.date_deleted.clone()
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


}