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
    let option: u32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number for the selection.");
            return;
        }
    };

    let mut value = String::new();
    let mut valid_number = false;
    let mut number = 0.0;

    while !valid_number {
        println!("Enter the value for conversion:");
        value.clear();
        io::stdin().read_line(&mut value).expect("Failed to read input");

        match value.trim().parse::<f64>() {
            Ok(num) => {
                number = num;
                valid_number = true;
            }
            Err(_) => {
                println!("Please enter a valid number.");
            }
        }
    }

    match option {
        1 => println!("Result: {} kilometers", meters_to_kilometers::convert(number)),
        2 => println!("Result: {} kilograms", pounds_to_kilograms::convert(number)),
        3 => println!("Result: {} centimeters", inches_to_centimeters::convert(number)),
        _ => println!("Invalid option!"),
    }
}
