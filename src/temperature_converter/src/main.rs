use std::io;

fn main() {
    println!("Enter temperature to convert: ");

    let mut temperature = String::new();
    let mut unit = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read temperature!");

    let temperature: f64 = temperature
        .trim()
        .parse()
        .expect("Invalid temperature value!");

    println!("Convert to unit? (1 for Celcius, 2 for Farenheit) ");

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read unit to convert to!");

    let unit: u8 = unit.trim().parse().expect("Failed to read unit!");

    if unit == 1 {
        println!("{}F is equivalent to {}C", temperature, to_celcius(temperature));
    }
    else if unit == 2 {
        println!("{}C is equivalent to {}F", temperature, to_farenheit(temperature));
    }
    else {
        println!("Invalid unit!");
    }
}

fn to_celcius(temperature: f64) -> f64 {
    (temperature - 32.0) * 5.0 / 9.0
}

fn to_farenheit(temperature: f64) -> f64 {
    (temperature * (9.0 / 5.0)) + 32.0
}
