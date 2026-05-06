#![allow(dead_code)] // this line prevents compiler warnings

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam,
}
enum PoisonType {
    Acidic,
    Painful,
    Lethal,
}
enum Size {
    Big,
    Small,
}
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None,
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

fn main() {
    // SeaCreature's data is on stack
    let ferris = SeaCreature {
        // String struct is also on stack,
        // but holds a reference to data on heap
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small), // we can store data in an enum variant, and that data can be of any type, including other enums
    };

    match ferris.species {
        Species::Crab => match ferris.weapon {
            Weapon::Claw(num_claws, size) => {
                let size_description = match size {
                    Size::Big => "big",
                    Size::Small => "small",
                }; // we can match on the data inside the enum variant to get more information about it
                println!(
                    "ferris is a crab with {} {} claws",
                    num_claws, size_description
                )
            }
            _ => println!("ferris is a crab with some other weapon"),
        },
        _ => println!("ferris is some other animal"),
    }
}
