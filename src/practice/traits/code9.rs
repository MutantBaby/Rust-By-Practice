#[derive(Debug, Copy, Clone)]
struct User<T, U> {
    val1: T,
    val2: U,
}

impl<T, U> User<T, U> {
    fn new(val1: T, val2: U) -> Self {
        Self { val1, val2 }
    }

    fn update1<V, W>(&self, oth: &User<V, W>) -> User<V, U>
    where
        V: Clone,
        U: Clone,
    {
        let val1: V = oth.val1.clone();
        let val2: U = self.val2.clone();

        User { val1, val2 }
    }

    fn update2<V, W, X, Y>(&self, oth: &User<V, W>) -> User<X, Y>
    where
        V: Into<X> + Clone,
        U: Into<Y> + Clone,
    {
        let val1: X = oth.val1.clone().into();
        let val2: Y = self.val2.clone().into();

        User { val1, val2 }
    }
}

fn main() {
    let user1: User<String, i32> = User::new("John".to_string(), 42);
    let user2: User<char, f64> = User::new('F', 30.5);

    let upd_user1: User<char, i32> = user1.update2(&user2);
    let upd_user2: User<String, f64> = user2.update1(&user1);

    println!("Val1: {}, Val2: {}", upd_user1.val1, upd_user2.val2);
}
