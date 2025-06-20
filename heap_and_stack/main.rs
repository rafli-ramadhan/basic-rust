fn main() {
    let string1 = String::from("hello"); // heap
    println!("Address of string1         : {:p}", &string1);

    let string2 = "world".to_string(); // heap
    let string3 = string1; // heap
    println!("Address of string2         : {:p}", &string2);
    println!("Address of string3         : {:p}", &string3);

    let bool_flag = true; // stack
    println!("Address of bool_flag       : {:p}", &bool_flag);

    let stack_var = 123; // stack
    let heap_var = Box::new(456); // heap

    println!("Alamat stack_var           : {:p}", &stack_var);
    println!("Alamat heap_var            : {:p}", &heap_var);
    println!("Alamat isi heap_var        : {:p}", heap_var);
}
