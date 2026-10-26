mod command;
mod helper;
use helper::{file_operations::FileType, file_operations::read_file};

fn main() {
    let dummy_name = "packages.json";
    let dummy_path = "";
    println!("This is a defensive Program ! ");
    println!("{}", read_file(&dummy_name, &dummy_path, FileType::JSON));
}
