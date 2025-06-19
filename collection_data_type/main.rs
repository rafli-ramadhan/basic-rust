fn main() {
    // Membuat vector kosong bertipe i32
    let mut numbers: Vec<i32> = Vec::new();

    // Menambahkan elemen ke vector
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    // Menampilkan seluruh isi vector
    println!("Isi vector: {:?}", numbers);

    // Mengakses elemen dengan indexing
    println!("Elemen pertama: {}", numbers[0]);

    // Menggunakan match untuk akses aman
    match numbers.get(2) {
        Some(value) => println!("Elemen ketiga: {}", value),
        None => println!("Elemen tidak ditemukan."),
    }

    // Iterasi dengan for
    for num in &numbers {
        println!("Angka: {}", num);
    }

    // Menghapus elemen terakhir
    numbers.pop();
    println!("Setelah pop: {:?}", numbers);
}
