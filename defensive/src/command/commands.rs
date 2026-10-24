use serde_json::{Value, json};
use std::fs;


pub fn command_lists() {
    let path: &str = "src/command/data.json"; // This is a temp json file for commands (" New commands will be added later on ...")
    let raw_data = fs::read_to_string(path).unwrap();
    let data: Value = serde_json::from_str(&raw_data).unwrap();
    println!("{}", data);
}
