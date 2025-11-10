use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use anyhow::Result;
use sha2::{Sha256, Sha512, Digest};
use crate::{Database, Config};
use std::fs;

pub fn identify_hash(hash: &str) -> Result<()> {
    println!("\n{}", "🔍 Hash Identifier".bright_cyan().bold());
    println!("Hash: {}", hash.bright_yellow());
    
    let hash_len = hash.len();
    let is_hex = hash.chars().all(|c| c.is_ascii_hexdigit());
    
    println!("\n{}", "Possible hash types:".bright_green().bold());
    
    match hash_len {
        32 if is_hex => {
            println!("  • {} (32 hex characters)", "MD5".bright_magenta());
            println!("  • {} (32 hex characters)", "NTLM".bright_magenta());
        }
        40 if is_hex => {
            println!("  • {} (40 hex characters)", "SHA-1".bright_magenta());
        }
        64 if is_hex => {
            println!("  • {} (64 hex characters)", "SHA-256".bright_magenta());
        }
        96 if is_hex => {
            println!("  • {} (96 hex characters)", "SHA-384".bright_magenta());
        }
        128 if is_hex => {
            println!("  • {} (128 hex characters)", "SHA-512".bright_magenta());
        }
        _ => {
            if hash.starts_with("$2a$") || hash.starts_with("$2b$") || hash.starts_with("$2y$") {
                println!("  • {}", "bcrypt".bright_magenta());
            } else if hash.starts_with("$6$") {
                println!("  • {}", "SHA-512 (Unix)".bright_magenta());
            } else if hash.starts_with("$5$") {
                println!("  • {}", "SHA-256 (Unix)".bright_magenta());
            } else if hash.starts_with("$1$") {
                println!("  • {}", "MD5 (Unix)".bright_magenta());
            } else {
                println!("  • {}", "Unknown or encoded format".yellow());
            }
        }
    }
    
    Ok(())
}

