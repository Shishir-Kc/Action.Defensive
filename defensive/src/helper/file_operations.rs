// use serde_json::{Value, json};
use std::{
    fs::{self, File},
    io::ErrorKind,
};

use serde_json::Value;

pub fn create_file(name: &str, path: &str) -> Result<bool, String> {
    /*! This is a helper function which will create a file on a specific path.
     It will return true if file was created else false and Err if any error occured !
     Best suggested to use match rather  then using try .
      - Eg match create_file("test.txt","home/user/"){
      - Ok(trus) => println!("file crated !"),
      - Ok(false) => println!("file not created"),
      - Err(e)=> print!("Error Occured {}",e);
      - }
    */
    let mut full_path = path.to_owned();
    full_path.push_str(name);
    match File::create_new(&full_path) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(false),
        Err(e) => Err(format!("Error on file operation {}", e)),
    }
}
#[allow(dead_code)] // it's a temp fix for JSON , not that it is a bug but more like un-used ;)
pub enum FileType {
    // This is a "Schema for supported file types
    TXT,
    JSON,
}

fn read_txt(full_path: String) -> String {
    /*!
        This function will only read txt file format and it is meant
        to be used internally by file_operations
    */
    fs::read_to_string(full_path).expect("Unnable to read File")
}

fn read_json(full_path: String) -> String {
    /*!
        This code will only read Json data !, It is only meant to be used
        by file_operations.
    */
    let raw_data = fs::read_to_string(full_path).unwrap();
    let data: Value = serde_json::from_str(&raw_data).unwrap();
    data.to_string()
}

pub fn read_file(name: &str, path: &str, type_of_file: FileType) -> String {
    /*!
        This Function is responsible for reading file!
        currently supported :
            - FileType::TXT
            - FileType::JSON

    */

    let mut full_path = path.to_owned();
    full_path.push_str(name);
    match type_of_file {
        FileType::TXT => read_txt(full_path),
        FileType::JSON => read_json(full_path),
    }
}
