use std::io;
mod command;
mod helper;
use command::commands::command_lists;
use helper::file_operations::create_file;

fn main() {
    println!("This is a defensive Program ! ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    print!("{}", user_input);
    command_lists();
    match create_file("test.txt", "/home/x64_x86/") {
        Ok(true) => print!("File creted "),
        Ok(false) => print!("File not created "),
        Err(e) => print!("Error {}", e),
    }
}
