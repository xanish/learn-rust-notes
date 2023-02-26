use std::io;

fn main() {
    println!("Which fibonacci number do you need? ");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read N!");
    
    let n: u32 = n.trim().parse().expect("Invalid N value!");

    println!("The fibonacci number is {}", fibonacci(n));
}

fn fibonacci(n: u32) -> u64 {
    if n <= 0 {
        return 0;
    }
    else if n <= 3 {
        // into to convert to required return type
        return n.into();
    }

    fibonacci(n-1) + fibonacci(n-2)
}
