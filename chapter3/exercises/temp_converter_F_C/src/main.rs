use std::io;

enum ConverterMode {
    CelsiusToFahrenheit,
    FahrenheitToCelsius,
}

fn welcome_text() {
    println!("Welcome to the Temperature Converter CLI app");
}

fn display_options_text() {
    println!("Please choose a conversion mode to start and press enter:");
    println!();
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
}

fn display_result_text(result: f32, mode: &ConverterMode) {
    let target_temp_alias = match mode {
        ConverterMode::CelsiusToFahrenheit => "F",
        ConverterMode::FahrenheitToCelsius => "C",
    };

    println!();
    println!();
    println!("Your result is: {result}{target_temp_alias}");
    println!();
    println!();
}

fn input_temp_text() {
    println!();
    println!("Now enter the temp you want to convert from and press enter:");
}

fn handle_mode_input() -> ConverterMode {
    loop {
        let mut mode = String::new();

        io::stdin()
            .read_line(&mut mode)
            .expect("Failed to read line");

        match mode.trim().parse() {
            Ok(1) => return ConverterMode::CelsiusToFahrenheit,
            Ok(2) => return ConverterMode::FahrenheitToCelsius,
            Ok(_) => {
                println!("Please choose a valid option!");
            }
            Err(_) => {
                println!("Please enter 1 or 2.");
            }
        }
    }
}

fn handle_temp_input() -> f32 {
    loop {
        input_temp_text();

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        match temp.trim().parse() {
            Ok(temp) => return temp,
            Err(_) => {
                println!("Please enter a valid temperature.");
            }
        }
    }
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    ((fahrenheit - 32.0) * 5.0) / 9.0
}

fn temp_converter(mode: &ConverterMode, temp_to_be_converted: f32) -> f32 {
    match mode {
        ConverterMode::CelsiusToFahrenheit => celsius_to_fahrenheit(temp_to_be_converted),
        ConverterMode::FahrenheitToCelsius => fahrenheit_to_celsius(temp_to_be_converted),
    }
}

fn main() {
    welcome_text();

    loop {
        display_options_text();

        let mode = handle_mode_input();
        let temp = handle_temp_input();
        let result = temp_converter(&mode, temp);

        display_result_text(result, &mode);
    }
}
