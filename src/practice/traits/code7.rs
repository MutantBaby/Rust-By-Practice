use std::fmt::Debug;
// FIX the errors.
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Pair<T>
where
    T: Debug + PartialOrd,
{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Unit(i32);

fn main() {
    let pair: Pair<Unit> = Pair {
        x: Unit(1),
        y: Unit(3),
    };

    pair.cmp_display();
}
