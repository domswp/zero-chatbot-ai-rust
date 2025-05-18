# Penjelasan Modifikasi Chatbot Zero dengan Interleaving Pesan

## Perubahan yang Dilakukan

1. **Personality Zero**
   - Ditambahkan pesan sistem dengan personality Zero dari Megaman Zero
   - Diubah tampilan prompt dan pesan perpisahan untuk mencerminkan karakter Zero

2. **Model DeepSeek Reasoner**
   - Diubah model dari "deepseek-chat" menjadi "deepseek-reasoner" untuk kemampuan penalaran yang lebih baik

3. **Fitur Thinking**
   - Ditambahkan fungsi `display_thinking()` untuk menampilkan animasi "berpikir"
   - Menggunakan frasa-frasa yang sesuai dengan karakter Zero
   - Menggunakan warna abu-abu untuk efek thinking

4. **Warna Respons Kuning**
   - Ditambahkan konstanta ANSI color codes untuk warna kuning, abu-abu, dan reset
   - Diimplementasikan warna kuning pada semua respons Zero

5. **Perbaikan Interleaving Pesan**
   - Ditambahkan fungsi `prepare_messages_for_reasoner()` untuk memastikan pesan user dan assistant selalu berselang-seling
   - Mengatasi error "deepseek-reasoner does not support successive user or assistant messages"
   - Memastikan pesan sistem tetap dipertahankan di awal

## Penjelasan Kode Baru

### Struktur ChatResponse yang Diperbarui
```rust
struct ChatResponse {
    id: Option<String>,
    choices: Vec<Choice>,
}
```
Field `id` diubah menjadi `Option<String>` untuk mengatasi kemungkinan nilai null dari API.

### Fungsi Prepare Messages for Reasoner
```rust
fn prepare_messages_for_reasoner(conversation_history: &[Message]) -> Vec<Message> {
    // Selalu sertakan pesan sistem di awal jika ada
    let mut prepared_messages = Vec::new();
    let mut has_system = false;
    
    // Cek apakah ada pesan sistem
    if !conversation_history.is_empty() && conversation_history[0].role == "system" {
        prepared_messages.push(conversation_history[0].clone());
        has_system = true;
    }
    
    // Mulai dari pesan pertama setelah sistem (jika ada)
    let start_idx = if has_system { 1 } else { 0 };
    
    // Pastikan pesan terakhir adalah dari user
    let mut last_role = "assistant";
    
    for i in start_idx..conversation_history.len() {
        let message = &conversation_history[i];
        
        // Jika pesan saat ini dan sebelumnya memiliki role yang sama, lewati
        if message.role == last_role {
            continue;
        }
        
        prepared_messages.push(message.clone());
        last_role = &message.role;
    }
    
    prepared_messages
}
```
Fungsi ini memastikan bahwa:
1. Pesan sistem tetap dipertahankan di awal
2. Pesan user dan assistant selalu berselang-seling
3. Tidak ada dua pesan berturut-turut dengan role yang sama

### Penggunaan Fungsi Prepare Messages
```rust
// Menyiapkan pesan untuk DeepSeek Reasoner (memastikan interleaving)
let prepared_messages = prepare_messages_for_reasoner(&conversation_history);

// Membuat request ke DeepSeek API
match send_message_to_deepseek(&client, &api_key, &prepared_messages) {
    // ...
}
```
Sebelum mengirim pesan ke API, kita memastikan bahwa pesan-pesan tersebut sudah dalam format yang benar.

## Cara Menggunakan

1. Pastikan Anda telah menginstal dependensi yang diperlukan:
   ```
   sudo apt-get update
   sudo apt-get install -y pkg-config libssl-dev
   ```

2. Buat file `.env` dengan API key DeepSeek Anda:
   ```
   DEEPSEEK_API_KEY=your_api_key_here
   ```

3. Jalankan chatbot:
   ```
   cargo run
   ```

4. Berinteraksi dengan Zero dan nikmati pengalaman chatting dengan karakter Zero!

## Catatan Penting

- Model DeepSeek Reasoner memiliki batasan bahwa pesan user dan assistant harus selalu berselang-seling
- Jika Anda masih mengalami error, pastikan API key Anda valid dan memiliki akses ke model DeepSeek Reasoner
- Pastikan terminal Anda mendukung ANSI color codes untuk melihat warna kuning dan abu-abu
