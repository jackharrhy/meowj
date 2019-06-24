use std::io::{self, Read};
use serde_json::{Map, Value};

fn handle_array(array: &Vec<Value>, mut location: &mut String) {
    location.push('[');
    for (index, item) in array.iter().enumerate() {
        let new_position = format!("{}]", index);
        location.push_str(&new_position);
        handle_value(&item, &mut location);
        location.truncate(location.len() - new_position.len());
    }
    location.pop();
}

fn handle_object(object: &Map<String, Value>, mut location: &mut String) {
    if location.len() != 1 {
        location.push('.');
    }

    for (key, value) in object.iter() {
        location.push_str(key);
        handle_value(&value, &mut location);
        location.truncate(location.len() - key.len());
    }
    location.pop();
}

fn handle_value(value: &Value, mut location: &mut String) {
    match value {
        Value::Object(object) => handle_object(&object, &mut location),
        Value::Array(array) => handle_array(&array, &mut location),
        Value::Bool(boolean) => println!("{} = {}", location, boolean),
        Value::Number(number) => println!("{} = {}", location, number),
        Value::String(string) => println!("{} = \"{}\"", location, string),
        Value::Null => (),
    };
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;

    let base: Value = serde_json::from_str(&buffer)?;

    let mut location = String::new();
    location.push('.');

    handle_value(&base, &mut location);

    Ok(())
}
