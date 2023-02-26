fn main() {
    let number = 3;

    // it is mandatory for the condition to be a bool
    // unlike other languages if (3) {} will not evaluate
    // to true by converting the type to bool
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // infinite loop
    // loop {
    //     println!("again!");
    // }

    // returning value from loop on break
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    loop_label_selective_breaking();
    while_test();
    for_test();
}

fn loop_label_selective_breaking() {
    let mut count = 0;
    
    // yes label must begin with single quote
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // will break the current inner loop
                break;
            }
            if count == 2 {
                // will break out of the outer loop
                // labeled counting_up
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_test() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_test() {
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is: {element}");
    }

    // print from 1 to 3
    for number in (1..4) {
        println!("{number}!");
    }

    // reverse print from 3 to 1
    for number in (1..4).rev() {
        println!("{number}!");
    }
}