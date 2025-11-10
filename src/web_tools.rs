use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use anyhow::Result;
use scraper::{Html, Selector};
use crate::{Database, Config};
use std::collections::HashSet;
use regex::Regex;

pub async fn crawl(start_url: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("🕷️  Smart Web Crawler: {}", start_url).bright_cyan().bold());
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout))
        .user_agent(&config.user_agent)
        .danger_accept_invalid_certs(true)
        .build()?;
    
    let mut visited = HashSet::new();
    let mut to_visit = vec![start_url.to_string()];
    let max_depth = 3;
    let max_pages = 50;
    
    let pb = ProgressBar::new(max_pages as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    
    let base_url = url::Url::parse(start_url)?;
    
    while !to_visit.is_empty() && visited.len() < max_pages {
        let current_url = to_visit.pop().unwrap();
        
        if visited.contains(&current_url) {
            continue;
        }
        
        pb.set_message(format!("Crawling: {}...", &current_url[..40.min(current_url.len())]));
        
        match client.get(&current_url).send().await {
            Ok(response) => {
                if let Ok(body) = response.text().await {
                    let document = Html::parse_document(&body);
                    let link_selector = Selector::parse("a[href]").unwrap();
                    
                    for element in document.select(&link_selector) {
                        if let Some(href) = element.value().attr("href") {
                            if let Ok(absolute_url) = base_url.join(href) {
                                let url_str = absolute_url.to_string();
                                if absolute_url.domain() == base_url.domain() && !visited.contains(&url_str) {
                                    to_visit.push(url_str);
                                }
                            }
                        }
                    }
                    
                    println!("  ✓ {}", current_url.bright_green());
                }
            }
            Err(e) => {
                if config.verbose {
                    println!("  ✗ {} ({})", current_url.bright_black(), e);
                }
            }
        }
        
        visited.insert(current_url);
        pb.inc(1);
        
        tokio::time::sleep(tokio::time::Duration::from_millis(config.rate_limit)).await;
    }
    
    pb.finish_with_message("Crawling complete!");
    
    println!("\n{}", format!("✅ Crawled {} pages", visited.len()).green().bold());
    
    Ok(())
}

pub async fn scrape(url: &str, _db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("📥 Content Scraper: {}", url).bright_cyan().bold());
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout))
        .user_agent(&config.user_agent)
        .danger_accept_invalid_certs(true)
        .build()?;
    
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    
    let document = Html::parse_document(&body);
    
    // Extract title
    if let Ok(title_selector) = Selector::parse("title") {
        if let Some(title) = document.select(&title_selector).next() {
            println!("\n{}", "📌 Title:".bright_yellow().bold());
            println!("  {}", title.inner_html().bright_white());
        }
    }
    
    // Extract headings
    println!("\n{}", "📋 Headings:".bright_yellow().bold());
    for i in 1..=3 {
        if let Ok(selector) = Selector::parse(&format!("h{}", i)) {
            for (j, heading) in document.select(&selector).enumerate().take(5) {
                println!("  [h{}] {}", i, heading.inner_html().trim().bright_white());
                if j >= 4 {
                    println!("  ... {} more", document.select(&selector).count() - 5);
                    break;
                }
            }
        }
    }
    
    // Extract paragraphs
    println!("\n{}", "📝 Content Preview:".bright_yellow().bold());
    if let Ok(p_selector) = Selector::parse("p") {
        for (i, para) in document.select(&p_selector).enumerate().take(3) {
            let html = para.inner_html();
            let text = html.trim();
            if !text.is_empty() {
                println!("  {}", &text[..100.min(text.len())].bright_white());
            }
        }
    }
    
    println!("\n{}", "✅ Scraping completed!".green().bold());
    
    Ok(())
}

