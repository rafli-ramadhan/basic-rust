use std::collections::HashMap;

fn main() {
    println!("==== HashMap ====");

    let mut grades = HashMap::new();
    
    // insert
    grades.insert("Angga", 91.2);
    grades.insert("Wira", 92.3);
    grades.insert("Luthfi", 92.1);
    println!("HashMap before: {:?}", grades);
    grades.insert("Ramadhan", 99.99);
    grades.insert("Yahya", 99.99);
    println!("HashMap after: {:?}", grades);

    // update
    grades.insert("Angga", 95.0);
    println!("HashMap after: {:?}", grades);

    // delete
    grades.remove("Wira");
    println!("HashMap after: {:?}", grades);
}
