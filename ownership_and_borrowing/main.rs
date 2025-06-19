fn main() {
    let text1 = String::from("Rust immutable");

    println!("=== Ownership ===");
    let str1 = text1;
    println!("Value of str1: {}", str1);

    println!("=== Borrowing (Immutable Reference) ===");
    let str2 = &str1;
    println!("Value of str1: {}", str1);
    println!("Value of str2: {}", str2);
    println!("Memory address of s3: {:p}", str2);

    let mut text2 = String::from("Rust mutable");

    println!("=== Borrowing (Mutable Reference) ===");
    append_world(&mut text2);
    println!("s4 setelah dimodifikasi: {}", text2);

    println!("=== Ownership with Functions ===");
    let name1 = String::from("Fadil");
    takes_ownership(name1);

    let age = 30;
    makes_copy(age);
    println!("age masih bisa dipakai: {}", age);

    let name2: &str = "Umar";
    let name3 = name2;
    println!("Name: {}", name2);
    println!("Name: {}", name2);
}

// Fungsi dengan borrowing (mutable)
fn append_world(s: &mut String) {
    s.push_str(" example!");
}

// Fungsi mengambil ownership
fn takes_ownership(name: String) {
    println!("Name, {}!", name);
}

// Fungsi menerima tipe Copy (i32)
fn makes_copy(age: i32) {
    println!("Name: {}", age);
}
