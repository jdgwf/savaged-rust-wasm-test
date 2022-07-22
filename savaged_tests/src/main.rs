extern crate dotenv;

use dotenv::dotenv;
use std::env;

use savaged_libs;
use savaged_libs::json_data::json_save_db_row::JSONSaveDBRow;
// use savaged_libs::json_data::json_chargen_data::JSONChargenData;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut api_key: String = "".to_string();
    for (key, value) in env::vars() {
        if key == "APIKEY" {
            api_key = value.to_string();
        }
    }
    println!("apikey: {}", api_key);
    let available_data = savaged_libs::utils::get_chargen_data::get_chargen_data( api_key.to_string() ).await;
    let user_saves_json = savaged_libs::utils::get_user_saves::get_user_saves( api_key.to_string() ).await;


    println!("available_data.len() {:#?}", available_data.len() );
    println!("user_saves.len() {:#?}", user_saves_json.len() );
    // println!("user_saves {:#?}", user_saves_json );


    let user_saves: Vec<JSONSaveDBRow> = serde_json::from_str(&user_saves_json).unwrap();

    let mut pc = savaged_libs::player_character::PlayerCharacter::new( available_data.clone() );
    // for count in  0..100000 {
    //     pc.reset();

    //     let new_name = "New PC!".to_string() + &" #".to_string() + &count.to_string();
    //     pc.set_name( new_name  );
    //     pc.set_attribute_selected_agility(1);
    //     pc.set_attribute_selected_smarts(0);
    //     pc.set_attribute_selected_spirit(1);
    //     pc.set_attribute_selected_strength(1);
    //     pc.set_attribute_selected_vigor(2);

    //     // pc.set_attribute_bonus_vigor(2);

    //     // pc.set_uuid("67e55044-10b1-426f-9247-bb680e5fe0c8".to_string());

    //     // println!("Count # {}", count + 1);
    //     // println!("PC Name: {}", pc.name );
    //     // println!("UUID: {}", pc.uuid );
    //     // println!("* Agility {}", pc.attributes().agility_hr() );
    //     // println!("* Smarts {}", pc.attributes().smarts_hr() );
    //     // println!("* Spirit {}", pc.attributes().spirit_hr() );
    //     // println!("* Strength {}", pc.attributes().strength_hr() );
    //     // println!("* Vigor {}", pc.attributes().vigor_hr() );

    //     // println!("-----------------------------");


    // }
    for save in user_saves {
        if save.save_type == "character" && !save.deleted {
            pc.reset();
            pc.import_json( save.data );

            println!("PC Name: {}", pc.name );
            println!("UUID: {}", pc.uuid );
            println!("* Agility {}", pc.attributes().agility_hr() );
            println!("* Smarts {}", pc.attributes().smarts_hr() );
            println!("* Spirit {}", pc.attributes().spirit_hr() );
            println!("* Strength {}", pc.attributes().strength_hr() );
            println!("* Vigor {}", pc.attributes().vigor_hr() );

            break;
        }
    }
    println!("# books len: {:#?}", pc.get_available_books().len() );
    println!("# edges len: {:#?}", pc.get_available_hindrances().len() );
    println!("# hindrances len: {:#?}", pc.get_available_hindrances().len() );

    println!("# armor len: {:#?}", pc.get_available_armor().len() );
    println!("# weapons len: {:#?}", pc.get_available_weapons().len() );
    println!("# gear len: {:#?}", pc.get_available_gear().len() );

}
