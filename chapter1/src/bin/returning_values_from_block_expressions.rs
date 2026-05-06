fn example() -> i32 {
    let x = 42;
    // Rust's ternary expression
    let v = if x < 42 { -1 } else { 1 }; // if is an expression, so it can return a value
    println!("from if: {}", v);

    let food = "hamburger";
    let result = match food {
        "hotdog" => "is hotdog",
        // notice the braces are optional when its just a single return expression
        _ => "is not hotdog",
    }; // we can assign the result of a match to a variable
    println!("identifying food: {}", result);

    let v = {
        // This scope block lets us get a result without polluting function scope
        let a = 1;
        let b = 2;
        a + b
    }; // the value of the block is the value of the last expression, so we can return a value from a block without explicitly using return
    println!("from block: {}", v);

    // The idiomatic way to return a value in rust from a function at the end
    v + 4 // we can omit the semicolon to return the value of the expression, but if we put a semicolon it becomes a statement and returns nothing
}

fn main() {
    println!("from function: {}", example());
}
