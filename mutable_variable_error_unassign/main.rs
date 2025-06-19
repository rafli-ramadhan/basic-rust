fn main() {
    let test: String; // deklarasi
    // println!("{}", test); // used binding `test` is possibly-uninitialized
    test = String::from("Hello"); // inisialisasi sebelum digunakan
    println!("{}", test);
}