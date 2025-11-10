use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use anyhow::Result;
use crate::{Database, Config};
use std::fs;

const COMMON_SUBDOMAINS: &[&str] = &[
    "www", "mail", "ftp", "localhost", "webmail", "smtp", "pop", "ns1", "webdisk",
    "ns2", "cpanel", "whm", "autodiscover", "autoconfig", "m", "imap", "test",
    "ns", "blog", "pop3", "dev", "www2", "admin", "forum", "news", "vpn", "ns3",
    "mail2", "new", "mysql", "old", "lists", "support", "mobile", "mx", "static",
    "docs", "beta", "shop", "sql", "secure", "demo", "cp", "calendar", "wiki",
    "web", "media", "email", "images", "img", "www1", "intranet", "portal",
    "video", "sip", "dns2", "api", "cdn", "stats", "dns1", "ns4", "www3", "dns",
    "search", "staging", "server", "mx1", "chat", "wap", "my", "svn", "mail1",
    "sites", "proxy", "ads", "host", "crm", "cms", "backup", "mx2", "lyncdiscover",
    "info", "apps", "download", "remote", "db", "forums", "store", "relay",
    "files", "newsletter", "app", "live", "owa", "en", "start", "sms", "office",
    "exchange", "ipv4",
];

pub async fn enumerate_subdomains(domain: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("🔍 Subdomain Enumeration: {}", domain).bright_cyan().bold());
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout))
        .danger_accept_invalid_certs(true)
        .build()?;
    
    println!("\n{}", format!("Testing {} common subdomains...", COMMON_SUBDOMAINS.len()).bright_yellow());
    
    let pb = ProgressBar::new(COMMON_SUBDOMAINS.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    
    let mut found_subdomains = Vec::new();
    
    for subdomain in COMMON_SUBDOMAINS {
        let full_domain = format!("{}.{}", subdomain, domain);
        pb.set_message(format!("Testing: {}...", &full_domain[..30.min(full_domain.len())]));
        
        // Try HTTP request
        let url = format!("http://{}", full_domain);
        
        match client.get(&url).timeout(std::time::Duration::from_secs(3)).send().await {
            Ok(_) => {
                println!("\n  {} {}", "✓".bright_green(), full_domain.bright_cyan());
                found_subdomains.push(full_domain.clone());
                
                let _ = db.save_subdomain(domain, &full_domain, None);
            }
            Err(_) => {
                // Also try HTTPS
                let https_url = format!("https://{}", full_domain);
                if client.get(&https_url).timeout(std::time::Duration::from_secs(3)).send().await.is_ok() {
                    println!("\n  {} {} (HTTPS)", "✓".bright_green(), full_domain.bright_cyan());
                    found_subdomains.push(full_domain.clone());
                    
                    let _ = db.save_subdomain(domain, &full_domain, None);
                }
            }
        }
        
        pb.inc(1);
        tokio::time::sleep(tokio::time::Duration::from_millis(config.rate_limit)).await;
    }
    
    pb.finish_with_message("Enumeration complete!");
    
    println!("\n{}", "📊 Results:".bright_yellow().bold());
    
    if found_subdomains.is_empty() {
        println!("{}", "  No subdomains found".yellow());
    } else {
        println!("{}", format!("  Found {} subdomains", found_subdomains.len()).bright_green());
        
        // Save to file
        fs::write(
            format!("{}_subdomains.txt", domain.replace(".", "_")),
            found_subdomains.join("\n")
        )?;
        
        println!("\n{}", format!("💾 Saved to {}_subdomains.txt", domain.replace(".", "_")).green().bold());
    }
    
    Ok(())
}

