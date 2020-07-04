use std::io;

// Fahrenheit to celsius conversion

fn main() {
    let mut temperature = String::new();

    println!("Enter your temperature in Fahrenheit!");

    io::stdin()
        .read_line(&mut temperature)
        .expect("Please enter a valid input");

    // shadowing for type conversion
    let temperature = temperature
        .trim()
        .parse()
        .expect("Please enter a valid number");
    let temp_celsius: f32 = convert_to_celsius(temperature);
    println!("{}F => {}C", temperature, temp_celsius);
}

fn convert_to_celsius(temperature: f32) -> f32 {
    (temperature - 32.0) * 5.0 / 9.0
}
