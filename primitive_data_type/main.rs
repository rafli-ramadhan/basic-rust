fn main() {
    // string
    let mut name: String = String::from("Alice");
    println!("Name before: {}", name);
    name = String::from("Bob");
    println!("Name after: {}", name);

    // integer
    let mut age: i32 = 25;
    println!("Age before: {}", age);
    age += 1;
    println!("Age after: {}", age);

    let mut balance: i64 = 1_000_000;
    println!("Balance before: {}", balance);
    balance -= 250_000;
    println!("Balance after: {}", balance);

    let mut price: isize = 1_000_000;
    println!("Balance before: {}", price);
    price -= 200_000;
    println!("Balance after: {}", price);

    // boolean
    let mut is_active: bool = true;
    println!("Is active before: {}", is_active);
    is_active = false;
    println!("Is active after: {}", is_active);
}
