// 1st Approach with static dispatching
trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> Self {
        *self
    }
}

impl MyTrait for String {
    fn f(&self) -> Self {
        self.clone()
    }
}

fn my_function<T>(x: T) -> T
where
    T: MyTrait,
{
    x.f()
}

fn main() {
    let a = my_function(13_u32);
    let b = my_function(String::from("abc"));

    println!("Success! {:?} | {:?}", a, b);
}
