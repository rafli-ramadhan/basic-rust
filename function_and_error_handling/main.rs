fn main() {
    match divide(10.0, 2.0) {
        Some(result) => println!("The result is: {}", result),
        None => println!("Error: Cannot divide by zero!"),
    }

    match divide(10.0, 0.0) {
        Some(result) => println!("The result is: {}", result),
        None => println!("Error: Cannot divide by zero!"),
    }

    match safe_divide(10.0, 2.0) {
        Ok(res) => println!("The result is: {}", res),
        Err(e)  => eprintln!("Error: {}", e),
    }

    match safe_divide(5.0, 0.0) {
        Ok(res) => println!("The result is: {}", res),
        Err(e)  => eprintln!("Error: {}", e),
    }

    match compute_sqrt(-4.0) {
        Ok(v) => println!("Hasil: {}", v),
        Err(code) => eprintln!("Error code: {}", code),
    }
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero!".into())
    } else {
        Ok(a / b)
    }
}

fn compute_sqrt(value: f64) -> Result<f64, i32> {
    if value < 0.0 {
        Err(500)
    } else {
        Ok(value.sqrt())
    }
}
