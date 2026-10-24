use std::io;
mod command;
use command::commands::command_lists;


fn main() {
    println!("This is a defensive Program ! ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    print!("{}",user_input);
    command_lists();
}
