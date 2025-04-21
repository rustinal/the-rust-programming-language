use std::io;
fn main() {
    // Convert temperatures between Fahrenheit and Celsius
    println!("Please enter the temperature, e.g. 50F or 10C");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    let (temp, unit) = input.split_at(input.len() - 1);
    let unit: String = unit.to_uppercase();
    if unit != "F" && unit != "C" {
        println!("Invalid unit. Please use 'F' for Fahrenheit or 'C' for Celsius.");
        return;
    }
    if temp.parse::<f64>().is_err() {
        println!("Invalid temperature. Please enter a valid number.");
        return;
    }
    println!("You entered {temp} degrees in {unit}.");
    let converted_temp: f64;
    if unit == "F" {
        converted_temp =
            fahrenheit_to_celsius(temp.parse::<f64>().expect("Failed to parse temperature"));
    } else if unit == "C" {
        converted_temp =
            celsius_to_fahrenheit(temp.parse::<f64>().expect("Failed to parse temperature"));
    } else {
        println!("Invalid unit. Please use 'F' for Fahrenheit or 'C' for Celsius.");
        return;
    }
    println!(
        "Converted temperature: {converted_temp} degrees in {}",
        if unit == "F" { "C" } else { "F" }
    );
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (9.0 * c) / 5.0 + 32.0
}
