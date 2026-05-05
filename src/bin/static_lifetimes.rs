static PI: f64 = 3.1415;

fn main() {
    // static variables can also be scoped to a function
    static SECRET: &'static str = "swordfish";

    // string literals have a 'static lifetime
    let msg: &'static str = "Hello World!";
    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);

    unsafe {
        SECRET = "shark"; // this is unsafe because
    }
    println!("{}", SECRET);
}
