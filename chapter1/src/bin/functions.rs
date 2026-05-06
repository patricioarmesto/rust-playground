fn add(x: i32, y: i32) -> i32 {
    return x + y; // we can use the return keyword to return a value from a function, but it is not required if the last expression in the function is the value we want to return
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y // we can omit the return keyword and the semicolon for the last expression in a function, it will be returned implicitly
}

fn main() {
    println!("42 + 13 = {}", add(42, 13));
    println!("42 - 13 = {}", subtract(42, 13));
}
