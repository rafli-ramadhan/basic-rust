fn main() {
    // Ownership Example
    let s1 = String::from("Hello");
    let s2 = s1; // Ownership moved, s1 is no longer valid
    println!("{}", s2);
    // println!("{}", s1); // This would cause a compile-time error

    // Borrowing Example
    let s3 = String::from("World");
    print_string(&s3); // Pass by reference (borrow), s3 is still valid
    println!("{}", s3); // Safe to use again

    // Mutable Borrow Example
    let mut s4 = String::from("Hi");
    append_world(&mut s4); // Mutable borrow
    println!("{}", s4);
}

fn print_string(s: &String) {
    println!("Borrowed: {}", s);
}

fn append_world(s: &mut String) {
    s.push_str(" World");
}
