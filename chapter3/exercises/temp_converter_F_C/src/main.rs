use std::io;

enum ConverterMode {
    CelsiusToFahrenheit,
    FahrenheitToCelsius
}

fn display_options_text() {
    println!("Please choose a conversion mode to start and press enter:");
    println!(" ");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
}

#[warn(dead_code)]
fn handle_input() {
    //placeholder
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    (c * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(f: f32) -> f32  {
    ((f - 32.0) * 5.0)/9.0
}

fn temp_converter(mode: ConverterMode, temp_to_be_converted: f32) -> f32 {
    match mode {
        ConverterMode::CelsiusToFahrenheit => return celsius_to_fahrenheit(temp_to_be_converted),
        ConverterMode::FahrenheitToCelsius => return fahrenheit_to_celsius(temp_to_be_converted),
    }
}


fn main() {
    println!("Welcome to the Temperature Converter CLI app");

    loop{
        display_options_text();

        let mut mode = String::new();
        let mut temp = String::new();

        io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");

        let mode: ConverterMode = match mode.trim().parse() {
            Ok(1) => ConverterMode::CelsiusToFahrenheit,
            Ok(2) => ConverterMode::FahrenheitToCelsius,
            Ok(_) => {
                println!("Please choose a valid option!");
                continue;
            }
            Err(_) => continue,
        };

        println!(" ");
        println!("Now enter the temp you want to convert from:");

        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(temp) => temp,
            Err(_) => continue,
        };

        let result = temp_converter(mode, temp);

        println!(" ");
        println!(" ");
        println!("Your result is: {result}");
        println!(" ");
        println!(" ");
    }
}