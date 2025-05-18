# Penjelasan Modifikasi Chatbot Zero dengan Prompt Modular

## Perubahan yang Dilakukan

1. **Prompt Modular dalam File Eksternal**
   - Personality Zero sekarang disimpan dalam file terpisah `zero_prompt.txt`
   - Memudahkan pengeditan personality tanpa perlu mengubah kode
   - Implementasi pembacaan prompt dari file eksternal

2. **Perubahan Model ke DeepSeek Chat**
   - Diubah dari "deepseek-reasoner" menjadi "deepseek-chat"
   - Menghindari error parsing JSON yang terjadi dengan model Reasoner
   - Tidak memerlukan interleaving pesan user/assistant

3. **Fitur Thinking dan Warna Kuning Tetap Dipertahankan**
   - Animasi "berpikir" dengan frasa-frasa karakter Zero
   - Respons berwarna kuning untuk karakter Zero

## Penjelasan Kode Baru

### Fungsi Membaca Prompt dari File
```rust
// Fungsi untuk membaca prompt dari file
fn read_prompt_from_file(file_path: &str) -> Result<String> {
    fs::read_to_string(file_path)
        .context(format!("Gagal membaca file prompt: {}", file_path))
}
```
Fungsi ini membaca isi file prompt dan mengembalikannya sebagai string.

### Pencarian dan Pembacaan File Prompt
```rust
// Path ke file prompt
let prompt_file = "zero_prompt.txt";
let prompt_path = Path::new(prompt_file);

// Jika file prompt tidak ada di direktori saat ini, coba cari di direktori proyek
let prompt_content = if prompt_path.exists() {
    read_prompt_from_file(prompt_file)?
} else {
    let project_prompt_path = Path::new("rust_chatbot_project").join(prompt_file);
    read_prompt_from_file(project_prompt_path.to_str().unwrap())?
};
```
Kode ini mencari file prompt di direktori saat ini atau di direktori proyek, lalu membacanya.

### Penggunaan Model DeepSeek Chat
```rust
let request_body = ChatRequest {
    model: "deepseek-chat".to_string(), // Menggunakan model DeepSeek Chat biasa
    messages: conversation_history.to_vec(),
    temperature: 0.7,
    max_tokens: 1000,
};
```
Model diubah dari "deepseek-reasoner" menjadi "deepseek-chat" untuk menghindari error.

## Cara Menggunakan dan Mengedit Prompt

1. **Menjalankan Chatbot**:
   ```
   cargo run
   ```

2. **Mengedit Personality Zero**:
   - Buka file `zero_prompt.txt` dengan editor teks
   - Ubah isi file sesuai keinginan
   - Simpan file
   - Jalankan kembali chatbot untuk melihat perubahan

3. **Membuat Personality Baru**:
   - Buat file baru, misalnya `x_prompt.txt`
   - Isi dengan personality yang diinginkan
   - Ubah kode di `main.rs` untuk membaca dari file baru tersebut
   - Jalankan chatbot

## Catatan Penting

- Pastikan file prompt berada di direktori yang benar (direktori saat ini atau direktori proyek)
- Jika ingin menambahkan personality baru, cukup buat file prompt baru dan ubah path di kode
- Pastikan terminal Anda mendukung ANSI color codes untuk melihat warna kuning dan abu-abu
