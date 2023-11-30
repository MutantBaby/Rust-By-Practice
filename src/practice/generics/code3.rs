use std::{fmt::Debug, usize};

fn print_array<T, const N: usize>(arr: [T; N])
where
    T: Debug,
{
    println!("{:?}", arr);
}

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    print_array(arr);

    let arr: [&str; 2] = ["hello", "world"];
    print_array(arr);
}
