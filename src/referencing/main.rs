/*
 * Code with issue
 * m1 and m2 get deallocated after their ownership gets moved to g1 and g2
 * hence we cannot use them to print again
 */
// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(m1, m2);
//     let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved
// }

// fn greet(g1: String, g2: String) {
//     println!("{} {}!", g1, g2);
// }

/*
 * Possible solution (a bit too verbose, ugly)
 * return the ownership back from greet() and assign to new variables?
 * then use the updated variables to print
 */
// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     let (m1_again, m2_again) = greet(m1, m2);
//     let s = format!("{} {}", m1_again, m2_again);
// }

// fn greet(g1: String, g2: String) -> (String, String) {
//     println!("{} {}!", g1, g2);
//     (g1, g2)
// }

/*
 * Clean rust preferred solution - use pointers
 * just pass the reference of m1 and m2 to greet
 * when passing reference ownership is not transferred since
 * reference is to the variables m1 and m2 and not the Box / heap location with value
 */
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
}

fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);
}
