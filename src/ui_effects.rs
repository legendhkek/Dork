// UI Effects and Animations
// Made by @LEGEND_BL

use colored::Colorize;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn loading_animation(message: &str, duration_ms: u64) {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let iterations = duration_ms / 100;
    
    for i in 0..iterations {
        let frame = frames[i as usize % frames.len()];
        print!("\r{} {} ", frame.bright_cyan().bold(), message.bright_white());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("\r✓ {} {}", message.bright_green(), "Done!".bright_green().bold());
}

pub fn progress_dots(message: &str, count: usize) {
    print!("{}", message.bright_yellow());
    io::stdout().flush().unwrap();
    
    for _ in 0..count {
        thread::sleep(Duration::from_millis(200));
        print!("{}", ".".bright_yellow());
        io::stdout().flush().unwrap();
    }
    println!(" {}", "✓".bright_green().bold());
}

pub fn typewriter_effect(text: &str, delay_ms: u64) {
    for ch in text.chars() {
        print!("{}", ch);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(delay_ms));
    }
    println!();
}

pub fn print_success(message: &str) {
    println!("{} {}", "✓".bright_green().bold(), message.bright_green());
}

pub fn print_error(message: &str) {
    println!("{} {}", "✗".bright_red().bold(), message.bright_red());
}

pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".bright_yellow().bold(), message.bright_yellow());
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".bright_cyan().bold(), message.bright_cyan());
}

pub fn print_section_header(title: &str) {
    println!();
    println!("{}", "═".repeat(63).bright_cyan().bold());
    println!("{}", format!("  {} {}", "▶".bright_yellow(), title.bright_cyan().bold()));
    println!("{}", "═".repeat(63).bright_cyan().bold());
    println!();
}

pub fn print_box(title: &str, content: Vec<&str>) {
    let width = 61;
    println!("{}", "┌".to_string() + &"─".repeat(width) + "┐");
    println!("│ {} │", title.bright_yellow().bold());
    println!("{}", "├".to_string() + &"─".repeat(width) + "┤");
    
    for line in content {
        println!("│ {:<59} │", line);
    }
    
    println!("{}", "└".to_string() + &"─".repeat(width) + "┘");
}

pub fn animated_countdown(seconds: u8) {
    for i in (1..=seconds).rev() {
        print!("\r⏱️  Starting in {} seconds... ", i);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!("\r✓ Starting now!                ");
}

pub fn show_startup_animation() {
    clear_screen();
    
    // Legend Dorker logo animation
    let logo = vec![
        "🔥 L E G E N D   D O R K E R 🔥",
        "═══════════════════════════════",
    ];
    
    for line in logo {
        typewriter_effect(line, 30);
        thread::sleep(Duration::from_millis(100));
    }
    
    println!();
    loading_animation("Initializing system", 800);
    loading_animation("Loading modules", 600);
    loading_animation("Checking dependencies", 500);
    loading_animation("Preparing tools", 400);
    
    println!();
    print_success("All systems ready!");
    thread::sleep(Duration::from_millis(500));
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn print_gradient_line(text: &str) {
    // Simulated gradient effect
    println!("{}", text.bright_cyan().bold());
}

pub fn show_scanning_animation(target: &str, scan_type: &str) {
    print_section_header(&format!("Scanning: {}", target));
    println!("{}", format!("  Scan Type: {}", scan_type).bright_yellow());
    println!("{}", format!("  Target: {}", target).bright_cyan());
    println!();
    
    loading_animation("Preparing scan", 500);
    loading_animation("Analyzing target", 600);
    loading_animation("Running checks", 700);
}

pub fn print_stats_box(title: &str, stats: Vec<(&str, String)>) {
    println!();
    println!("{}", "╔".to_string() + &"═".repeat(59) + "╗");
    println!("{}", format!("║ {} {:^50} ║", "📊".bright_yellow(), title.bright_cyan().bold()));
    println!("{}", "╠".to_string() + &"═".repeat(59) + "╣");
    
    for (label, value) in stats {
        println!("║ {:30} {:>27} ║", 
            label.bright_white(), 
            value.bright_green().bold()
        );
    }
    
    println!("{}", "╚".to_string() + &"═".repeat(59) + "╝");
    println!();
}

pub fn show_completion_message(item_count: usize, item_type: &str) {
    println!();
    println!("{}", "═".repeat(63).bright_green().bold());
    println!("{}", format!("  ✓ Scan Completed Successfully!").bright_green().bold());
    println!("{}", "═".repeat(63).bright_green().bold());
    println!();
    println!("{}", format!("  📊 Results: {} {}", item_count, item_type).bright_cyan());
    println!("{}", format!("  💾 Data saved to database").bright_cyan());
    println!("{}", format!("  📁 Report available for export").bright_cyan());
    println!();
}
