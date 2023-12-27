// FIX the errors
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    let slice1: &[i32] = &v[..];
    let slice2: &[i32] = &v[0..v.len()];

    assert_eq!(slice1, slice2);

    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);

    let slice3: &[i32] = &v[0..v.len()];

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!");
}