pub async fn lookup_records(domain: &str, config: &Config) -> Result<()> {
    println!("\n{}", format!("📡 DNS Record Lookup: {}", domain).bright_cyan().bold());
    
    println!("\n{}", "🔍 Querying DNS records...".bright_yellow());
    
    // Simulate DNS lookups (real implementation would use trust-dns-resolver)
    let record_types = vec!["A", "AAAA", "MX", "NS", "TXT", "CNAME", "SOA"];
    
    for record_type in record_types {
        println!("\n{}", format!("{} Records:", record_type).bright_cyan().bold());
        println!("  {}", "DNS lookup not fully implemented - use dig/nslookup".yellow());
    }
    
    // Try to resolve A record using reqwest
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout))
        .build()?;
    
    match client.get(&format!("http://{}", domain)).send().await {
        Ok(response) => {
            if let Some(remote_addr) = response.remote_addr() {
                println!("\n{}", "A Record (via connection):".bright_green().bold());
                println!("  {}", remote_addr.ip().to_string().bright_yellow());
            }
        }
        Err(_) => {
            println!("\n{}", "  Could not resolve domain".yellow());
        }
    }
    
    println!("\n{}", "✅ DNS lookup completed!".green().bold());
    println!("{}", "Note: For full DNS enumeration, use tools like dig or dnsenum".bright_black());
    
    Ok(())
}

pub async fn zone_transfer(domain: &str, config: &Config) -> Result<()> {
    println!("\n{}", format!("🌐 DNS Zone Transfer Test: {}", domain).bright_cyan().bold());
    
    println!("\n{}", "⚠  Testing for zone transfer vulnerability...".bright_yellow());
    println!("{}", "This feature requires DNS query capabilities".yellow());
    
    // In a real implementation, this would attempt AXFR queries
    println!("\n{}", "Zone transfer testing not fully implemented".yellow());
    println!("{}", "Use tools like dig for actual zone transfer attempts:".bright_black());
    println!("{}", format!("  dig axfr @ns1.{} {}", domain, domain).bright_black());
    
    println!("\n{}", "✅ Test completed!".green().bold());
    
    Ok(())
}

pub async fn brute_force(domain: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("🎯 DNS Brute Force: {}", domain).bright_cyan().bold());
    
    // Extended wordlist for brute forcing
    let mut wordlist = COMMON_SUBDOMAINS.to_vec();
    
    // Add some variations
    let extras = vec![
        "staging", "prod", "production", "uat", "qa", "test1", "test2",
        "dev1", "dev2", "web1", "web2", "app1", "app2", "api1", "api2",
        "old", "new", "backup", "temp", "tmp", "archive",
    ];
    
    wordlist.extend(extras);
    
    println!("\n{}", format!("Brute forcing with {} entries...", wordlist.len()).bright_yellow());
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout))
        .danger_accept_invalid_certs(true)
        .build()?;
    
    let pb = ProgressBar::new(wordlist.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    
    let mut found = Vec::new();
    
    for word in wordlist {
        let full_domain = format!("{}.{}", word, domain);
        pb.set_message(format!("Testing: {}...", &full_domain[..30.min(full_domain.len())]));
        
        // Try to connect
        let url = format!("http://{}", full_domain);
        
        if client.get(&url).timeout(std::time::Duration::from_secs(2)).send().await.is_ok() {
            println!("\n  {} {}", "✓".bright_green(), full_domain.bright_cyan());
            found.push(full_domain.clone());
            
            let _ = db.save_subdomain(domain, &full_domain, None);
        }
        
        pb.inc(1);
        tokio::time::sleep(tokio::time::Duration::from_millis(config.rate_limit)).await;
    }
    
    pb.finish_with_message("Brute force complete!");
    
    println!("\n{}", "📊 Results:".bright_yellow().bold());
    
    if found.is_empty() {
        println!("{}", "  No subdomains found".yellow());
    } else {
        println!("{}", format!("  Found {} subdomains", found.len()).bright_green());
        
        for subdomain in &found {
            println!("    • {}", subdomain.bright_cyan());
        }
        
        // Save results
        fs::write(
            format!("{}_bruteforce.txt", domain.replace(".", "_")),
            found.join("\n")
        )?;
        
        println!("\n{}", format!("💾 Saved to {}_bruteforce.txt", domain.replace(".", "_")).green().bold());
    }
    
    Ok(())
}
