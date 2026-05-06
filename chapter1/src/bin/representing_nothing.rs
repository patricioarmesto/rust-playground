#![allow(dead_code)] // this line prevents compiler warnings

enum Item {
    Inventory(String),
    // None represents the absence of an item
    None,
}

struct BagOfHolding {
    item: Item,
}

fn main() {
    let bag = BagOfHolding {
        item: Item::Inventory(String::from("sword")),
    };

    match bag.item {
        Item::Inventory(name) => println!("The bag contains a {}.", name),
        Item::None => println!("The bag is empty."),
    }
}
