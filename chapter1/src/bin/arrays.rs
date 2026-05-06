fn main() {
    let nums: [i32; 3] = [1, 2, 3]; // this is an array of 3 integers, the type annotation is optional because the compiler can infer it from the values
    println!("{:?}", nums); // we can print the whole array using the debug format specifier
    println!("{}", nums[1]); // we can access individual elements using indexing
}
