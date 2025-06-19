use std::collections::HashMap;

fn main() {
    println!("==== Array ====");
    let mut arr_num: [i32; 3] = [1, 2, 3];
    println!("Array before: {:?}", arr_num);
    arr_num[0] = 4;
    arr_num[1] = 5;
    arr_num[2] = 6;
    println!("Array after: {:?}", arr_num);
   
    println!("==== Slice ====");
    let arr_score = [9.1, 9.2, 9.3, 9.4];
    let mut slice_num = &arr_score[0..4];
    println!("Slice before: {:?}", slice_num);

    let new_arr_score = [9.5, 9.6, 9.7, 9.8, 9.9];
    slice_num = &new_arr_score[1..5];
    println!("Slice after: {:?}", slice_num);
    
    println!("==== Tuple ====");
    let mut user_score: (i32, String, f64, bool) = (1, "andika".to_string(), 6.4, true);
    println!("Tuple before: {:?}", user_score);
    user_score.0 = 2;
    user_score.1 = "andika_permana".to_string();
    user_score.2 = 7.2;
    user_score.3 = false;
    println!("Tuple after: {:?}", user_score);

    println!("==== Struct ====");
    #[derive(Debug)]
    struct Person {
        id: i32,
        name: String,
        age: u8,
        score: f32,
        is_active: bool,
    }
    let mut person = Person {
        id: 1,
        name: String::from("Rezi"),
        age: 30,
        score: 91.2,
        is_active: true,
    };
    println!("Struct before: {:?}", person);
    person.id = 2;
    person.name = String::from("Rezi Rahardian");
    person.age = 27;
    person.score = 99.99;
    person.is_active = false;
    println!("Struct after: {:?}", person);

    println!("==== HashMap ====");
    let mut grades = HashMap::new();
    grades.insert("Angga", 91.2);
    grades.insert("Wira", 92.3);
    grades.insert("Luthfi", 92.1);
    println!("HashMap before: {:?}", grades);
    grades.insert("Ramadhan", 99.99);
    grades.insert("Yahya", 99.99);
    println!("HashMap after: {:?}", grades);
}
