fn main() {
    let mut v: Vec<i32> = Vec::from([1, 2, 3]);

    for i in 0..5 {
        println!("{:?}", v.get(i))
    }

    for i in 0..5 {
        match v.get(i) {
            Some(&x) => v[i] = x + 1,
            None => v.push(i as i32 + 2),
        }
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");
}
