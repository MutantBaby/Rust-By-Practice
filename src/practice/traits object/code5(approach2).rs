// 1st Approach with dynamic dispatching
use std::fmt::Debug;

trait MyTrait: Debug {
    fn f(&self) -> Box<dyn MyTrait>;
}

impl MyTrait for u32 {
    fn f(&self) -> Box<dyn MyTrait> {
        Box::new(*self)
    }
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn MyTrait> {
        Box::new(self.clone())
    }
}

fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
    x.f()
}

fn main() {
    let a: Box<dyn MyTrait> = my_function(Box::new(13_u32));
    let b: Box<dyn MyTrait> = my_function(Box::new(String::from("abc")));

    println!("Success! {:?} | {:?}", a, b);
}
