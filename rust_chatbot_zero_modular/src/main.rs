use anyhow::{Context, Result};
use dotenv::dotenv;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::path::Path;

// ANSI color codes
const YELLOW: &str = "\x1b[33m";
const GRAY: &str = "\x1b[90m";
const RESET: &str = "\x1b[0m";

// Struktur untuk request ke DeepSeek API
#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

// Struktur untuk pesan dalam format yang diharapkan oleh DeepSeek API
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    role: String,
    content: String,
}

// Struktur untuk respons dari DeepSeek API
#[derive(Deserialize, Debug)]
struct ChatResponse {
    id: Option<String>,
    choices: Vec<Choice>,
}

// Struktur untuk pilihan dalam respons
#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

// Fungsi untuk membaca prompt dari file
fn read_prompt_from_file(file_path: &str) -> Result<String> {
    fs::read_to_string(file_path)
        .context(format!("Gagal membaca file prompt: {}", file_path))
}

// Fungsi untuk menampilkan efek thinking
fn display_thinking() {
    let thinking_phrases = [
        "Menganalisis situasi...",
        "Memproses data...",
        "Mengaktifkan protokol cyber-elf...",
        "Mengkalibrasi saber...",
        "Menyiapkan respons...",
    ];
    
    for phrase in thinking_phrases.iter() {
        print!("\r{}{}{}", GRAY, phrase, RESET);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(500));
    }
    println!();
}

// Fungsi utama
fn main() -> Result<()> {
    // Memuat variabel lingkungan dari file .env (jika ada)
    dotenv().ok();
    
    // Mendapatkan API key dari variabel lingkungan
    let api_key = env::var("DEEPSEEK_API_KEY")
        .context("API key tidak ditemukan. Pastikan DEEPSEEK_API_KEY telah diatur di variabel lingkungan atau file .env")?;
    
    // Membuat HTTP client
    let client = Client::new();
    
    // Menyimpan riwayat percakapan
    let mut conversation_history: Vec<Message> = Vec::new();
    
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
    
    // Menambahkan pesan sistem untuk personality Zero dari file
    conversation_history.push(Message {
        role: "system".to_string(),
        content: prompt_content,
    });
    
    println!("=== Zero AI Chatbot ===");
    println!("Ketik pesan Anda dan tekan Enter untuk mengirim.");
    println!("Ketik 'exit' atau 'quit' untuk keluar.");
    
    // Loop utama chatbot
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        // Membaca input pengguna
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        
        // Menghapus whitespace di awal dan akhir input
        let user_input = user_input.trim();
        
        // Memeriksa apakah pengguna ingin keluar
        if user_input.eq_ignore_ascii_case("exit") || user_input.eq_ignore_ascii_case("quit") {
            println!("{}Zero: Sampai jumpa, sahabat. Semoga perjalananmu selalu dipenuhi keberanian.{}", YELLOW, RESET);
            break;
        }
        
        // Menambahkan pesan pengguna ke riwayat percakapan
        conversation_history.push(Message {
            role: "user".to_string(),
            content: user_input.to_string(),
        });
        
        // Menampilkan efek thinking
        display_thinking();
        
        // Membuat request ke DeepSeek API
        match send_message_to_deepseek(&client, &api_key, &conversation_history) {
            Ok(response) => {
                // Mendapatkan respons dari AI
                if let Some(choice) = response.choices.first() {
                    let ai_message = &choice.message;
                    
                    // Menampilkan respons dengan warna kuning
                    println!("{}Zero: {}{}", YELLOW, ai_message.content, RESET);
                    
                    // Menambahkan respons AI ke riwayat percakapan
                    conversation_history.push(ai_message.clone());
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    
    Ok(())
}

// Fungsi untuk mengirim pesan ke DeepSeek API
fn send_message_to_deepseek(
    client: &Client,
    api_key: &str,
    conversation_history: &[Message],
) -> Result<ChatResponse> {
    // URL endpoint DeepSeek API
    let url = "https://api.deepseek.com/v1/chat/completions";
    
    // Membuat request body
    let request_body = ChatRequest {
        model: "deepseek-chat".to_string(), // Menggunakan model DeepSeek Chat biasa
        messages: conversation_history.to_vec(),
        temperature: 0.7,
        max_tokens: 1000,
    };
    
    // Mengirim request ke DeepSeek API
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .context("Gagal mengirim request ke DeepSeek API")?;
    
    // Memeriksa status code
    if !response.status().is_success() {
        // Simpan status code sebelum mengkonsumsi response dengan text()
        let status = response.status();
        let error_text = response.text()?;
        return Err(anyhow::anyhow!(
            "API error ({}): {}",
            status,
            error_text
        ));
    }
    
    // Mengurai respons JSON
    let chat_response = response
        .json::<ChatResponse>()
        .context("Gagal mengurai respons JSON dari DeepSeek API")?;
    
    Ok(chat_response)
}
