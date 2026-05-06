fn main() {
    let a = 13u8; // this is an unsigned 8-bit integer
    let b = 7u32; // this is a 32-bit unsigned integer
    let c = a as u32 + b; // we need to convert a to u32 before we can add it to b
    println!("{}", c);

    let t = true;
    println!("{}", t as u8); // we can convert a boolean to an integer, true becomes 1 and false becomes 0
}
