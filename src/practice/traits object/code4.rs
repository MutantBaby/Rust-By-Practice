trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

// IMPLEMENT below with generics.
fn static_dispatch<T>(x: T)
where
    T: Foo,
{
    x.method();
}

// IMPLEMENT below with trait objects.
fn dynamic_dispatch(x: &dyn Foo) {
    x.method();
}

fn main() {
    let x: u8 = 5u8;
    let y: String = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!");
}
