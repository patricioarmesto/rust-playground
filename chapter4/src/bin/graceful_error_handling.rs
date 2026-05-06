fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

fn main() -> Result<(), String> {
    // Look at how much code we saved!
    let v = do_something_that_might_fail(42)?; // the ? operator is a shortcut for propagating errors, it will return the error from main if do_something_that_might_fail returns an error, otherwise it will return the value inside the Ok variant
    println!("found {}", v);
    Ok(())
}
