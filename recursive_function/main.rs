fn main() {
    let fact = factorial(5);
    println!("5! = {}", fact);

    let fact = fibonacci(5);
    println!("5! = {}", fact);

    println!("{}", reverse("hello world"));
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn reverse(s: &str) -> String {
    if s.is_empty() {
        String::new()
    } else {
        let mut chars = s.chars();
        let first = chars.next().unwrap();
        let rest = chars.as_str();
        format!("{}{}", reverse(rest), first)
    }
}