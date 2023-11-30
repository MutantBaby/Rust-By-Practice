struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, oth: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: oth.y,
        }
    }
}

fn main() {
    let p1: Point<i32, i32> = Point { x: 5, y: 10 };
    let p2: Point<&str, char> = Point {
        x: "Hello",
        y: '中',
    };

    let p3: Point<i32, char> = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}
