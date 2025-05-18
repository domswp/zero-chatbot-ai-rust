# Penjelasan Kode Chatbot CLI Rust dengan DeepSeek API

## Struktur Proyek
Proyek ini terdiri dari beberapa komponen utama:
1. `Cargo.toml` - File konfigurasi proyek Rust
2. `src/main.rs` - Kode utama aplikasi chatbot CLI

## Penjelasan Dependensi
Dalam file `Cargo.toml`, kita menggunakan beberapa library penting:

- `reqwest`: Library HTTP client untuk melakukan request ke API DeepSeek
- `serde` dan `serde_json`: Library untuk serialisasi dan deserialisasi data JSON
- `tokio`: Runtime asinkron untuk Rust
- `dotenv`: Untuk memuat variabel lingkungan dari file .env
- `anyhow`: Library untuk penanganan error yang lebih baik

## Penjelasan Sintaks Rust

### Struktur Data (Struct)
```rust
#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}
```

- `#[derive(Serialize)]` adalah atribut makro yang secara otomatis mengimplementasikan trait `Serialize` untuk struct ini
- `struct` adalah kata kunci untuk mendefinisikan struktur data
- Setiap field memiliki tipe data yang eksplisit (String, Vec<Message>, f32, u32)

### Fungsi
```rust
fn main() -> Result<()> {
    // Kode fungsi
}
```

- `fn` adalah kata kunci untuk mendefinisikan fungsi
- `main()` adalah fungsi utama yang dijalankan saat program dimulai
- `-> Result<()>` menunjukkan bahwa fungsi mengembalikan tipe `Result` yang berisi `()` (unit type) jika sukses, atau error jika gagal

### Penanganan Error
```rust
let api_key = env::var("DEEPSEEK_API_KEY")
    .context("API key tidak ditemukan. Pastikan DEEPSEEK_API_KEY telah diatur di variabel lingkungan atau file .env")?;
```

- Operator `?` digunakan untuk propagasi error. Jika `env::var()` mengembalikan error, fungsi akan langsung return dengan error tersebut
- `.context()` menambahkan informasi konteks ke error

### Loop dan Kondisional
```rust
loop {
    // Kode loop
    
    if user_input.eq_ignore_ascii_case("exit") || user_input.eq_ignore_ascii_case("quit") {
        println!("Terima kasih telah menggunakan chatbot!");
        break;
    }
}
```

- `loop` membuat loop tak terbatas yang akan terus berjalan sampai menemui `break`
- `if` digunakan untuk kondisional
- `break` digunakan untuk keluar dari loop

### Pattern Matching
```rust
match send_message_to_deepseek(&client, &api_key, &conversation_history) {
    Ok(response) => {
        // Kode jika sukses
    }
    Err(e) => {
        // Kode jika error
    }
}
```

- `match` adalah ekspresi pattern matching yang kuat di Rust
- `Ok` dan `Err` adalah varian dari enum `Result`

## Cara Kerja Kode

1. Program dimulai dengan memuat API key dari variabel lingkungan
2. Membuat HTTP client untuk komunikasi dengan API
3. Memulai loop interaktif untuk menerima input pengguna
4. Mengirim pesan pengguna ke DeepSeek API
5. Menampilkan respons dari API
6. Menyimpan riwayat percakapan untuk konteks

## Cara Menggunakan

1. Buat file `.env` di direktori proyek dengan isi:
   ```
   DEEPSEEK_API_KEY=your_api_key_here
   ```

2. Jalankan aplikasi dengan perintah:
   ```
   cargo run
   ```

3. Ketik pesan dan tekan Enter untuk berkomunikasi dengan AI
4. Ketik 'exit' atau 'quit' untuk keluar dari aplikasi

## Catatan Penting

- Pastikan URL endpoint dan nama model DeepSeek sudah benar sesuai dokumentasi terbaru
- Kode ini menggunakan reqwest blocking client untuk kesederhanaan, tetapi untuk aplikasi yang lebih kompleks, pendekatan asinkron lebih disarankan
- Parameter seperti temperature dan max_tokens dapat disesuaikan sesuai kebutuhan
