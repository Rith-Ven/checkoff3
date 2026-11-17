fn show(text: &String) {
    println!("{}", text);
}

fn main() {
    let first = String::from("coral");

    show(&first);                    // A
    println!("{}", first);           // B

    let second = first;              // C

    println!("{}", first);           // D
    show(&second);                   // E

    {
        let third = String::from("reef");
        show(&third);                // F
    }

    println!("{}", third);           // G
}
