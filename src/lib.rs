use serde_json::{Map, Value};
use wasm_bindgen::prelude::*;

fn handle_array(array: &[Value], mut location: &mut String, mut collection: &mut Vec<String>) {
    match array.len() {
        0 => collection.push(format!("{} = []", location)),
        _ => {
            location.push('[');
            for (index, item) in array.iter().enumerate() {
                let new_position = format!("{}]", index.to_string());
                location.push_str(&new_position);
                handle_value(&item, &mut location, &mut collection);
                location.truncate(location.len() - new_position.len());
            }
            location.pop();
        }
    }
}

fn handle_object(
    object: &Map<String, Value>,
    mut location: &mut String,
    mut collection: &mut Vec<String>,
) {
    if location.len() != 1 {
        location.push('.');
    }

    match object.len() {
        0 => collection.push(format!("{} = {{}}", location)),
        _ => {
            for (key, value) in object.iter() {
                location.push_str(key);
                handle_value(&value, &mut location, &mut collection);
                location.truncate(location.len() - key.len());
            }
        }
    };

    location.pop();
}

fn handle_value(value: &Value, mut location: &mut String, mut collection: &mut Vec<String>) {
    match value {
        Value::Object(object) => handle_object(&object, &mut location, &mut collection),
        Value::Array(array) => handle_array(&array, &mut location, &mut collection),
        Value::Bool(boolean) => collection.push(format!("{} = {}", location, boolean.to_string())),
        Value::Number(number) => collection.push(format!("{} = {}", location, number)),
        Value::String(string) => {
            collection.push(format!(
                "{} = {}",
                location,
                format!("\"{}\"", string.to_string().escape_debug())
            ));
        }
        Value::Null => collection.push(format!("{} = {}", location, "null")),
    };
}

#[wasm_bindgen]
pub fn flatten(input: String) -> String {
    let rep: Value = serde_json::from_str(&input).unwrap();

    let mut location = String::new();
    location.push('.');

    let mut collection: Vec<String> = Vec::new();

    handle_value(&rep, &mut location, &mut collection);

    let mut return_value = String::new();
    for i in &collection {
        return_value += i;
        return_value += "\n";
    }

    return_value
}
