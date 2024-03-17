use std::io;
use colored::*;

mod converters;

use converters::meters_to_kilometers;
use converters::pounds_to_kilograms;
use converters::inches_to_centimeters;
use converters::miles_to_kilometers;

fn main() {
    let conversions = vec![
        ("Meters to Kilometers", meters_to_kilometers::convert as fn(f64) -> f64),
        ("Pounds to Kilograms", pounds_to_kilograms::convert as fn(f64) -> f64),
        ("Inches to Centimeters", inches_to_centimeters::convert as fn(f64) -> f64),
        ("Miles to Kilometers", miles_to_kilometers::convert as fn(f64) -> f64)
    ];

    let mut option = 0usize;

    while option == 0 {
        println!("{}", "Select a conversion:".blue().bold());
        for (i, (name, _)) in conversions.iter().enumerate() {
            println!("{}: {}", i + 1, name);
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= conversions.len() => {
                option = num;
            }
            _ => println!("{}", "Please select a valid option from the list.".red()),
        }
    }

    println!("{}", "Enter the value for conversion:".green().bold());
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read input");
    let number: f64 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Please enter a valid number.".red());
            return;
        }
    };

    let conversion = conversions[option - 1].1;
    let result = conversion(number);

    println!("{} {:.2} {} {:.2}", "Result:".green().bold(), number, "=>".yellow().bold(), result);
}
