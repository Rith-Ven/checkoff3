fn borrow_text(text: &String) {
    println!("Borrowed: {}", text);
}

fn take_text(text: String) {
    println!("Taken: {}", text);
} // The owned String is dropped here.

fn main() {
    let ocean = String::from("ocean");

    borrow_text(&ocean);              // A
    println!("{}", ocean);           // B

    let reef = ocean;                // C
    borrow_text(&reef);              // D

    println!("{}", ocean);           // E

    take_text(reef);                  // F
    println!("{}", reef);            // G
}
