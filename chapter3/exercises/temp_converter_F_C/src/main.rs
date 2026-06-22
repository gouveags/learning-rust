use std::io;

enum ConverterMode {
    CelsiusToFahrenheit,
    FahrenheitToCelsius
}

fn welcome_text() {
    println!("Welcome to the Temperature Converter CLI app");
}

fn display_options_text() {
    println!("Please choose a conversion mode to start and then press enter key to confirm:");
    println!(" ");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
}

fn display_result_text(result: &f32, mode: &ConverterMode) {
    let target_temp_alias = match mode.cmp(&ConverterMode) {
        ConverterMode::CelsiusToFahrenheit => "F",
        ConverterMode::FahrenheitToCelsius => "ºC",
    }

    println!(" ");
    println!(" ");
    println!("Your result is: {result}{target_temp_alias}");
    println!(" ");
    println!(" ");
}

fn input_temp_text() {
    println!(" ");
    println!("Now enter the temp you want to convert from and then press enter key to confirm:");
}

fn handle_mode_input() -> ConverterMode  {
    loop {
        let mut mode = String::new();

        io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");

        return match mode.trim().parse() {
            Ok(1) => ConverterMode::CelsiusToFahrenheit,
            Ok(2) => ConverterMode::FahrenheitToCelsius,
            Ok(_) => {
                println!("Please choose a valid option!");
                continue;
            }
            Err(_) => continue,
        };
    }
}

fn handle_temp_input() -> f32 {
    loop {
        input_temp_text();

        let mut temp = String::new();

        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

        return match temp.trim().parse() {
            Ok(temp) => temp,
            Err(_) => continue,
        };
    }
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    (c * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(f: f32) -> f32  {
    ((f - 32.0) * 5.0)/9.0
}

fn temp_converter(mode: &ConverterMode, temp_to_be_converted: &f32) -> f32 {
    match mode {
        ConverterMode::CelsiusToFahrenheit => return celsius_to_fahrenheit(temp_to_be_converted),
        ConverterMode::FahrenheitToCelsius => return fahrenheit_to_celsius(temp_to_be_converted),
    }
}

fn main() {
    welcome_text();

    loop{
        display_options_text();

        let mode: ConverterMode = handle_mode_input();

        let temp: f32 = handle_temp_input();

        let result = temp_converter(&mode, &temp);

        display_result_text(&result, &mode);
    }
}