pub async fn extract_links(url: &str, _db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("🔗 Link Extractor: {}", url).bright_cyan().bold());
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout))
        .user_agent(&config.user_agent)
        .danger_accept_invalid_certs(true)
        .build()?;
    
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    
    let document = Html::parse_document(&body);
    let link_selector = Selector::parse("a[href]").unwrap();
    
    let mut internal_links = HashSet::new();
    let mut external_links = HashSet::new();
    
    let base_url = url::Url::parse(url)?;
    
    for element in document.select(&link_selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(absolute_url) = base_url.join(href) {
                if absolute_url.domain() == base_url.domain() {
                    internal_links.insert(absolute_url.to_string());
                } else {
                    external_links.insert(absolute_url.to_string());
                }
            }
        }
    }
    
    println!("\n{}", format!("🔗 Internal Links ({}):", internal_links.len()).bright_yellow().bold());
    for (i, link) in internal_links.iter().take(20).enumerate() {
        println!("  {}. {}", i + 1, link.bright_green());
    }
    if internal_links.len() > 20 {
        println!("  ... and {} more", internal_links.len() - 20);
    }
    
    println!("\n{}", format!("🌐 External Links ({}):", external_links.len()).bright_yellow().bold());
    for (i, link) in external_links.iter().take(10).enumerate() {
        println!("  {}. {}", i + 1, link.bright_cyan());
    }
    if external_links.len() > 10 {
        println!("  ... and {} more", external_links.len() - 10);
    }
    
    // Save to file
    let all_links: Vec<String> = internal_links.iter()
        .chain(external_links.iter())
        .cloned()
        .collect();
    
    std::fs::write("extracted_links.txt", all_links.join("\n"))?;
    println!("\n{}", "💾 Links saved to extracted_links.txt".green().bold());
    
    Ok(())
}

pub async fn harvest_contacts(url: &str, _db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("📧 Email & Contact Harvester: {}", url).bright_cyan().bold());
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout))
        .user_agent(&config.user_agent)
        .danger_accept_invalid_certs(true)
        .build()?;
    
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    
    // Email regex
    let email_regex = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b")?;
    let phone_regex = Regex::new(r"(\+\d{1,3})?[-.\s]?\(?\d{1,4}\)?[-.\s]?\d{1,4}[-.\s]?\d{1,9}")?;
    
    let mut emails = HashSet::new();
    let mut phones = HashSet::new();
    
    for cap in email_regex.captures_iter(&body) {
        emails.insert(cap[0].to_string());
    }
    
    for cap in phone_regex.captures_iter(&body) {
        let phone = cap[0].to_string();
        if phone.len() >= 10 {
            phones.insert(phone);
        }
    }
    
    println!("\n{}", format!("📧 Emails Found ({}):", emails.len()).bright_yellow().bold());
    for (i, email) in emails.iter().enumerate() {
        println!("  {}. {}", i + 1, email.bright_green());
    }
    
    println!("\n{}", format!("📱 Phone Numbers Found ({}):", phones.len()).bright_yellow().bold());
    for (i, phone) in phones.iter().enumerate() {
        println!("  {}. {}", i + 1, phone.bright_cyan());
    }
    
    // Save to file
    let mut output = String::new();
    output.push_str("=== EMAILS ===\n");
    for email in &emails {
        output.push_str(&format!("{}\n", email));
    }
    output.push_str("\n=== PHONE NUMBERS ===\n");
    for phone in &phones {
        output.push_str(&format!("{}\n", phone));
    }
    
    std::fs::write("contacts.txt", output)?;
    println!("\n{}", "💾 Contacts saved to contacts.txt".green().bold());
    
    Ok(())
}

pub async fn download_images(url: &str, _db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("🖼️  Image Downloader: {}", url).bright_cyan().bold());
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout))
        .user_agent(&config.user_agent)
        .danger_accept_invalid_certs(true)
        .build()?;
    
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    
    let document = Html::parse_document(&body);
    let img_selector = Selector::parse("img[src]").unwrap();
    
    let base_url = url::Url::parse(url)?;
    let mut images = Vec::new();
    
    for element in document.select(&img_selector) {
        if let Some(src) = element.value().attr("src") {
            if let Ok(absolute_url) = base_url.join(src) {
                images.push(absolute_url.to_string());
            }
        }
    }
    
    println!("\n{}", format!("🖼️  Found {} images", images.len()).bright_yellow().bold());
    
    for (i, img_url) in images.iter().take(20).enumerate() {
        println!("  {}. {}", i + 1, img_url.bright_green());
    }
    
    if images.len() > 20 {
        println!("  ... and {} more", images.len() - 20);
    }
    
    std::fs::write("image_urls.txt", images.join("\n"))?;
    println!("\n{}", "💾 Image URLs saved to image_urls.txt".green().bold());
    println!("{}", "Note: Actual downloading not implemented in this version".bright_black());
    
    Ok(())
}
