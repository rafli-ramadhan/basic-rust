fn main() {
    println!("==== If else ====");
    let number = 7;
    if number < 5 {
        println!("Number is less than 5");
    } else if number == 5 {
        println!("Number is exactly 5");
    } else {
        println!("Number is greater than 5");
    }

    println!("==== For loop ====");
    for i in 0..3 {
        println!("for loop: {}", i);
    }
    
    println!("==== Loop ====");
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("Loop reached 3, breaking.");
            break;
        }
    }

    println!("==== White loop ====");
    let mut n = 0;
    while n < 3 {
        println!("while loop: {}", n);
        n += 1;
    }

    println!("==== Match (like switch) ====");
    let grade = 'B';
    match grade {
        'A' => println!("Excellent"),
        'B' => println!("Good"),
        'C' => println!("Average"),
        _ => println!("Needs improvement"),
    }

    println!("==== Match with return value ====");
    let number = 1;
    let result = match number {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "other",
    };
    println!("Match result: {}", result);
}
