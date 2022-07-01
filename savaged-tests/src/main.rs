use savagedlib;

fn main() {
    let mut pc = savagedlib::player_character::PlayerCharacter::new();

    let new_name = "New PC!".to_string();
    pc.set_name( new_name  );
    pc.set_attribute_selected_agility(2);
    pc.set_attribute_selected_smarts(1);
    pc.set_attribute_selected_spirit(2);
    pc.set_attribute_selected_strength(2);
    pc.set_attribute_selected_vigor(3);

    pc.set_attribute_bonus_vigor(2);

    pc.set_uuid("67e55044-10b1-426f-9247-bb680e5fe0c8".to_string());

    println!("PC Name: {}", pc.name() );
    println!("UUID: {}", pc.uuid() );
    println!("* Agility {}", pc.attributes().agility_hr() );
    println!("* Smarts {}", pc.attributes().smarts_hr() );
    println!("* Spirit {}", pc.attributes().spirit_hr() );
    println!("* Strength {}", pc.attributes().strength_hr() );
    println!("* Vigor {}", pc.attributes().vigor_hr() );
}
