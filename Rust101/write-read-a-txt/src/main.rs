use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file_name = "a_txt.txt";
    let contents = "This a string, which is written in a txt file.";
    match write_a_string_to_a_txt(file_name, contents) {
        Ok(()) => println!("File is written successfully."),
        Err(error) => eprintln!("Error met: {}", error),
    }

    println!("Let's read what we wrote just now.");

    match read_from_txt(file_name) {
        Ok(buffer) => println!("Check what's in the file: {}", buffer),
        Err(error) => eprintln!("Error met: {}", error),
    }
}

fn write_a_string_to_a_txt(file_name: &str, contents: &str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn read_from_txt(file_name: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_name)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}
