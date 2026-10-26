mod command;
mod helper;
use command::commands::command_lists;
use helper::{file_operations::FileType, file_operations::create_file, file_operations::read_file};
use std::io;

fn main() {
    let dummy_name = "test.txt";
    let dummy_path = "/home/x64_x86/";
    println!("This is a defensive Program ! ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    print!("{}", user_input);
    command_lists();
    match create_file(&dummy_name, &dummy_path) {
        Ok(true) => print!("File creted "),
        Ok(false) => print!("File not created it already Exist."),
        Err(e) => print!("Error {}", e),
    };
    println!("{}", read_file(&dummy_name, &dummy_path, FileType::TXT));
}
