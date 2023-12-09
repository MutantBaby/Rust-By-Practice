use std::fs::File;
use std::io::{self, Read};

fn read_file1() -> Result<String, io::Error> {
    let f: Result<File, io::Error> = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_file2() -> Result<String, io::Error> {
    let mut s: String = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    assert_eq!(
        read_file1().unwrap_err().to_string(),
        read_file2().unwrap_err().to_string()
    );

    println!("Success!");
}
