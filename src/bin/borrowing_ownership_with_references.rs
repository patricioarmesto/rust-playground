struct Foo {
    x: i32,
}

fn main() {
    let foo = Foo { x: 42 };
    let f = &foo; // f is a reference to foo, it does not own the data
    println!("{}", f.x);
    // f is dropped here
    // foo is dropped here
}
