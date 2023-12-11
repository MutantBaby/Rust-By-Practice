// 1 Method
// fn invalid_output() -> String {
//     String::from("foo")
// }

// 2 Method
// fn invalid_output() -> &'static str {
//     "foo"
// }

// 3 Method
fn invalid_output<'a>(x: &'a String) -> &'a str {
    x
}

fn main() {
    let a: String = String::from("foo");

    let x: &str = invalid_output(&a);
}
