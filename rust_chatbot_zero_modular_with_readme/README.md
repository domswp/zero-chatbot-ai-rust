# Zero AI Chatbot

![Zero AI Chatbot](https://static.wikia.nocookie.net/megaman/images/9/9d/ZeroMMZ.png/revision/latest?cb=20100616051411)

Zero AI Chatbot adalah aplikasi CLI (Command Line Interface) yang dibangun dengan bahasa pemrograman Rust dan terintegrasi dengan DeepSeek API. Chatbot ini memiliki personality Zero dari seri game Megaman Zero, memberikan respons yang tenang, tegas, dan suportif layaknya karakter tersebut.

## Fitur

- **Personality Zero**: Chatbot dengan karakter Zero dari Megaman Zero
- **Prompt Modular**: Personality disimpan dalam file terpisah yang mudah diedit
- **Efek Thinking**: Animasi "berpikir" dengan frasa-frasa karakter Zero
- **Respons Berwarna**: Respons Zero ditampilkan dengan warna kuning
- **Integrasi DeepSeek API**: Menggunakan model DeepSeek Chat untuk respons AI berkualitas tinggi

## Prasyarat

Sebelum menggunakan Zero AI Chatbot, pastikan Anda telah menginstal:

- [Rust dan Cargo](https://www.rust-lang.org/tools/install)
- Paket pengembangan OpenSSL:
  - Ubuntu/Debian: `sudo apt-get install pkg-config libssl-dev`
  - Fedora/RHEL: `sudo dnf install openssl-devel`
  - Arch Linux: `sudo pacman -S openssl`
  - macOS (dengan Homebrew): `brew install openssl`

## Instalasi

1. Clone repositori ini:
   ```
   git clone https://github.com/username/zero-ai-chatbot.git
   cd zero-ai-chatbot
   ```

2. Buat file `.env` di direktori proyek dan tambahkan API key DeepSeek Anda:
   ```
   DEEPSEEK_API_KEY=your_api_key_here
   ```

3. Build proyek:
   ```
   cargo build
   ```

## Penggunaan

1. Jalankan chatbot:
   ```
   cargo run
   ```

2. Ketik pesan Anda dan tekan Enter untuk mengirim.

3. Untuk keluar dari chatbot, ketik `exit` atau `quit`.

## Mengubah Personality

Salah satu fitur utama Zero AI Chatbot adalah prompt modular yang memudahkan pengguna untuk mengubah personality chatbot tanpa perlu mengubah kode.

### Mengedit Personality Zero

1. Buka file `zero_prompt.txt` dengan editor teks.
2. Ubah isi file sesuai keinginan Anda.
3. Simpan file dan jalankan kembali chatbot untuk melihat perubahan.

### Membuat Personality Baru

1. Buat file baru, misalnya `custom_prompt.txt`.
2. Isi dengan personality yang diinginkan.
3. Ubah kode di `src/main.rs` untuk membaca dari file baru tersebut:
   ```rust
   // Ubah baris ini
   let prompt_file = "zero_prompt.txt";
   
   // Menjadi
   let prompt_file = "custom_prompt.txt";
   ```

## Struktur Proyek

```
zero-ai-chatbot/
├── src/
│   └── main.rs          # Kode utama chatbot
├── Cargo.toml           # Konfigurasi proyek dan dependensi
├── zero_prompt.txt      # File prompt personality Zero
├── .env                 # File konfigurasi API key (perlu dibuat)
├── .env.example         # Contoh file konfigurasi
└── README.md            # Dokumentasi proyek
```

## Dependensi

- `reqwest`: HTTP client untuk komunikasi dengan API
- `serde`: Serialisasi dan deserialisasi data JSON
- `tokio`: Runtime asinkron untuk Rust
- `dotenv`: Memuat variabel lingkungan dari file .env
- `anyhow`: Penanganan error yang lebih baik

## Kontribusi

Kontribusi selalu diterima! Jika Anda ingin berkontribusi pada proyek ini:

1. Fork repositori
2. Buat branch fitur baru (`git checkout -b feature/amazing-feature`)
3. Commit perubahan Anda (`git commit -m 'Add some amazing feature'`)
4. Push ke branch (`git push origin feature/amazing-feature`)
5. Buka Pull Request

## Lisensi

Proyek ini dilisensikan di bawah [MIT License](LICENSE).

## Pengembang

Dibuat dengan ❤️ oleh [Your Name]

## Ucapan Terima Kasih

- DeepSeek API untuk model AI
- Capcom untuk karakter Zero dari seri Megaman Zero
