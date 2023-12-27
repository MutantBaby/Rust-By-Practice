fn main() {
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);

    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    let i3: i32 = i32::from('a' as u8);
    let i3: i32 = ('a' as u8).into();

    let s: String = String::from('a');
    let s: String = 'a'.into();

    println!("Success!");
}
