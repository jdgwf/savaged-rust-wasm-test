use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_dice_value (
   val: u8,
   bonus: i8
) -> String {
   let mut bonus_string: String = "".to_string();
   if  bonus > 0 {
      bonus_string = "+".to_string() + &bonus.to_string();
   } else if bonus < 0 {
      bonus_string = bonus.to_string();
   }
   if val == 1 {
      return "d4".to_string() + &bonus_string;
   }
   if val == 2 {
      return "d6".to_string() + &bonus_string;
   }
   if val == 3 {
      return "d8".to_string() + &bonus_string;
   }
   if val == 4 {
      return "d10".to_string() + &bonus_string;
   }
   if val == 5 {
      return "d12".to_string() + &bonus_string;
   }
   if val == 6 {
      return "d12+1".to_string() + &bonus_string;
   }
   if val == 7 {
      return "d12+2".to_string() + &bonus_string;
   }
   if val == 8 {
      return "d12+3".to_string() + &bonus_string;
   }
   if val == 9 {
      return "d12+4".to_string() + &bonus_string;
   }
   if val == 10 {
      return "d12+5".to_string() + &bonus_string;
   }
   if val == 11 {
      return "d12+6".to_string() + &bonus_string;
   }
   if val == 12 {
      return "d12+7".to_string() + &bonus_string;
   }
   if val == 13 {
      return "d12+8".to_string() + &bonus_string;
   }
   if val == 14 {
      return "d12+9".to_string() + &bonus_string;
   }
   if val == 15 {
      return "d12+10".to_string() + &bonus_string;
   }
   if val == 16 {
      return "d12+11".to_string() + &bonus_string;
   }
   if val == 17 {
      return "d12+12".to_string() + &bonus_string;
   }

   return "n/a".to_string() + &bonus_string;
}