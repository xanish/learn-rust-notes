fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    // will throw an error if x is not mutable
    x = 6;
    println!("The value of x is: {x}");
}
