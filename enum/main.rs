enum Operation {
    Add(fn(i32, i32) -> i32),
    Subtract(fn(i32, i32) -> i32),
}

enum PrimitiveValue {
    Integer(i32),
    Boolean(bool),
    Character(char),
    None,
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn describe_value(value: PrimitiveValue) {
    match value {
        PrimitiveValue::Integer(n) => {
            println!("It is an integer: {}", n);
        }
        PrimitiveValue::Boolean(b) => {
            println!("It is a boolean: {}", b);
        }
        PrimitiveValue::Character(c) => {
            println!("It is a character: '{}'", c);
        }
        PrimitiveValue::None => {
            println!("It is nothing.");
        }
    }
}

fn main() {
    let op = Operation::Add(add);

    match op {
        Operation::Add(f) => println!("Result: {}", f(2, 3)),
        Operation::Subtract(f) => println!("Result: {}", f(5, 2)),
    }

    let a = PrimitiveValue::Integer(42);
    let b = PrimitiveValue::Boolean(true);
    let c = PrimitiveValue::Character('R');
    let d = PrimitiveValue::None;

    describe_value(a);
    describe_value(b);
    describe_value(c);
    describe_value(d);
}
