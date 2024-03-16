mod converters;

use converters::meters_to_kilometers;
use converters::pounds_to_kilograms;
use converters::inches_to_centimeters;
use std::io;

fn main() {
    println!("Select a conversion:");
    println!("1: Meters to Kilometers");
    println!("2: Pounds to Kilograms");
    println!("3: Inches to Centimeters");

    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read input");
    let option: u32 = option.trim().parse().expect("Please enter a number");

    println!("Enter the value for conversion:");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read input");
    let value: f64 = value.trim().parse().expect("Please enter a valid number");

    match option {
        1 => println!("Result: {} kilometers", meters_to_kilometers::convert(value)),
        2 => println!("Result: {} kilograms", pounds_to_kilograms::convert(value)),
        3 => println!("Result: {} centimeters", inches_to_centimeters::convert(value)),
        _ => println!("Invalid option!"),
    }
}
