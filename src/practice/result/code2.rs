use std::num::ParseIntError;

fn add_two_with_map(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().map(|x| x + 2)
}

fn add_two_with_andthen(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().and_then(|x| Ok(x + 2))
}

fn main() {
    assert_eq!(add_two_with_map("4").unwrap(), 6);
    assert_eq!(add_two_with_andthen("4").unwrap(), 6);

    println!("Success!");
}
