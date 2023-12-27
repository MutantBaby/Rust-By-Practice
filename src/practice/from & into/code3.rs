use std::fs;
use std::io;
use std::num;

#[derive(Debug)]
enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        CliError::IoError(value)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(value: num::ParseIntError) -> Self {
        CliError::ParseError(value)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? automatically converts io::Error to CliError
    let contents = fs::read_to_string(&file_name)?;

    // num::ParseIntError -> CliError
    let num = contents.trim().parse()?;

    Ok(num)
}

fn main() {
    let val: Result<i32, CliError> = open_and_parse_file("lib");

    println!("{:?}", val);

    println!("Success!");
}
