use std::io::{self, Write};
use std::str::FromStr;

#[derive(Debug)]
enum Unit {
    Celsius, Fahrenheit, Kelvin, Meter, Kilometer, Gram, Kilogram, Pound, Second, Minute,
}

impl Unit {
    fn convert(&self, value: f64, to: &Unit) -> f64 {
        match (self, to) {
            (Unit::Celsius, Unit::Fahrenheit) => value * 1.8 + 32.0,
            (Unit::Fahrenheit, Unit::Celsius) => (value - 32.0) / 1.8,
            (Unit::Celsius, Unit::Kelvin) => value + 273.15,
            (Unit::Kelvin, Unit::Celsius) => value - 273.15,
            (Unit::Fahrenheit, Unit::Kelvin) => (value - 32.0) / 1.8 + 273.15,
            (Unit::Kelvin, Unit::Fahrenheit) => (value - 273.15) * 1.8 + 32.0,
            (Unit::Meter, Unit::Kilometer) => value / 1000.0,
            (Unit::Kilometer, Unit::Meter) => value * 1000.0,
            (Unit::Gram, Unit::Kilogram) => value / 1000.0,
            (Unit::Kilogram, Unit::Gram) => value * 1000.0,
            (Unit::Pound, Unit::Kilogram) => value * 0.453592,
            (Unit::Kilogram, Unit::Pound) => value / 0.453592,
            (Unit::Second, Unit::Minute) => value / 60.0,
            (Unit::Minute, Unit::Second) => value * 60.0,
            _ => value, // Same unit
        }
    }
}

impl FromStr for Unit {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim().to_lowercase().as_str() {
            "celsius" | "c" => Ok(Unit::Celsius),
            "fahrenheit" | "f" => Ok(Unit::Fahrenheit),
            "kelvin" | "k" => Ok(Unit::Kelvin),
            "meter" | "m" => Ok(Unit::Meter),
            "kilometer" | "km" => Ok(Unit::Kilometer),
            "gram" | "g" => Ok(Unit::Gram),
            "kilogram" | "kg" => Ok(Unit::Kilogram),
            "pound" | "lb" => Ok(Unit::Pound),
            "second" | "s" => Ok(Unit::Second),
            "minute" | "min" => Ok(Unit::Minute),
            _ => Err(()),
        }
    }
}

fn main() {
    println!("Unit Converter\n1. Use an expression (e.g., 10Kg -> g)\n2. Enter units and values manually");
    let choice = read_input("Choose an option: ");
    match choice.trim() {
        "1" => handle_expression(),
        "2" => handle_manual_input(),
        _ => println!("Invalid choice!"),
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn handle_expression() {
    let expr = read_input("Enter a conversion expression (e.g., 10Kg -> g): ");
    if let Some((value, from_unit, to_unit)) = parse_expression(&expr) {
        let result = from_unit.convert(value, &to_unit);
        println!("{} {:?} = {} {:?}", value, from_unit, result, to_unit);
    } else {
        println!("Invalid format! Use '10Kg -> g'.");
    }
}

fn handle_manual_input() {
    let from_unit = get_unit("Choose a unit to convert from: ");
    let to_unit = get_unit("Choose a unit to convert to: ");
    let value = read_input("Enter the value to convert: ").trim().parse::<f64>().unwrap();
    let result = from_unit.convert(value, &to_unit);
    println!("{} {:?} = {} {:?}", value, from_unit, result, to_unit);
}

fn get_unit(prompt: &str) -> Unit {
    let unit_str = read_input(prompt);
    Unit::from_str(unit_str.trim()).unwrap_or_else(|_| panic!("Invalid unit!"))
}

fn parse_expression(expression: &str) -> Option<(f64, Unit, Unit)> {
    let parts: Vec<&str> = expression.split("->").map(|s| s.trim()).collect();
    if parts.len() == 2 {
        let value_unit = parts[0];
        let to_unit_str = parts[1];

        let value: f64 = value_unit.chars().take_while(|c| c.is_digit(10) || *c == '.').collect::<String>().parse().unwrap();
        let from_unit_str: String = value_unit.chars().skip_while(|c| c.is_digit(10) || *c == '.').collect();

        if let (Ok(from_unit), Ok(to_unit)) = (Unit::from_str(&from_unit_str), Unit::from_str(to_unit_str)) {
            return Some((value, from_unit, to_unit));
        }
    }
    None
}
