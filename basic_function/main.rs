fn main() {
    // Function with parameter
    greet_user("Ramadhan");

    // Function with return value
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);

    // Function with multiple return values
    let (min, max) = min_max(10, 4);
    println!("min = {}, max = {}", min, max);
}

fn greet_user(name: &str) {
    println!("Username, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}