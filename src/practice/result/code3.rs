use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    match n1_str.parse::<i32>() {
        Ok(n1) => match n2_str.parse::<i32>() {
            Ok(n2) => Ok(n1 * n2),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    n1_str
        .parse::<i32>()
        .and_then(|n1| n2_str.parse::<i32>().map(|n2| n1 * n2))
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("Ans is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let twenty: Result<i32, ParseIntError> = multiply1("10", "2");
    print(twenty);

    let tt: Result<i32, ParseIntError> = multiply("t", "2");
    print(tt);

    println!("Success!");
}
