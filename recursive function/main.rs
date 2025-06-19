fn main() {
    // Faktorial
    let n = 5;
    println!("Faktorial dari {} adalah {}", n, factorial(n));

    // Fibonacci
    for i in 0..10 {
        println!("Fibonacci ke-{} adalah {}", i, fibonacci(i));
    }
}

// Fungsi rekursif untuk faktorial
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Fungsi rekursif untuk fibonacci
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
