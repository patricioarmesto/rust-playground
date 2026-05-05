fn main() {
    let x = 12; // by default this is i32
    let a = 12u8; // this is an unsigned 8-bit integer
    let b = 4.3; // by default this is f64
    let c = 4.3f32; // this is a 32-bit floating point number
    let d = 'r'; // unicode character
    let ferris = '🦀'; // also a unicode character
    let bv = true; // boolean value
    let t = (13, false); // this is a tuple, it can contain multiple values of different types
    let sentence = "hello world!"; // this is a string slice, it is a reference to a string literal
    println!(
        "{} {} {} {} {} {} {} {} {} {}",
        x, a, b, c, d, ferris, bv, t.0, t.1, sentence
    );
}
