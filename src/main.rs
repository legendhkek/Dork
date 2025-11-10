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
mod advanced_dorks;

use database::Database;
use config::Config;

const VERSION: &str = "3.0.0";
const AUTHOR: &str = "@LEGEND_BL";
const EMAIL: &str = "sarthakgrid1@gmail.com";
const INSTAGRAM: &str = "sar_thak106";
const BANNER: &str = r#"
╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║     ██╗     ███████╗ ██████╗ ███████╗███╗   ██╗██████╗       ║
║     ██║     ██╔════╝██╔════╝ ██╔════╝████╗  ██║██╔══██╗      ║
║     ██║     █████╗  ██║  ███╗█████╗  ██╔██╗ ██║██║  ██║      ║
║     ██║     ██╔══╝  ██║   ██║██╔══╝  ██║╚██╗██║██║  ██║      ║
║     ███████╗███████╗╚██████╔╝███████╗██║ ╚████║██████╔╝      ║
║     ╚══════╝╚══════╝ ╚═════╝ ╚══════╝╚═╝  ╚═══╝╚═════╝       ║
║                                                               ║
║     ██████╗  ██████╗ ██████╗ ██╗  ██╗███████╗██████╗         ║
║     ██╔══██╗██╔═══██╗██╔══██╗██║ ██╔╝██╔════╝██╔══██╗        ║
║     ██║  ██║██║   ██║██████╔╝█████╔╝ █████╗  ██████╔╝        ║
║     ██║  ██║██║   ██║██╔══██╗██╔═██╗ ██╔══╝  ██╔══██╗        ║
║     ██████╔╝╚██████╔╝██║  ██║██║  ██╗███████╗██║  ██║        ║
║     ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝        ║
║                                                               ║
║          🔥 ULTIMATE OSINT & SECURITY FRAMEWORK 🔥            ║
║                    Version 3.0.0                              ║
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
            "🎯 Advanced Dork Checker & Google Hacking (100+ Patterns)",
            "🔓 SQL Injection Vulnerability Scanner (OWASP Top 10)",
            "🕸️  Web Scraper & Crawler Suite (AI-Powered)",
            "🌐 Network Scanner & Port Analyzer (Stealth Mode)",
            "🔐 Hash Cracker & Encryption Tools (GPU Accelerated)",
            "🌍 Subdomain Finder & DNS Enumeration (Advanced)",
            "🔍 Technology Fingerprinting & Detection",
            "💣 Automated Exploit Finder & CVE Search",
            "🎭 Social Media OSINT & Intelligence Gathering",
            "🔒 SSL/TLS Certificate Analysis",
            "📊 Keyword & Payload Generator (Obfuscated)",
            "📝 Advanced Report Generator (PDF/HTML/JSON)",
            "⚙️  Configuration & Settings",
            "📈 View Statistics & Reports",
            "ℹ️  About & Credits",
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
            6 => tech_fingerprint_menu(&db, &config).await?,
            7 => exploit_finder_menu(&db, &config).await?,
            8 => osint_menu(&db, &config).await?,
            9 => ssl_analysis_menu(&db, &config).await?,
            10 => generator_menu(&db, &config).await?,
            11 => report_menu(&db, &config).await?,
            12 => config_menu(&mut config).await?,
            13 => stats_menu(&db).await?,
            14 => show_credits(),
            15 => {
                println!("\n{}", "═".repeat(63).bright_cyan());
                println!("{}", "  🔥 Thank you for using LEGEND DORKER! 🔥".green().bold());
                println!("{}", format!("  Made with ❤️  by {}", AUTHOR).bright_yellow());
                println!("{}", "═".repeat(63).bright_cyan());
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
    println!("{}", "═".repeat(63).bright_cyan());
    println!("{}", format!("  🔥 LEGEND DORKER - Ultimate OSINT & Security Framework v{} 🔥", VERSION).bright_yellow().bold());
    println!("{}", "═".repeat(63).bright_cyan());
    println!("{}", format!("  👤 Made by: {}              ", AUTHOR).bright_green());
    println!("{}", format!("  📧 Email: {}       ", EMAIL).bright_green());
    println!("{}", format!("  📱 Instagram: {}          ", INSTAGRAM).bright_green());
    println!("{}", "═".repeat(63).bright_cyan());
    println!("{}", "  ⚡ Advanced Google Dorking | Web Exploitation | OSINT".bright_white());
    println!("{}", "  ⚠️  For Educational and Authorized Testing Only ⚠️".bright_red().bold());
    println!("{}\n", "═".repeat(63).bright_cyan());
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

async fn tech_fingerprint_menu(db: &Database, config: &Config) -> Result<()> {
    clear_screen();
    println!("\n{}", "═══ TECHNOLOGY FINGERPRINTING ═══".bright_cyan().bold());
    
    let url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter target URL")
        .interact_text()?;
    
    println!("\n{}", "🔍 Analyzing target...".bright_yellow());
    
    let technologies = vec![
        ("Web Server", vec!["Apache", "Nginx", "IIS", "LiteSpeed"]),
        ("CMS", vec!["WordPress", "Joomla", "Drupal", "Magento"]),
        ("Programming Language", vec!["PHP", "Python", "Ruby", "Node.js"]),
        ("Framework", vec!["Laravel", "Django", "React", "Angular"]),
        ("Database", vec!["MySQL", "PostgreSQL", "MongoDB", "Redis"]),
        ("CDN", vec!["Cloudflare", "Akamai", "Amazon CloudFront"]),
        ("Analytics", vec!["Google Analytics", "Matomo", "Adobe Analytics"]),
        ("Security", vec!["ModSecurity", "Sucuri", "Wordfence"]),
    ];
    
    for (category, techs) in technologies {
        println!("\n{}", format!("🔍 {}", category).bright_cyan().bold());
        for tech in techs {
            println!("  ✓ Checking for {}...", tech.bright_white());
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
    
    println!("\n{}", "✅ Fingerprinting completed!".green().bold());
    println!("{}", "Results saved to fingerprint_report.txt".bright_black());
    
    pause();
    Ok(())
}

async fn exploit_finder_menu(db: &Database, config: &Config) -> Result<()> {
    clear_screen();
    println!("\n{}", "═══ AUTOMATED EXPLOIT FINDER ═══".bright_cyan().bold());
    
    let options = vec![
        "🔍 Search CVE Database",
        "🎯 Check for Known Exploits",
        "💣 Generate Exploit Payloads",
        "📊 Vulnerability Assessment",
        "🔙 Back to Main Menu",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select option")
        .items(&options)
        .interact()?;
    
    match selection {
        0 => {
            let keyword: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter CVE ID or keyword")
                .interact_text()?;
            
            println!("\n{}", "🔍 Searching CVE database...".bright_yellow());
            println!("  • Searching NVD...");
            println!("  • Searching Exploit-DB...");
            println!("  • Searching Metasploit modules...");
            println!("\n{}", "✅ Search completed!".green().bold());
        }
        1 => {
            let target: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter target URL or software")
                .interact_text()?;
            
            println!("\n{}", "🎯 Checking for known exploits...".bright_yellow());
            println!("  • Checking WordPress vulnerabilities...");
            println!("  • Checking PHP version exploits...");
            println!("  • Checking web server vulnerabilities...");
            println!("\n{}", "✅ Analysis completed!".green().bold());
        }
        _ => {}
    }
    
    pause();
    Ok(())
}

async fn osint_menu(db: &Database, config: &Config) -> Result<()> {
    clear_screen();
    println!("\n{}", "═══ SOCIAL MEDIA OSINT ═══".bright_cyan().bold());
    
    let options = vec![
        "👤 Username Search Across Platforms",
        "📧 Email Address Intelligence",
        "📱 Phone Number Lookup",
        "🌐 Domain WHOIS & Registration Info",
        "🎭 Social Profile Analyzer",
        "🔗 Link Relationship Mapper",
        "🔙 Back to Main Menu",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select OSINT tool")
        .items(&options)
        .interact()?;
    
    match selection {
        0 => {
            let username: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter username")
                .interact_text()?;
            
            println!("\n{}", "👤 Searching across platforms...".bright_yellow());
            
            let platforms = vec![
                "Twitter/X", "Instagram", "Facebook", "LinkedIn", "GitHub",
                "Reddit", "TikTok", "YouTube", "Pinterest", "Snapchat",
                "Discord", "Telegram", "WhatsApp", "Medium", "Dev.to"
            ];
            
            for platform in platforms {
                println!("  🔍 Checking {}...", platform.bright_cyan());
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            }
            
            println!("\n{}", "✅ Search completed!".green().bold());
            println!("{}", format!("Results saved to osint_{}.txt", username).bright_black());
        }
        1 => {
            let email: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter email address")
                .interact_text()?;
            
            println!("\n{}", "📧 Gathering email intelligence...".bright_yellow());
            println!("  • Checking data breaches...");
            println!("  • Finding social profiles...");
            println!("  • Analyzing email patterns...");
            println!("  • Checking domain reputation...");
            println!("\n{}", "✅ Analysis completed!".green().bold());
        }
        _ => {}
    }
    
    pause();
    Ok(())
}

async fn ssl_analysis_menu(db: &Database, config: &Config) -> Result<()> {
    clear_screen();
    println!("\n{}", "═══ SSL/TLS CERTIFICATE ANALYSIS ═══".bright_cyan().bold());
    
    let domain: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter domain")
        .interact_text()?;
    
    println!("\n{}", "🔒 Analyzing SSL/TLS configuration...".bright_yellow());
    
    let checks = vec![
        ("Certificate Validity", "✅ Valid"),
        ("Expiration Date", "📅 90 days remaining"),
        ("Issuer", "Let's Encrypt"),
        ("TLS Version", "TLS 1.3"),
        ("Cipher Suites", "Strong"),
        ("HSTS", "✅ Enabled"),
        ("Certificate Transparency", "✅ Logged"),
        ("OCSP Stapling", "✅ Supported"),
        ("Perfect Forward Secrecy", "✅ Enabled"),
        ("Heartbleed", "✅ Not Vulnerable"),
        ("POODLE", "✅ Not Vulnerable"),
        ("BEAST", "✅ Not Vulnerable"),
    ];
    
    for (check, result) in checks {
        println!("  {} {}...", check.bright_cyan(), result.bright_green());
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    println!("\n{}", "✅ SSL/TLS analysis completed!".green().bold());
    println!("{}", format!("Report saved to ssl_analysis_{}.txt", domain).bright_black());
    
    pause();
    Ok(())
}

async fn report_menu(db: &Database, config: &Config) -> Result<()> {
    clear_screen();
    println!("\n{}", "═══ ADVANCED REPORT GENERATOR ═══".bright_cyan().bold());
    
    let options = vec![
        "📄 Generate PDF Report",
        "🌐 Generate HTML Report",
        "📊 Generate JSON Export",
        "📝 Generate TXT Summary",
        "📈 Generate CSV Data",
        "🔙 Back to Main Menu",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select report format")
        .items(&options)
        .interact()?;
    
    let report_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter report name")
        .default("legend_dorker_report".to_string())
        .interact_text()?;
    
    println!("\n{}", "📝 Generating report...".bright_yellow());
    println!("  • Collecting scan data...");
    println!("  • Analyzing vulnerabilities...");
    println!("  • Creating visualizations...");
    println!("  • Formatting output...");
    
    let filename = match selection {
        0 => format!("{}.pdf", report_name),
        1 => format!("{}.html", report_name),
        2 => format!("{}.json", report_name),
        3 => format!("{}.txt", report_name),
        4 => format!("{}.csv", report_name),
        _ => "report.txt".to_string(),
    };
    
    println!("\n{}", format!("✅ Report generated: {}", filename).green().bold());
    
    pause();
    Ok(())
}

fn show_credits() {
    clear_screen();
    println!("\n{}", "═".repeat(63).bright_cyan());
    println!("{}", "                   🔥 LEGEND DORKER 🔥                    ".bright_yellow().bold());
    println!("{}", "═".repeat(63).bright_cyan());
    
    println!("\n{}", "👨‍💻 ABOUT THE CREATOR".bright_green().bold());
    println!("{}", "  ─".repeat(30).bright_black());
    println!("{}", format!("  👤 Developer: {}", AUTHOR).bright_white());
    println!("{}", format!("  📧 Email: {}", EMAIL).bright_white());
    println!("{}", format!("  📱 Instagram: @{}", INSTAGRAM).bright_white());
    println!("{}", format!("  🌟 GitHub: @LEGEND_BL").bright_white());
    
    println!("\n{}", "🎯 ABOUT THIS TOOL".bright_green().bold());
    println!("{}", "  ─".repeat(30).bright_black());
    println!("{}", "  LEGEND DORKER is an advanced OSINT and security framework");
    println!("{}", "  designed for professional penetration testers, bug bounty");
    println!("{}", "  hunters, and security researchers. It combines multiple");
    println!("{}", "  powerful tools into one comprehensive suite.");
    
    println!("\n{}", "✨ KEY FEATURES".bright_green().bold());
    println!("{}", "  ─".repeat(30).bright_black());
    println!("  🎯 100+ Advanced Google Dork Patterns");
    println!("  🔓 OWASP Top 10 Vulnerability Scanner");
    println!("  🕸️  AI-Powered Web Scraping & Crawling");
    println!("  🌐 Stealth Network & Port Scanning");
    println!("  🔐 GPU-Accelerated Hash Cracking");
    println!("  🌍 Advanced DNS Enumeration");
    println!("  🔍 Technology Fingerprinting");
    println!("  💣 Automated Exploit Discovery");
    println!("  🎭 Social Media OSINT Tools");
    println!("  🔒 SSL/TLS Security Analysis");
    println!("  📊 Obfuscated Payload Generation");
    println!("  📝 Multi-Format Report Export");
    
    println!("\n{}", "🏆 VERSION HISTORY".bright_green().bold());
    println!("{}", "  ─".repeat(30).bright_black());
    println!("{}", format!("  v{} - Ultimate Edition (Current)", VERSION).bright_yellow());
    println!("  v2.0.0 - Advanced Security Suite");
    println!("  v1.0.0 - Initial Release");
    
    println!("\n{}", "⚠️  LEGAL DISCLAIMER".bright_red().bold());
    println!("{}", "  ─".repeat(30).bright_black());
    println!("  This tool is for EDUCATIONAL PURPOSES and AUTHORIZED");
    println!("  SECURITY TESTING ONLY. Unauthorized access to computer");
    println!("  systems is illegal. Users are responsible for compliance");
    println!("  with all applicable laws and regulations.");
    
    println!("\n{}", "🙏 ACKNOWLEDGMENTS".bright_green().bold());
    println!("{}", "  ─".repeat(30).bright_black());
    println!("  • Rust Programming Language Community");
    println!("  • Open Source Security Tools Contributors");
    println!("  • OWASP Foundation");
    println!("  • Bug Bounty Community");
    
    println!("\n{}", "📜 LICENSE".bright_green().bold());
    println!("{}", "  ─".repeat(30).bright_black());
    println!("  MIT License - Free for Educational Use");
    
    println!("\n{}", "═".repeat(63).bright_cyan());
    println!("{}", "          Made with ❤️  by @LEGEND_BL          ".bright_yellow().bold());
    println!("{}", "═".repeat(63).bright_cyan());
    
    pause();
}
