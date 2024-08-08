//This is the standart library in Rust:
use std::io;
use colored::Colorize;

mod utils;

fn main() {
    println!("{}","Hello, world!".white().bold().italic());
    println!("{}","Try to sum two numbers!:".white().bold().italic());
    print!("\n\n{}", "First Number: ".white().bold().italic());
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failde to read line");
    //The before variable use the method to use the console input
    //whit string variables.
    let firts_number: f64 = input.trim().parse().unwrap();

    println!("{}", firts_number);

    print!("\n{}", "Second Number: ".white().bold().italic());
    input = String::new();
    io::stdin().read_line(&mut input).expect("failde to read line");
    //The before variable use the method to use the console input
    //whit string variables.
    let second_number: f64 = input.trim().parse().unwrap();

    println!("{}", second_number);

    println!("Now {} + {} = {}",firts_number.to_string().green().bold(),second_number.to_string().blue().bold(),utils::math_functions::sum(firts_number, second_number));
}

