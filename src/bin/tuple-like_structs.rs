struct Location(i32, i32);

fn main() {
    // This is still a struct on a stack
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1); // we can access the fields of a tuple struct using dot notation and the index of the field
}
