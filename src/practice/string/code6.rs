// FILL in the blanks
use std::mem;

fn main() {
    let story: String = String::from("Rust By Practice");

    // Prevent automatically dropping of the String's data
    let mut story: mem::ManuallyDrop<String> = mem::ManuallyDrop::new(story);

    let len: usize = story.len();
    let ptr: *mut u8 = story.as_mut_ptr();
    let capacity: usize = story.capacity();

    assert_eq!(16, len);

    println!("{} | {:?} | {}", len, ptr, capacity);

    // We can rebuild a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    // let s: String = unsafe { String::from_raw_parts(ptr, 8, capacity) };
    // assert_eq!("Rust By ", s);

    let s: String = unsafe { String::from_raw_parts(ptr, len, capacity) };
    assert_eq!(*story, s);

    println!("Success!");
}
