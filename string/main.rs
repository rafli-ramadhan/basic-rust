fn main() {
    // using &str
    let name_literal: &str = "hello1";
    println!("Literal (&str): {}", name_literal);
    let name_literal2 = name_literal;
    println!("Literal (&str): {}", name_literal);

    let raw = "raw"; // &str
    println!("Literal (&str): {}", raw);
    let raw2 = raw;
    println!("Literal (&str): {}", raw);


    // using ::from
    let name_string_from: String = String::from("hello2");
    println!("String::from: {}", name_string_from);
    let name_string_from2: String = name_string_from;
    // println!("String::from: {}", name_string_from); // error

    let mut name_string_from: String = String::from("hello2");
    println!("String::from: {}", name_string_from);
    let name_string_from2: String = name_string_from;
    name_string_from = name_string_from2;
    // println!("String::from: {}", name_string_from2); // error
    println!("String::from: {}", name_string_from);


    // to_string()
    let name_to_string: String = "hello3".to_string();
    println!("to_string(): {}", name_to_string);

    let number = 123;
    let number_string = number.to_string();
    println!("Integer to string: {}", number_string);


    // combined
    let greeting = get_greeting("hello4");
    println!("From function: {}", greeting);

    let name = "hello5";
    let combined = format!("Hello, {}", name);
    println!("Formatted string: {}", combined);
}

fn get_greeting(name: &str) -> String {
    format!("Hi, {}! Welcome to Rust.", name)
}
