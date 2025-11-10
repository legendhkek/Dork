# 🔥 LEGEND DORKER v3.0 - Ultimate OSINT & Security Framework

<div align="center">

![Version](https://img.shields.io/badge/version-3.0.0-red.svg)
![Rust](https://img.shields.io/badge/rust-1.91+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows-blue.svg)

**The Most Advanced Google Dorking & OSINT Framework**

Made with ❤️ by **[@LEGEND_BL](https://instagram.com/sar_thak106)**

</div>

---

## 👨‍💻 Creator & Contact

<div align="center">

**Developer:** @LEGEND_BL  
**Email:** sarthakgrid1@gmail.com  
**Instagram:** [@sar_thak106](https://instagram.com/sar_thak106)  
**GitHub:** @LEGEND_BL

*For bug reports, feature requests, or collaboration*

</div>

---

## ⚠️ Legal Disclaimer

**CRITICAL:** This tool is designed for **EDUCATIONAL PURPOSES** and **AUTHORIZED SECURITY TESTING ONLY**.

- ✅ Use only on systems you own or have explicit written permission to test
- ✅ Respect all applicable local, state, and federal laws
- ✅ Follow responsible disclosure practices
- ❌ Unauthorized access to computer systems is illegal
- ❌ The author (@LEGEND_BL) is not responsible for misuse

**By using this software, you agree to use it responsibly and ethically.**

---

## 🔥 What is LEGEND DORKER?

LEGEND DORKER is the **ultimate OSINT and security framework** that combines **100+ Google dork patterns**, advanced vulnerability scanning, social media intelligence gathering, and comprehensive penetration testing tools into one powerful suite.

Unlike basic dorking tools, LEGEND DORKER provides:
- ✨ **14 categorized dork databases** with 100+ patterns
- 🚀 **Real-time vulnerability scanning**
- 🎯 **AI-powered web scraping**
- 🔒 **SSL/TLS security analysis**
- 🎭 **Cross-platform OSINT** (15+ social media platforms)
- 💣 **Automated exploit discovery**
- 📊 **Professional report generation**

---

## ✨ Key Features

### 1. 🎯 **Advanced Dork Checker (100+ Patterns)**
- **14 Specialized Categories:**
  - Admin Panel Discovery (20 dorks)
  - Database Files (15 dorks)
  - Backup Files (11 dorks)
  - Configuration Files (12 dorks)
  - Log Files (8 dorks)
  - Directory Listings (7 dorks)
  - Login Pages (9 dorks)
  - WordPress Vulnerabilities (7 dorks)
  - File Upload Pages (5 dorks)
  - SQL Error Messages (6 dorks)
  - Sensitive Information (9 dorks)
  - Git Repositories (4 dorks)
  - API Endpoints (7 dorks)
  - Web Shells (5 dorks)
- Single target & bulk checking
- Category-based selection
- Results saved to database

### 2. 🔓 **SQL Injection Scanner (OWASP Top 10)**
- Quick Scan (GET parameters)
- Deep Scan (GET, POST, Headers, Cookies)
- OWASP Top 10 Full Assessment
- Blind SQL Injection (Time-based & Boolean-based)
- Custom Payload Testing
- 18+ injection patterns
- 16+ error signature detection

### 3. 🕸️ **AI-Powered Web Scraper & Crawler**
- Smart crawling with depth control
- Content extraction (titles, headings, text)
- Link extraction (internal/external)
- Email & phone harvesting
- Image URL extraction
- Automatic result saving

### 4. 🌐 **Stealth Network Scanner**
- Multi-threaded port scanning
- 23 common ports scanned
- Service detection & fingerprinting
- Vulnerability checks
- Network range scanning (CIDR)
- Stealth mode operation

### 5. 🔐 **GPU-Accelerated Hash Cracker**
- Hash type identification (MD5, SHA-1, SHA-256, SHA-512, bcrypt)
- Rainbow table attacks
- Dictionary attacks
- Brute force capabilities
- Encoding/Decoding (Base64, Hex, ROT13)
- Multi-algorithm support

### 6. 🌍 **Advanced DNS Enumeration**
- 100+ common subdomains tested
- HTTP & HTTPS checking
- DNS record lookup
- Zone transfer testing
- DNS brute forcing
- Auto-export results

### 7. 🔍 **Technology Fingerprinting**
- Web server detection
- CMS identification
- Programming language detection
- Framework recognition
- Database identification
- CDN detection
- Analytics platform discovery
- Security tool detection

### 8. 💣 **Automated Exploit Finder**
- CVE database search
- Known exploit checker
- Exploit payload generator
- Vulnerability assessment
- Integration with Exploit-DB
- Metasploit module search

### 9. 🎭 **Social Media OSINT**
- **15+ Platform Username Search:**
  - Twitter/X, Instagram, Facebook
  - LinkedIn, GitHub, Reddit
  - TikTok, YouTube, Pinterest
  - Snapchat, Discord, Telegram
  - WhatsApp, Medium, Dev.to
- Email intelligence gathering
- Phone number lookup
- Domain WHOIS info
- Social profile analyzer
- Link relationship mapping

### 10. 🔒 **SSL/TLS Security Analysis**
- Certificate validity check
- Expiration monitoring
- Issuer verification
- TLS version detection
- Cipher suite analysis
- HSTS status
- Certificate transparency
- OCSP stapling
- Heartbleed/POODLE/BEAST testing

### 11. 📊 **Advanced Payload Generator**
- 20+ SQL injection payloads
- 15+ XSS payloads
- Keyword mutation (20+ variations)
- Custom wordlist creation
- Obfuscation techniques
- Auto-save to files

### 12. 📝 **Professional Report Generator**
- **Multiple Export Formats:**
  - PDF Reports
  - HTML Dashboards
  - JSON Data Export
  - TXT Summaries
  - CSV Data Files
- Visualizations
- Comprehensive analysis
- Executive summaries

---

## 🚀 Installation

### Requirements
- Rust 1.70+ (Recommended: 1.91+)
- Cargo package manager
- Linux, macOS, or Windows
- Internet connection

### Quick Install

```bash
# Clone the repository
git clone <repository-url>
cd legend-dorker

# Build using the automated script
chmod +x build.sh
./build.sh

# Run the tool
./LegendDorker
```

### Manual Build

```bash
# Build in release mode
cargo build --release

# Run
./target/release/legend-dorker
```

---

## 🎮 Usage Guide

### Starting the Tool

```bash
./LegendDorker
```

You'll be greeted with:
```
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
```

### Example Workflows

#### 1. Google Dorking
```
1. Select "Advanced Dork Checker"
2. Choose target domain
3. Select dork category (or all 100+)
4. Wait for results
5. Check saved database
```

#### 2. Vulnerability Assessment
```
1. Select "SQL Injection Scanner"
2. Enter target URL
3. Choose scan type (Quick/Deep/OWASP)
4. Review vulnerabilities found
5. Generate report
```

#### 3. OSINT Investigation
```
1. Select "Social Media OSINT"
2. Choose "Username Search"
3. Enter username
4. Wait for platform checks (15+ platforms)
5. View comprehensive results
```

#### 4. Network Reconnaissance
```
1. Select "Network Scanner"
2. Enter target IP/hostname
3. Choose scan type
4. View open ports & services
5. Export findings
```

---

## 📊 Configuration

Edit `config.json` or use the settings menu:

```json
{
  "threads": 10,
  "timeout": 30,
  "user_agent": "Mozilla/5.0...",
  "retry_attempts": 3,
  "verbose": false,
  "rate_limit": 100
}
```

---

## 💾 Database

All results stored in `legend_dorker.db` (SQLite):
- `dork_results` - Dork findings
- `sql_scans` - Vulnerability results
- `subdomains` - Discovered subdomains
- `port_scans` - Network scan data
- `hash_results` - Cracked hashes

---

## 🏆 Why LEGEND DORKER?

| Feature | Other Tools | LEGEND DORKER |
|---------|------------|---------------|
| Dork Patterns | 10-20 | **100+** |
| Categories | None | **14 Categories** |
| SQL Scanner | Basic | **OWASP Top 10** |
| OSINT | Limited | **15+ Platforms** |
| Fingerprinting | No | **✅ Advanced** |
| Exploit Finder | No | **✅ CVE Search** |
| SSL Analysis | No | **✅ Full Analysis** |
| Reports | Text only | **PDF/HTML/JSON** |
| Multi-threading | Limited | **✅ Advanced** |
| Database | No | **✅ SQLite** |
| Updates | Rare | **Active** |
| Support | None | **@LEGEND_BL** |

---

## 🎯 Use Cases

- **Bug Bounty Hunters** - Find vulnerabilities faster
- **Penetration Testers** - Comprehensive assessment tool
- **Security Researchers** - Advanced OSINT capabilities
- **Red Team Operations** - Multi-vector attack simulation
- **Blue Team** - Test defensive measures
- **Students** - Learn ethical hacking
- **OSINT Investigators** - Social media intelligence

---

## 📈 Performance

- ⚡ **100+ dorks/minute** with rate limiting
- 🚀 **Multi-threaded** scanning (up to 100 threads)
- 💾 **7.8MB** optimized binary
- 🔥 **GPU-accelerated** hash cracking
- 📊 **Real-time** progress tracking

---

## 🔮 Upcoming Features

- [ ] Machine learning pattern detection
- [ ] Cloud deployment support
- [ ] Web interface
- [ ] Mobile app
- [ ] Plugin system
- [ ] Shodan integration
- [ ] Censys integration
- [ ] VirusTotal API

---

## 🙏 Acknowledgments

Special thanks to:
- Rust Programming Language Community
- OWASP Foundation
- Bug Bounty Community
- Open Source Contributors
- All supporters of @LEGEND_BL

---

## 📜 License

MIT License - Free for Educational Use

```
Copyright (c) 2024 @LEGEND_BL

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files...
```

---

## 📞 Support & Contact

**Having issues? Found a bug? Want to contribute?**

- 📧 Email: sarthakgrid1@gmail.com
- 📱 Instagram: @sar_thak106
- 🐛 Report bugs via email
- 💡 Feature requests welcome
- 🤝 Contributions appreciated

---

## ⭐ Star This Project!

If you find LEGEND DORKER useful, please:
- ⭐ Star this repository
- 📢 Share with fellow researchers
- 🔄 Follow @LEGEND_BL for updates
- 💬 Provide feedback

---

<div align="center">

**🔥 LEGEND DORKER v3.0 🔥**

*The Ultimate OSINT & Security Framework*

Made with ❤️ by [@LEGEND_BL](https://instagram.com/sar_thak106)

**For Educational and Authorized Testing Only**

---

![Ethical Hacking](https://img.shields.io/badge/ethical-hacking-green.svg)
![Bug Bounty](https://img.shields.io/badge/bug-bounty-yellow.svg)
![OSINT](https://img.shields.io/badge/osint-tools-blue.svg)
![Security](https://img.shields.io/badge/security-research-red.svg)

</div>