pub async fn rainbow_attack(hash: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", "⚡ Rainbow Table Attack".bright_cyan().bold());
    println!("Hash: {}", hash.bright_yellow());
    
    // Simulated rainbow table
    let rainbow_table = vec![
        ("password", "5f4dcc3b5aa765d61d8327deb882cf99"),
        ("123456", "e10adc3949ba59abbe56e057f20f883e"),
        ("admin", "21232f297a57a5a743894a0e4a801fc3"),
        ("letmein", "0d107d09f5bbe40cade3de5c71e9e9b7"),
        ("welcome", "40be4e59b9a2a2b5dffb918c0e86b3d7"),
        ("monkey", "d0763edaa9d9bd2a9516280e9044d885"),
        ("password123", "482c811da5d5b4bc6d497ffa98491e38"),
        ("qwerty", "d8578edf8458ce06fbc5bb76a58c5ca4"),
    ];
    
    println!("\n{}", "Searching rainbow table...".bright_yellow());
    
    let pb = ProgressBar::new(rainbow_table.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    
    let mut found = false;
    
    for (plain, table_hash) in rainbow_table {
        pb.inc(1);
        
        if table_hash == hash.to_lowercase() {
            println!("\n{}", "🎉 HASH CRACKED!".bright_green().bold());
            println!("  Plain text: {}", plain.bright_yellow());
            
            let _ = db.save_hash_result(hash, Some(plain), "MD5", "Rainbow Table");
            found = true;
            break;
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
    
    pb.finish();
    
    if !found {
        println!("\n{}", "❌ Hash not found in rainbow table".yellow());
    }
    
    Ok(())
}

pub async fn dictionary_attack(hash: &str, dict_file: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", "📖 Dictionary Attack".bright_cyan().bold());
    println!("Hash: {}", hash.bright_yellow());
    println!("Dictionary: {}", dict_file.bright_cyan());
    
    if !std::path::Path::new(dict_file).exists() {
        println!("{}", "❌ Dictionary file not found".bright_red());
        return Ok(());
    }
    
    let contents = fs::read_to_string(dict_file)?;
    let words: Vec<&str> = contents.lines().collect();
    
    println!("\n{}", format!("Loaded {} words", words.len()).bright_yellow());
    
    let pb = ProgressBar::new(words.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    
    let hash_lower = hash.to_lowercase();
    let mut found = false;
    
    for word in words {
        pb.set_message(format!("Trying: {}...", &word[..10.min(word.len())]));
        
        // Try MD5
        let md5_hash = format!("{:x}", md5::compute(word.as_bytes()));
        if md5_hash == hash_lower {
            println!("\n{}", "🎉 HASH CRACKED!".bright_green().bold());
            println!("  Plain text: {}", word.bright_yellow());
            println!("  Algorithm: MD5");
            
            let _ = db.save_hash_result(hash, Some(word), "MD5", "Dictionary");
            found = true;
            break;
        }
        
        // Try SHA-256
        let sha256_hash = format!("{:x}", Sha256::digest(word.as_bytes()));
        if sha256_hash == hash_lower {
            println!("\n{}", "🎉 HASH CRACKED!".bright_green().bold());
            println!("  Plain text: {}", word.bright_yellow());
            println!("  Algorithm: SHA-256");
            
            let _ = db.save_hash_result(hash, Some(word), "SHA-256", "Dictionary");
            found = true;
            break;
        }
        
        pb.inc(1);
        
        if pb.position() % 100 == 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }
    }
    
    pb.finish();
    
    if !found {
        println!("\n{}", "❌ Hash not found in dictionary".yellow());
    }
    
    Ok(())
}

pub async fn brute_force(hash: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", "🔢 Brute Force Attack".bright_cyan().bold());
    println!("Hash: {}", hash.bright_yellow());
    
    println!("\n{}", "⚠  Brute force can take a very long time!".yellow());
    println!("{}", "Starting with common patterns (4 characters)...".bright_cyan());
    
    let charset = "abcdefghijklmnopqrstuvwxyz0123456789";
    let max_length = 4;
    let hash_lower = hash.to_lowercase();
    
    let mut attempts = 0;
    let max_attempts = 10000;
    
    let pb = ProgressBar::new(max_attempts);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    
    // Simple brute force demonstration (not comprehensive)
    for c1 in charset.chars().take(10) {
        for c2 in charset.chars().take(10) {
            for c3 in charset.chars().take(10) {
                for c4 in charset.chars().take(10) {
                    let candidate = format!("{}{}{}{}", c1, c2, c3, c4);
                    
                    let md5_hash = format!("{:x}", md5::compute(candidate.as_bytes()));
                    if md5_hash == hash_lower {
                        println!("\n{}", "🎉 HASH CRACKED!".bright_green().bold());
                        println!("  Plain text: {}", candidate.bright_yellow());
                        
                        let _ = db.save_hash_result(hash, Some(&candidate), "MD5", "Brute Force");
                        pb.finish();
                        return Ok(());
                    }
                    
                    attempts += 1;
                    pb.inc(1);
                    
                    if attempts >= max_attempts {
                        break;
                    }
                }
                if attempts >= max_attempts {
                    break;
                }
            }
            if attempts >= max_attempts {
                break;
            }
        }
        if attempts >= max_attempts {
            break;
        }
    }
    
    pb.finish();
    
    println!("\n{}", format!("Tried {} combinations", attempts).bright_yellow());
    println!("{}", "❌ Hash not cracked (limited demonstration)".yellow());
    println!("{}", "For real brute forcing, use specialized tools like hashcat".bright_black());
    
    Ok(())
}

pub fn encrypt_decrypt(data: &str) -> Result<()> {
    println!("\n{}", "🔐 Encrypt/Decrypt Data".bright_cyan().bold());
    
    let encoded_base64 = base64::encode(data);
    let decoded_base64 = base64::decode(data).ok();
    
    println!("\n{}", "Base64 Encoding:".bright_yellow().bold());
    println!("  Original: {}", data.bright_white());
    println!("  Encoded: {}", encoded_base64.bright_green());
    
    if let Some(decoded) = decoded_base64 {
        if let Ok(decoded_str) = String::from_utf8(decoded) {
            println!("\n{}", "Base64 Decoding:".bright_yellow().bold());
            println!("  Decoded: {}", decoded_str.bright_cyan());
        }
    }
    
    println!("\n{}", "Hex Encoding:".bright_yellow().bold());
    let hex_encoded = hex::encode(data.as_bytes());
    println!("  Encoded: {}", hex_encoded.bright_green());
    
    if let Ok(hex_decoded) = hex::decode(data) {
        if let Ok(decoded_str) = String::from_utf8(hex_decoded) {
            println!("\n{}", "Hex Decoding:".bright_yellow().bold());
            println!("  Decoded: {}", decoded_str.bright_cyan());
        }
    }
    
    // ROT13
    let rot13: String = data
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                ((((c as u8 - base + 13) % 26) + base) as char)
            } else {
                c
            }
        })
        .collect();
    
    println!("\n{}", "ROT13:".bright_yellow().bold());
    println!("  Encoded: {}", rot13.bright_magenta());
    
    Ok(())
}

pub fn generate_hashes(data: &str) -> Result<()> {
    println!("\n{}", "🔑 Generate Hashes".bright_cyan().bold());
    println!("Input: {}", data.bright_yellow());
    
    // MD5
    let md5_hash = format!("{:x}", md5::compute(data.as_bytes()));
    println!("\n{}", "MD5:".bright_green().bold());
    println!("  {}", md5_hash.bright_white());
    
    // SHA-256
    let sha256_hash = format!("{:x}", Sha256::digest(data.as_bytes()));
    println!("\n{}", "SHA-256:".bright_green().bold());
    println!("  {}", sha256_hash.bright_white());
    
    // SHA-512
    let sha512_hash = format!("{:x}", Sha512::digest(data.as_bytes()));
    println!("\n{}", "SHA-512:".bright_green().bold());
    println!("  {}", sha512_hash.bright_white());
    
    // Save to file
    let output = format!(
        "Input: {}\n\nMD5:\n{}\n\nSHA-256:\n{}\n\nSHA-512:\n{}\n",
        data, md5_hash, sha256_hash, sha512_hash
    );
    
    fs::write("hashes.txt", output)?;
    println!("\n{}", "💾 Hashes saved to hashes.txt".green().bold());
    
    Ok(())
}
