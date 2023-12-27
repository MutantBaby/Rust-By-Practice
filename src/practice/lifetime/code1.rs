// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {}
fn longest<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str
where
    'a: 'c,
    'b: 'c,
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let val: &str = longest("12", "123");

    println!("{}", val);
}
