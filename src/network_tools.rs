use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use anyhow::Result;
use crate::{Database, Config};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use rayon::prelude::*;

const COMMON_PORTS: &[u16] = &[
    21, 22, 23, 25, 53, 80, 110, 111, 135, 139, 143, 443, 445, 993, 995,
    1723, 3306, 3389, 5900, 8080, 8443, 8888, 27017,
];

const PORT_SERVICES: &[(&str, u16)] = &[
    ("FTP", 21),
    ("SSH", 22),
    ("Telnet", 23),
    ("SMTP", 25),
    ("DNS", 53),
    ("HTTP", 80),
    ("POP3", 110),
    ("RPC", 111),
    ("NetBIOS", 135),
    ("NetBIOS", 139),
    ("IMAP", 143),
    ("HTTPS", 443),
    ("SMB", 445),
    ("IMAPS", 993),
    ("POP3S", 995),
    ("PPTP", 1723),
    ("MySQL", 3306),
    ("RDP", 3389),
    ("VNC", 5900),
    ("HTTP-Alt", 8080),
    ("HTTPS-Alt", 8443),
    ("HTTP-Proxy", 8888),
    ("MongoDB", 27017),
];

pub async fn port_scan(target: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("🔍 Advanced Port Scanner: {}", target).bright_cyan().bold());
    
    // Resolve hostname
    let address = format!("{}:80", target).to_socket_addrs()?.next();
    
    if address.is_none() {
        println!("{}", "❌ Could not resolve target".bright_red());
        return Ok(());
    }
    
    let ip = address.unwrap().ip();
    println!("Resolved to: {}", ip.to_string().bright_yellow());
    
    println!("\n{}", "Scanning common ports...".bright_cyan());
    
    let pb = ProgressBar::new(COMMON_PORTS.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    
    let open_ports: Vec<_> = COMMON_PORTS
        .par_iter()
        .filter_map(|&port| {
            pb.inc(1);
            
            let addr = format!("{}:{}", ip, port);
            match TcpStream::connect_timeout(
                &addr.parse().unwrap(),
                Duration::from_millis(config.timeout * 10)
            ) {
                Ok(_) => {
                    let service = PORT_SERVICES
                        .iter()
                        .find(|(_, p)| *p == port)
                        .map(|(s, _)| *s)
                        .unwrap_or("Unknown");
                    
                    Some((port, service))
                }
                Err(_) => None,
            }
        })
        .collect();
    
    pb.finish_with_message("Scan complete!");
    
    println!("\n{}", "📊 Scan Results:".bright_yellow().bold());
    
    if open_ports.is_empty() {
        println!("{}", "  No open ports found".yellow());
    } else {
        for (port, service) in &open_ports {
            println!(
                "  {} {} - {}",
                "✓".bright_green(),
                format!("Port {}", port).bright_white(),
                service.bright_cyan()
            );
            
            let _ = db.save_port_scan(&target, *port, "open", Some(service));
        }
        
        println!("\n{}", format!("✅ Found {} open ports", open_ports.len()).green().bold());
    }
    
    Ok(())
}

pub async fn service_detect(target: &str, _db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("🌐 Service Detection: {}", target).bright_cyan().bold());
    
    // First scan for open ports
    let address = format!("{}:80", target).to_socket_addrs()?.next();
    
    if address.is_none() {
        println!("{}", "❌ Could not resolve target".bright_red());
        return Ok(());
    }
    
    let ip = address.unwrap().ip();
    
    let open_ports: Vec<_> = COMMON_PORTS
        .par_iter()
        .filter_map(|&port| {
            let addr = format!("{}:{}", ip, port);
            match TcpStream::connect_timeout(
                &addr.parse().unwrap(),
                Duration::from_millis(config.timeout * 10)
            ) {
                Ok(_) => Some(port),
                Err(_) => None,
            }
        })
        .collect();
    
    println!("\n{}", format!("Found {} open ports, detecting services...", open_ports.len()).bright_yellow());
    
    for port in open_ports {
        let service = PORT_SERVICES
            .iter()
            .find(|(_, p)| *p == port)
            .map(|(s, _)| *s)
            .unwrap_or("Unknown");
        
        println!("\n{}", format!("Port {} ({})", port, service).bright_cyan().bold());
        
        // Try HTTP
        if port == 80 || port == 443 || port == 8080 || port == 8443 {
            let scheme = if port == 443 || port == 8443 { "https" } else { "http" };
            let url = format!("{}://{}:{}", scheme, target, port);
            
            match reqwest::get(&url).await {
                Ok(response) => {
                    println!("  {} HTTP Server detected", "✓".bright_green());
                    if let Some(server) = response.headers().get("server") {
                        println!("  Server: {}", server.to_str().unwrap_or("Unknown").bright_yellow());
                    }
                }
                Err(_) => {
                    println!("  {} Could not connect via HTTP", "✗".bright_black());
                }
            }
        } else {
            println!("  Service: {}", service.bright_yellow());
            println!("  Deep inspection not implemented for this service");
        }
    }
    
    println!("\n{}", "✅ Service detection completed!".green().bold());
    
    Ok(())
}

