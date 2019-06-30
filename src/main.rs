use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};

use serde_json::{Deserializer, Map, Value};

use colored::*;

fn handle_array(array: &[Value], mut location: &mut String) {
    match array.len() {
        0 => println!("{} = []", location),
        _ => {
            location.push('[');
            for (index, item) in array.iter().enumerate() {
                let new_position = format!("{}]", index.to_string().cyan());
                location.push_str(&new_position);
                handle_value(&item, &mut location);
                location.truncate(location.len() - new_position.len());
            }
            location.pop();
        },
    }
}

fn handle_object(object: &Map<String, Value>, mut location: &mut String) {
    if location.len() != 1 {
        location.push('.');
    }

    match object.len() {
        0 => println!("{} = {{}}", location),
        _ => {
            for (key, value) in object.iter() {
                location.push_str(key);
                handle_value(&value, &mut location);
                location.truncate(location.len() - key.len());
            }
        },
    }

    location.pop();
}

fn handle_value(value: &Value, mut location: &mut String) {
    match value {
        Value::Object(object) => handle_object(&object, &mut location),
        Value::Array(array) => handle_array(&array, &mut location),
        Value::Bool(boolean) => println!("{} = {}", location, boolean.to_string().yellow()),
        Value::Number(number) => println!("{} = {}", location, number),
        Value::String(string) => println!("{} = \"{}\"", location, string.to_string().bright_black()),
        Value::Null => println!("{} = {}", location, "null".red()),
    };
}

fn main() -> io::Result<()> {
    let input = env::args().nth(1);;
    let reader: Box<BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap())),
    };

    let stream = Deserializer::from_reader(reader).into_iter::<Value>();

    for value in stream {
        let mut location = String::new();
        location.push('.');
        handle_value(&value.unwrap(), &mut location);
    }

    Ok(())
}
