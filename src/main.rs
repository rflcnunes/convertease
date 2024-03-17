use std::io;
use colored::*;

mod converters;

use converters::meters_to_kilometers;
use converters::pounds_to_kilograms;
use converters::inches_to_centimeters;
use converters::miles_to_kilometers;

fn main() {
    let mut option = 0;

    while option == 0 {
        println!("{}", "Select a conversion:".blue().bold());
        println!("1️: Meters to Kilometers");
        println!("2️: Pounds to Kilograms");
        println!("3️: Inches to Centimeters");
        println!("4️: Miles to Kilometers");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<u32>() {
            Ok(num) if num >= 1 && num <= 4 => {
                option = num;
            }
            Ok(_) => println!("{}", "Please select a valid option from the list.".red()),
            Err(_) => println!("{}", "Please enter a number.".red()),
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

    match option {
        1 =>
            println!(
                "Result: {:.2} kilometers",
                meters_to_kilometers::convert(number).to_string().green()
            ),
        2 =>
            println!(
                "Result: {:.2} kilograms",
                pounds_to_kilograms::convert(number).to_string().green()
            ),
        3 =>
            println!(
                "Result: {:.2} centimeters",
                inches_to_centimeters::convert(number).to_string().green()
            ),
        4 =>
            println!(
                "Result: {:.2} kilometers",
                miles_to_kilometers::convert(number).to_string().green()
            ),
        _ => unreachable!(),
    }
}
