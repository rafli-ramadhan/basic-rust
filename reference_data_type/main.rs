use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("==== Immutable Reference ====");
    let num = 40;
    let ref_num = &num;
    println!("Immutable reference: {}", ref_num);

    println!("==== Mutable Reference ====");
    let mut score = 100;
    {
        let ref_score = &mut score;
        *ref_score += 50;
        *ref_score -= 30;
        println!("Value before : {}", ref_score);
    }
    println!("Value after : {}", score);

    println!("==== Boxed Pointer ====");
    let boxed = Box::new(500);
    println!("Boxed value: {}", boxed);

    println!("==== Rc (Reference Counted) ====");
    let rc1 = Rc::new(String::from("Hello"));
    let rc2 = Rc::clone(&rc1);
    println!("Rc pointer 1: {}", rc1);
    println!("Rc pointer 2: {}", rc2);
    println!("Rc count: {}", Rc::strong_count(&rc1));

    println!("==== RefCell (Interior Mutability) ====");
    let cell = RefCell::new(10);
    *cell.borrow_mut() += 5;
    println!("RefCell value after mutation: {}", cell.borrow());

    println!("==== Arc (Atomic Reference Counted) ====");
    let arc_data = Arc::new(999);
    let arc_clone = Arc::clone(&arc_data);

    let handle = thread::spawn(move || {
        println!("Arc in thread: {}", arc_clone);
    });

    println!("Arc in main: {}", arc_data);
    handle.join().unwrap();
}
