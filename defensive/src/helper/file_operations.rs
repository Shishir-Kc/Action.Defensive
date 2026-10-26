// use serde_json::{Value, json};
use std::fs::File;

pub fn create_file(name: &str, path: &str) -> Result<bool, String> {
    //! This is a helper function which will create a file on a specific path.
    //! It will return true if file was created else false and Err if any error occured !
    //! Best suggested to use match rather  then using try .
    //! Eg match create_file("test.txt","home/user/"){
    //! Ok(trus) => println!("file crated !"),
    //! Ok(false) => println!("file not created"),
    //! Err(e)=> print!("Error Occured {}",e);
    //! }
    let mut full_path = path.to_owned();
    full_path.push_str(name);
    println!("{}", full_path);
    match File::create_new(&full_path) {
        Ok(_) => Ok(true),
        Err(_e) => Err(String::from("Error occured on file operation.")),
    }
}
