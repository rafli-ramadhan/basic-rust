fn main() {
    println!("==== Vector ====");

    let mut grades: Vec<(String, f64)> = Vec::new();

    // insert data
    grades.push(("Angga".to_string(), 91.2));
    grades.push(("Wira".to_string(), 92.3));
    grades.push(("Luthfi".to_string(), 92.1));
    println!("Before: {:?}", grades);

    // insert new data
    grades.push(("Ramadhan".to_string(), 99.99));
    grades.push(("Yahya".to_string(), 99.99));
    println!("After: {:?}", grades);

    // update value
    for (name, score) in &mut grades {
        if name == "Angga" {
            *score = 97.0;
        }
    }
    println!("After: {:?}", grades);

    // delete data
    grades.retain(|(name, _)| name != "Wira");
    println!("After: {:?}", grades);
}
