use std::io;

fn main() {
    println!("Welcome to Temperature Converter!");

    println!("Please enter a temperature in Fahrenheit.");

    let mut temperature_input = String::new();

    io::stdin()
        .read_line(&mut temperature_input)
        .expect("Failed to read line");

    let temperature_input: u32 = temperature_input
        .trim().parse().expect("Please type a number.");

    let temperature_output: u32 = (temperature_input - 32) * 5/9;

    println!("{} Fahrenheit is equal to {} Celsius.", temperature_input, temperature_output);
}
