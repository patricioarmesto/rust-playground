struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f is dropped here
}

fn main() {
    let mut foo = Foo { x: 42 };
    let f = &mut foo; // f is a mutable reference to foo, it does not own the data

    // FAILURE: do_something(foo) would fail because
    // foo cannot be moved while mutably borrowed

    // FAILURE: foo.x = 13; would fail here because
    // foo is not modifiable while mutably borrowed

    f.x = 13;
    // f is dropped here because it's no longer used after this point

    println!("{}", foo.x);

    // this works now because all mutable references were dropped
    foo.x = 7;

    // move foo's ownership to a function
    do_something(foo);
}