pub async fn vuln_check(target: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("🔓 Vulnerability Check: {}", target).bright_cyan().bold());
    
    println!("\n{}", "Checking common vulnerabilities...".bright_yellow());
    
    let checks = vec![
        ("Anonymous FTP", 21),
        ("SSH Weak Ciphers", 22),
        ("Open Telnet", 23),
        ("Open SMTP Relay", 25),
        ("HTTP Security Headers", 80),
        ("HTTPS Configuration", 443),
        ("SMB Signing", 445),
        ("Open MongoDB", 27017),
    ];
    
    let pb = ProgressBar::new(checks.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    
    for (check_name, port) in checks {
        pb.set_message(check_name.to_string());
        
        let address = format!("{}:80", target).to_socket_addrs()?.next();
        if let Some(addr) = address {
            let ip = addr.ip();
            let target_addr = format!("{}:{}", ip, port);
            
            match TcpStream::connect_timeout(
                &target_addr.parse().unwrap(),
                Duration::from_millis(config.timeout * 10)
            ) {
                Ok(_) => {
                    println!("\n  {} {} - Port {} is open", "⚠".bright_yellow(), check_name, port);
                }
                Err(_) => {
                    if config.verbose {
                        println!("\n  {} {} - Port {} is closed", "✓".bright_green(), check_name, port);
                    }
                }
            }
        }
        
        pb.inc(1);
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    pb.finish_with_message("Vulnerability check complete!");
    
    println!("\n{}", "✅ Vulnerability check completed!".green().bold());
    println!("{}", "Note: This is a basic check. Use specialized tools for thorough assessment.".bright_black());
    
    Ok(())
}

pub async fn range_scan(range: &str, db: &Database, config: &Config) -> Result<()> {
    println!("\n{}", format!("📡 Network Range Scanner: {}", range).bright_cyan().bold());
    
    // Parse CIDR notation (simple implementation)
    if !range.contains('/') {
        println!("{}", "❌ Invalid range format. Use CIDR notation (e.g., 192.168.1.0/24)".bright_red());
        return Ok(());
    }
    
    let parts: Vec<&str> = range.split('/').collect();
    let base_ip = parts[0];
    let mask = parts[1].parse::<u8>().unwrap_or(24);
    
    if mask != 24 {
        println!("{}", "⚠  Only /24 ranges supported in this version".yellow());
        return Ok(());
    }
    
    // Extract base network
    let ip_parts: Vec<&str> = base_ip.split('.').collect();
    if ip_parts.len() != 4 {
        println!("{}", "❌ Invalid IP format".bright_red());
        return Ok(());
    }
    
    let base = format!("{}.{}.{}.", ip_parts[0], ip_parts[1], ip_parts[2]);
    
    println!("\n{}", "Scanning range...".bright_yellow());
    
    let pb = ProgressBar::new(254);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    
    let alive_hosts: Vec<_> = (1..=254)
        .into_par_iter()
        .filter_map(|i| {
            pb.inc(1);
            
            let ip = format!("{}{}", base, i);
            let addr = format!("{}:80", ip);
            
            // Try common ports to check if host is alive
            for port in &[80, 443, 22, 445] {
                let target = format!("{}:{}", ip, port);
                if TcpStream::connect_timeout(
                    &target.parse().unwrap(),
                    Duration::from_millis(100)
                ).is_ok() {
                    return Some(ip);
                }
            }
            None
        })
        .collect();
    
    pb.finish_with_message("Range scan complete!");
    
    println!("\n{}", "📊 Alive Hosts:".bright_yellow().bold());
    
    if alive_hosts.is_empty() {
        println!("{}", "  No active hosts found".yellow());
    } else {
        for (i, host) in alive_hosts.iter().enumerate() {
            println!("  {}. {}", i + 1, host.bright_green());
        }
        
        println!("\n{}", format!("✅ Found {} active hosts", alive_hosts.len()).green().bold());
    }
    
    Ok(())
}
