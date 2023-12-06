// FILL in the blanks
fn main() {
    let s: String = String::from("hello, world");

    // In two ways
    let slice1: &str = s.as_str();
    //  let slice1: &str = &s;
    assert_eq!(slice1, "hello, world");

    let slice2: &str = "hello";
    assert_eq!(slice2, "hello");

    let mut slice3: String = s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("Success!");
}
