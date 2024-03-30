// Fungsi utama yang akan dijalankan pertama kali
fn main() {
    // Mendeklarasikan variabel `name` dengan nilai "Toni"
    let name = "Toni";
    // Memanggil fungsi `greet` dengan `name` sebagai argumen
    greet(name);
}

// Fungsi `greet` yang menerima satu argumen `name` bertipe &str (reference ke string)
fn greet(name: &str) {
    // Mencetak pesan sapaan ke console
    println!("Hello, {}!", name);
}
