#![allow(dead_code)] // this line prevents compiler warnings

struct SeaCreature {
    // String is a struct
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    let squidward = SeaCreature {
        animal_type: "squid".to_string(),
        name: "squidward".to_string(),
        arms: 8,
        legs: 0,
        weapon: "ink".to_string(),
    };
    println!("{} is a {}", squidward.name, squidward.animal_type);
}
