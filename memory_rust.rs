fn main() {
    let first = Box::new(10);
    let second = first; // Ownership moves to second.

    // println!("{}", first); // Compile error: first was moved.
    println!("{}", second);
} // second is dropped; the heap allocation is freed.
