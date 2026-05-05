fn main() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "found the 13"; // we can break with a value, and that value will be the result of the loop
        }
    };
    println!("from loop: {}", v); // from loop: found the 13
}
