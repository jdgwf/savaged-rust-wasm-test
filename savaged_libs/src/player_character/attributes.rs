use wasm_bindgen::prelude::*;
use crate::utils::get_dice_value::get_dice_value;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Attributes {

    pub selected_agility: u8,
    pub selected_smarts: u8,
    pub selected_spirit: u8,
    pub selected_strength: u8,
    pub selected_vigor: u8,

    pub boosted_agility: u8,
    pub boosted_smarts: u8,
    pub boosted_spirit: u8,
    pub boosted_strength: u8,
    pub boosted_vigor: u8,

    pub bonus_agility: i8,
    pub bonus_smarts: i8,
    pub bonus_spirit: i8,
    pub bonus_strength: i8,
    pub bonus_vigor: i8,

    pub min_agility: u8,
    pub min_smarts: u8,
    pub min_spirit: u8,
    pub min_strength: u8,
    pub min_vigor: u8,

    pub max_agility: u8,
    pub max_smarts: u8,
    pub max_spirit: u8,
    pub max_strength: u8,
    pub max_vigor: u8,
}

#[wasm_bindgen]
impl Attributes {
    #[wasm_bindgen(constructor)]
    pub fn new()->Attributes {
        Attributes{
            selected_agility: 1,
            boosted_agility: 0,
            bonus_agility: 0,

            selected_smarts: 1,
            boosted_smarts: 0,
            bonus_smarts: 0,

            selected_spirit: 1,
            boosted_spirit: 0,
            bonus_spirit: 0,

            selected_strength: 1,
            boosted_strength: 0,
            bonus_strength: 0,

            selected_vigor: 1,
            boosted_vigor: 0,
            bonus_vigor: 0,


            min_agility: 1,
            min_smarts: 1,
            min_spirit: 1,
            min_strength: 1,
            min_vigor: 1,

            max_agility: 5,
            max_smarts: 5,
            max_spirit: 5,
            max_strength: 5,
            max_vigor: 5,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn agility_hr( &self ) -> String {
        get_dice_value( self.selected_agility + self.boosted_agility, self.bonus_agility ).clone()
    }

    #[wasm_bindgen(getter)]
    pub fn smarts_hr( &self ) -> String {
        get_dice_value( self.selected_smarts + self.boosted_smarts, self.bonus_smarts ).clone()
    }

    #[wasm_bindgen(getter)]
    pub fn spirit_hr( &self ) -> String {
        get_dice_value( self.selected_spirit  + self.boosted_spirit, self.bonus_spirit).clone()
    }

    #[wasm_bindgen(getter)]
    pub fn strength_hr( &self ) -> String {
        get_dice_value( self.selected_strength + self.boosted_strength, self.bonus_strength ).clone()
    }

    #[wasm_bindgen(getter)]
    pub fn vigor_hr( &self ) -> String {
        get_dice_value( self.selected_vigor  + self.boosted_vigor, self.bonus_vigor).clone()


    }

    pub fn set_attribute_boosted_agility( &mut self, new_val: u8 ) {
        self.min_agility = new_val + 1;
        self.max_agility = new_val + 5;
        self.boosted_agility = new_val;
    }
    pub fn set_attribute_boosted_smarts( &mut self, new_val: u8 ) {
        self.min_smarts = new_val + 1;
        self.max_smarts = new_val + 5;
        self.boosted_smarts = new_val;
    }
    pub fn set_attribute_boosted_spirit( &mut self, new_val: u8 ) {
        self.min_spirit = new_val + 1;
        self.max_spirit = new_val + 5;
        self.boosted_spirit = new_val;
    }
    pub fn set_attribute_boosted_strength( &mut self, new_val: u8 ) {
        self.min_strength = new_val + 1;
        self.max_strength = new_val + 5;
        self.boosted_strength = new_val;
    }
    pub fn set_attribute_boosted_vigor( &mut self, new_val: u8 ) {
        self.min_vigor = new_val + 1;
        self.max_vigor = new_val + 5;
        self.boosted_vigor = new_val;
    }

    pub fn set_attribute_bonus_agility( &mut self, new_val: i8 ) {
        self.bonus_agility = new_val;
    }
    pub fn set_attribute_bonus_smarts( &mut self, new_val: i8 ) {
        self.bonus_smarts = new_val;
    }
    pub fn set_attribute_bonus_spirit( &mut self, new_val: i8 ) {
        self.bonus_spirit = new_val;
    }
    pub fn set_attribute_bonus_strength( &mut self, new_val: i8 ) {
        self.bonus_strength = new_val;
    }
    pub fn set_attribute_bonus_vigor( &mut self, new_val: i8 ) {
        self.bonus_vigor = new_val;
    }

    pub fn reset( &mut self ) {
        self.boosted_agility = 0;
        self.boosted_smarts = 0;
        self.boosted_spirit = 0;
        self.boosted_strength = 0;
        self.boosted_vigor = 0;

        self.bonus_agility = 0;
        self.bonus_smarts = 0;
        self.bonus_spirit = 0;
        self.bonus_strength = 0;
        self.bonus_vigor = 0;

    }

}