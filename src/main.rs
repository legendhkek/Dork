use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select, Input};
use std::io::{self, Write};
use anyhow::Result;

mod database;
mod dork_checker;
mod sql_scanner;
mod web_tools;
mod network_tools;
mod crypto_tools;
mod dns_tools;
mod config;

use database::Database;
use config::Config;

const VERSION: &str = "2.0.0";
const BANNER: &str = r#"
╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║         ███████╗██╗    ██╗██╗███████╗███████╗                ║
║         ██╔════╝██║    ██║██║██╔════╝██╔════╝                ║
║         ███████╗██║ █╗ ██║██║███████╗███████╗                ║
║         ╚════██║██║███╗██║██║╚════██║╚════██║                ║
║         ███████║╚███╔███╔╝██║███████║███████║                ║
║         ╚══════╝ ╚══╝╚══╝ ╚═╝╚══════╝╚══════╝                ║
║                                                               ║
║            🛠️  ADVANCED SECURITY TOOLKIT 🛠️                   ║
║                    Version 2.0.0                             ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
"#;

#[tokio::main]
async fn main() -> Result<()> {
    clear_screen();
    
    // Initialize database and config
    let db = Database::new("swiss_army.db").await?;
    let mut config = Config::load().unwrap_or_default();
    
    loop {
        display_banner();
        
        let options = vec![
            "🎯 Advanced Dork Checker & Google Hacking",
            "🔓 SQL Injection Vulnerability Scanner",
            "🕸️  Web Scraper & Crawler Suite",
            "🌐 Network Scanner & Port Analyzer",
            "🔐 Hash Cracker & Encryption Tools",
            "🌍 Subdomain Finder & DNS Enumeration",
            "📊 Keyword & Payload Generator",
            "⚙️  Configuration & Settings",
            "📈 View Statistics & Reports",
            "ℹ️  About & Help",
            "❌ Exit",
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a tool")
            .items(&options)
            .default(0)
            .interact()?;
        
        match selection {
            0 => dork_menu(&db, &config).await?,
            1 => sql_scanner_menu(&db, &config).await?,
            2 => web_tools_menu(&db, &config).await?,
            3 => network_tools_menu(&db, &config).await?,
            4 => crypto_menu(&db, &config).await?,
            5 => dns_menu(&db, &config).await?,
            6 => generator_menu(&db, &config).await?,
            7 => config_menu(&mut config).await?,
            8 => stats_menu(&db).await?,
            9 => show_help(),
            10 => {
                println!("\n{}", "Thank you for using Swiss Army Suite!".green().bold());
                break;
            }
            _ => {}
        }
    }
    
    Ok(())
}

fn display_banner() {
    clear_screen();
    println!("{}", BANNER.bright_cyan().bold());
    println!("{}", format!("  Advanced Penetration Testing & Security Research Tool v{}", VERSION).bright_yellow());
    println!("{}\n", "  ⚠️  For Educational and Authorized Testing Only ⚠️".bright_red());
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn pause() {
    println!("\n{}", "Press Enter to continue...".bright_black());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

async fn dork_menu(db: &Database, config: &Config) -> Result<()> {
    clear_screen();
    println!("\n{}", "═══ ADVANCED DORK CHECKER ═══".bright_cyan().bold());
    
    let options = vec![
        "🔍 Single Target Dork Check",
        "📝 Bulk Dork Checking (From File)",
        "🎲 Generate Random Dorks",
        "💾 View Saved Results",
        "🔙 Back to Main Menu",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select option")
        .items(&options)
        .interact()?;
    
    match selection {
        0 => {
            let target: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target domain/keyword")
                .interact_text()?;
            
            dork_checker::check_single(&target, db, config).await?;
        }
        1 => {
            let filepath: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter file path")
                .interact_text()?;
            
            dork_checker::check_bulk(&filepath, db, config).await?;
        }
        2 => {
            dork_checker::generate_dorks(db).await?;
        }
        3 => {
            dork_checker::view_results(db).await?;
        }
        _ => {}
    }
    
    pause();
    Ok(())
}

async fn sql_scanner_menu(db: &Database, config: &Config) -> Result<()> {
    clear_screen();
    println!("\n{}", "═══ SQL VULNERABILITY SCANNER ═══".bright_cyan().bold());
    
    let options = vec![
        "🎯 Quick Scan (GET Parameters)",
        "🚀 Deep Scan (GET + POST + Headers)",
        "📋 OWASP Top 10 Full Scan",
        "🔬 Advanced Blind SQL Testing",
        "💉 Custom Payload Injection",
        "📊 View Scan History",
        "🔙 Back to Main Menu",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select scan type")
        .items(&options)
        .interact()?;
    
    match selection {
        0 => {
            let url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target URL")
                .interact_text()?;
            
            sql_scanner::quick_scan(&url, db, config).await?;
        }
        1 => {
            let url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target URL")
                .interact_text()?;
            
            sql_scanner::deep_scan(&url, db, config).await?;
        }
        2 => {
            let url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target URL")
                .interact_text()?;
            
            sql_scanner::owasp_scan(&url, db, config).await?;
        }
        3 => {
            let url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target URL")
                .interact_text()?;
            
            sql_scanner::blind_scan(&url, db, config).await?;
        }
        4 => {
            let url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target URL")
                .interact_text()?;
            let payload: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter custom payload")
                .interact_text()?;
            
            sql_scanner::custom_scan(&url, &payload, db, config).await?;
        }
        5 => {
            sql_scanner::view_history(db).await?;
        }
        _ => {}
    }
    
    pause();
    Ok(())
}

async fn web_tools_menu(db: &Database, config: &Config) -> Result<()> {
    clear_screen();
    println!("\n{}", "═══ WEB SCRAPER & CRAWLER ═══".bright_cyan().bold());
    
    let options = vec![
        "🕷️  Smart Web Crawler",
        "📥 Content Scraper",
        "🔗 Link Extractor",
        "📧 Email & Contact Harvester",
        "🖼️  Image Downloader",
        "🔙 Back to Main Menu",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select tool")
        .items(&options)
        .interact()?;
    
    match selection {
        0 => {
            let url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter starting URL")
                .interact_text()?;
            
            web_tools::crawl(&url, db, config).await?;
        }
        1 => {
            let url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target URL")
                .interact_text()?;
            
            web_tools::scrape(&url, db, config).await?;
        }
        2 => {
            let url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target URL")
                .interact_text()?;
            
            web_tools::extract_links(&url, db, config).await?;
        }
        3 => {
            let url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target URL")
                .interact_text()?;
            
            web_tools::harvest_contacts(&url, db, config).await?;
        }
        4 => {
            let url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target URL")
                .interact_text()?;
            
            web_tools::download_images(&url, db, config).await?;
        }
        _ => {}
    }
    
    pause();
    Ok(())
}

async fn network_tools_menu(db: &Database, config: &Config) -> Result<()> {
    clear_screen();
    println!("\n{}", "═══ NETWORK SCANNER & TOOLS ═══".bright_cyan().bold());
    
    let options = vec![
        "🔍 Advanced Port Scanner",
        "🌐 Service Detection",
        "🔓 Vulnerability Check",
        "📡 Network Range Scanner",
        "🔙 Back to Main Menu",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select tool")
        .items(&options)
        .interact()?;
    
    match selection {
        0 => {
            let target: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target IP/hostname")
                .interact_text()?;
            
            network_tools::port_scan(&target, db, config).await?;
        }
        1 => {
            let target: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target IP/hostname")
                .interact_text()?;
            
            network_tools::service_detect(&target, db, config).await?;
        }
        2 => {
            let target: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target IP/hostname")
                .interact_text()?;
            
            network_tools::vuln_check(&target, db, config).await?;
        }
        3 => {
            let range: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter IP range (e.g., 192.168.1.0/24)")
                .interact_text()?;
            
            network_tools::range_scan(&range, db, config).await?;
        }
        _ => {}
    }
    
    pause();
    Ok(())
}

async fn crypto_menu(db: &Database, config: &Config) -> Result<()> {
    clear_screen();
    println!("\n{}", "═══ HASH CRACKER & CRYPTO TOOLS ═══".bright_cyan().bold());
    
    let options = vec![
        "🔓 Hash Identifier",
        "⚡ Rainbow Table Attack",
        "📖 Dictionary Attack",
        "🔢 Brute Force Attack",
        "🔐 Encrypt/Decrypt Data",
        "🔑 Generate Hashes",
        "🔙 Back to Main Menu",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select tool")
        .items(&options)
        .interact()?;
    
    match selection {
        0 => {
            let hash: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter hash")
                .interact_text()?;
            
            crypto_tools::identify_hash(&hash)?;
        }
        1 => {
            let hash: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter hash")
                .interact_text()?;
            
            crypto_tools::rainbow_attack(&hash, db, config).await?;
        }
        2 => {
            let hash: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter hash")
                .interact_text()?;
            let dict: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter dictionary file path")
                .interact_text()?;
            
            crypto_tools::dictionary_attack(&hash, &dict, db, config).await?;
        }
        3 => {
            let hash: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter hash")
                .interact_text()?;
            
            crypto_tools::brute_force(&hash, db, config).await?;
        }
        4 => {
            let data: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter data")
                .interact_text()?;
            
            crypto_tools::encrypt_decrypt(&data)?;
        }
        5 => {
            let data: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter text to hash")
                .interact_text()?;
            
            crypto_tools::generate_hashes(&data)?;
        }
        _ => {}
    }
    
    pause();
    Ok(())
}

async fn dns_menu(db: &Database, config: &Config) -> Result<()> {
    clear_screen();
    println!("\n{}", "═══ SUBDOMAIN FINDER & DNS TOOLS ═══".bright_cyan().bold());
    
    let options = vec![
        "🔍 Subdomain Enumeration",
        "📡 DNS Record Lookup",
        "🌐 Zone Transfer Test",
        "🎯 DNS Brute Force",
        "🔙 Back to Main Menu",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select tool")
        .items(&options)
        .interact()?;
    
    match selection {
        0 => {
            let domain: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter domain")
                .interact_text()?;
            
            dns_tools::enumerate_subdomains(&domain, db, config).await?;
        }
        1 => {
            let domain: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter domain")
                .interact_text()?;
            
            dns_tools::lookup_records(&domain, config).await?;
        }
        2 => {
            let domain: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter domain")
                .interact_text()?;
            
            dns_tools::zone_transfer(&domain, config).await?;
        }
        3 => {
            let domain: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter domain")
                .interact_text()?;
            
            dns_tools::brute_force(&domain, db, config).await?;
        }
        _ => {}
    }
    
    pause();
    Ok(())
}

async fn generator_menu(db: &Database, config: &Config) -> Result<()> {
    clear_screen();
    println!("\n{}", "═══ PAYLOAD & KEYWORD GENERATOR ═══".bright_cyan().bold());
    
    let options = vec![
        "🎲 SQL Injection Payloads",
        "🔓 XSS Payloads",
        "📝 Keyword Combinations",
        "🎯 Custom Wordlist Generator",
        "🔙 Back to Main Menu",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select generator")
        .items(&options)
        .interact()?;
    
    match selection {
        0 => {
            dork_checker::generate_sql_payloads(db).await?;
        }
        1 => {
            dork_checker::generate_xss_payloads(db).await?;
        }
        2 => {
            let keyword: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter base keyword")
                .interact_text()?;
            
            dork_checker::generate_keywords(&keyword, db).await?;
        }
        3 => {
            dork_checker::generate_wordlist(db, config).await?;
        }
        _ => {}
    }
    
    pause();
    Ok(())
}

async fn config_menu(config: &mut Config) -> Result<()> {
    clear_screen();
    println!("\n{}", "═══ CONFIGURATION & SETTINGS ═══".bright_cyan().bold());
    
    let options = vec![
        format!("🔢 Threads: {}", config.threads),
        format!("⏱️  Timeout: {}s", config.timeout),
        format!("🌐 User Agent: {}", &config.user_agent[..50.min(config.user_agent.len())]),
        format!("🔄 Retry Attempts: {}", config.retry_attempts),
        format!("📊 Verbose Output: {}", config.verbose),
        "💾 Save Configuration".to_string(),
        "🔙 Back to Main Menu".to_string(),
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select setting")
        .items(&options)
        .interact()?;
    
    match selection {
        0 => {
            let threads: usize = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter number of threads (1-100)")
                .interact_text()?;
            config.threads = threads.clamp(1, 100);
        }
        1 => {
            let timeout: u64 = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter timeout in seconds")
                .interact_text()?;
            config.timeout = timeout;
        }
        2 => {
            let ua: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter User-Agent string")
                .interact_text()?;
            config.user_agent = ua;
        }
        3 => {
            let retries: usize = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter retry attempts")
                .interact_text()?;
            config.retry_attempts = retries;
        }
        4 => {
            config.verbose = !config.verbose;
            println!("{}", format!("Verbose mode: {}", config.verbose).green());
        }
        5 => {
            config.save()?;
            println!("{}", "Configuration saved!".green().bold());
            pause();
        }
        _ => {}
    }
    
    Ok(())
}

async fn stats_menu(db: &Database) -> Result<()> {
    clear_screen();
    println!("\n{}", "═══ STATISTICS & REPORTS ═══".bright_cyan().bold());
    
    let stats = db.get_statistics().await?;
    
    println!("\n{}", "📊 Overall Statistics:".bright_yellow().bold());
    println!("  • Total Scans: {}", stats.total_scans.to_string().bright_green());
    println!("  • Vulnerabilities Found: {}", stats.vulnerabilities.to_string().bright_red());
    println!("  • Dorks Checked: {}", stats.dorks_checked.to_string().bright_cyan());
    println!("  • Subdomains Found: {}", stats.subdomains.to_string().bright_blue());
    println!("  • Database Size: {}", stats.db_size.to_string().bright_magenta());
    
    pause();
    Ok(())
}

fn show_help() {
    clear_screen();
    println!("\n{}", "═══ ABOUT & HELP ═══".bright_cyan().bold());
    println!("\n{}", "Swiss Army Suite v2.0.0".bright_yellow().bold());
    println!("\n{}", "An advanced penetration testing and security research toolkit.".bright_white());
    println!("\n{}", "Features:".bright_green().bold());
    println!("  • Advanced Google Dork searching with multiple engines");
    println!("  • SQL injection vulnerability scanner with OWASP Top 10");
    println!("  • Web scraping and intelligent crawling");
    println!("  • Network port scanning and service detection");
    println!("  • Hash cracking with multiple attack methods");
    println!("  • DNS enumeration and subdomain discovery");
    println!("  • Payload and wordlist generation");
    println!("\n{}", "⚠️  LEGAL DISCLAIMER:".bright_red().bold());
    println!("  This tool is for EDUCATIONAL PURPOSES and AUTHORIZED");
    println!("  SECURITY TESTING ONLY. Unauthorized access to computer");
    println!("  systems is illegal. Users are responsible for compliance");
    println!("  with all applicable laws and regulations.");
    println!("\n{}", "Author: Advanced Security Tools Team".bright_black());
    println!("{}", "License: MIT".bright_black());
    
    pause();
}
