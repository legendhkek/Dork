# 🛠️ Swiss Army Suite v2.0 - Advanced Security Toolkit

<div align="center">

![Version](https://img.shields.io/badge/version-2.0.0-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

**A comprehensive, multi-functional security testing and penetration testing toolkit**

</div>

## ⚠️ Legal Disclaimer

**IMPORTANT:** This tool is designed for **EDUCATIONAL PURPOSES** and **AUTHORIZED SECURITY TESTING ONLY**.

- ✅ Use only on systems you own or have explicit written permission to test
- ✅ Respect all applicable local, state, and federal laws
- ❌ Unauthorized access to computer systems is illegal
- ❌ The authors are not responsible for misuse of this software

**By using this software, you agree to use it responsibly and ethically.**

---

## 🚀 Features

### 1. 🎯 Advanced Dork Checker & Google Hacking
- Single and bulk dork checking
- Random dork generation
- Pre-built dork templates for common vulnerabilities
- Results saved to database

### 2. 🔓 SQL Injection Vulnerability Scanner
- Quick GET parameter scanning
- Deep scan (GET, POST, Headers)
- OWASP Top 10 comprehensive testing
- Blind SQL injection detection (time-based & boolean-based)
- Custom payload injection
- Automatic error signature detection

### 3. 🕸️ Web Scraper & Crawler Suite
- Intelligent web crawling with depth control
- Content extraction and parsing
- Link extraction (internal & external)
- Email and contact information harvester
- Image URL extraction

### 4. 🌐 Network Scanner & Port Analyzer
- Fast multi-threaded port scanning
- Service detection and fingerprinting
- Common vulnerability checks
- Network range scanning (CIDR notation)
- Open port identification

### 5. 🔐 Hash Cracker & Encryption Tools
- Hash type identification
- Rainbow table attacks
- Dictionary attacks (with custom wordlists)
- Brute force capabilities
- Multiple hash algorithms (MD5, SHA-1, SHA-256, SHA-512)
- Encoding/Decoding (Base64, Hex, ROT13)
- Hash generation tool

### 6. 🌍 Subdomain Finder & DNS Enumeration
- Subdomain enumeration with 100+ common subdomains
- DNS record lookup
- Zone transfer vulnerability testing
- DNS brute forcing
- Results export to text files

### 7. 📊 Payload & Keyword Generator
- SQL injection payload generation
- XSS payload library
- Keyword mutation and combination
- Custom wordlist creation

### 8. ⚙️ Configuration & Settings
- Adjustable thread count
- Timeout configuration
- Custom User-Agent strings
- Retry attempt settings
- Verbose output mode
- Rate limiting

---

## 📋 Requirements

- **Rust** 1.70 or higher
- **Cargo** (Rust package manager)
- Internet connection (for network-based features)
- Linux, macOS, or Windows

---

## 🔧 Installation

### Quick Install

```bash
# Clone the repository
git clone <repository-url>
cd swiss-army-suite

# Build in release mode
cargo build --release

# Run the executable
./target/release/swiss-army-suite
```

### Using the Build Script

```bash
# Make the build script executable
chmod +x build.sh

# Run the build script
./build.sh

# The compiled binary will be in the current directory
./SwissArmySuite
```

---

## 🎮 Usage

### Starting the Application

```bash
./SwissArmySuite
```

### Main Menu

Upon starting, you'll see an interactive menu:

```
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
```

### Example Workflows

#### SQL Injection Testing
```
1. Select "SQL Injection Vulnerability Scanner"
2. Choose scan type (Quick, Deep, OWASP, Blind, Custom)
3. Enter target URL (e.g., http://example.com/page?id=1)
4. Wait for results
5. View findings in the database or statistics menu
```

#### Subdomain Enumeration
```
1. Select "Subdomain Finder & DNS Enumeration"
2. Choose "Subdomain Enumeration"
3. Enter target domain (e.g., example.com)
4. Results will be displayed and saved to file
```

#### Hash Cracking
```
1. Select "Hash Cracker & Encryption Tools"
2. Choose attack method (Rainbow Table, Dictionary, Brute Force)
3. Enter hash to crack
4. For dictionary attack, provide wordlist path
5. Wait for results
```

#### Port Scanning
```
1. Select "Network Scanner & Port Analyzer"
2. Choose "Advanced Port Scanner"
3. Enter target IP or hostname
4. Common ports will be scanned automatically
5. View open ports and identified services
```

---

## 🗂️ Project Structure

```
swiss-army-suite/
├── Cargo.toml              # Rust dependencies and project config
├── README.md               # This file
├── build.sh                # Build script
├── src/
│   ├── main.rs             # Main application and menu system
│   ├── config.rs           # Configuration management
│   ├── database.rs         # SQLite database operations
│   ├── dork_checker.rs     # Google dorking functionality
│   ├── sql_scanner.rs      # SQL injection scanner
│   ├── web_tools.rs        # Web scraping and crawling
│   ├── network_tools.rs    # Port scanning and network tools
│   ├── crypto_tools.rs     # Hash cracking and encryption
│   └── dns_tools.rs        # DNS enumeration tools
├── config.json             # Generated configuration file
├── swiss_army.db           # SQLite database (auto-created)
└── target/                 # Build output directory
```

---

## ⚙️ Configuration

The application creates a `config.json` file with customizable settings:

```json
{
  "threads": 10,
  "timeout": 30,
  "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64)...",
  "retry_attempts": 3,
  "verbose": false,
  "proxy": null,
  "rate_limit": 100
}
```

Modify settings through the Configuration menu or edit the JSON file directly.

---

## 📊 Database

All scan results are stored in `swiss_army.db` (SQLite):

- `dork_results` - Google dork findings
- `sql_scans` - SQL injection test results
- `subdomains` - Discovered subdomains
- `port_scans` - Port scan results
- `hash_results` - Cracked hashes

View statistics through the "View Statistics & Reports" menu option.

---

## 🔐 Security Best Practices

1. **Always get permission** before testing any system
2. **Use responsibly** - Don't overload targets with requests
3. **Respect rate limits** - Configure appropriate delays
4. **Keep logs secure** - Database contains sensitive information
5. **Stay updated** - Keep the tool and dependencies up to date
6. **Test on your own infrastructure first**
7. **Follow responsible disclosure** for any findings

---

## 🐛 Known Limitations

- DNS zone transfer requires additional system tools
- Some features are demonstrations and may need enhancement
- Rate limiting may be needed for large-scale scans
- Brute force capabilities are limited (use specialized tools)
- Certificate validation is disabled for testing (security consideration)

---

## 🚀 Roadmap

Future enhancements planned:

- [ ] Multi-threaded subdomain enumeration
- [ ] API integration for threat intelligence
- [ ] Report generation (PDF, HTML)
- [ ] Proxy support
- [ ] VPN integration
- [ ] Advanced payload encoding
- [ ] Web interface
- [ ] Plugin system

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

---

## 📝 License

This project is licensed under the MIT License.

```
MIT License

Copyright (c) 2024 Swiss Army Suite

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 🙏 Acknowledgments

- Rust community for excellent crates
- Security researchers for vulnerability patterns
- Open source security tools for inspiration

---

## 📧 Support

For issues, questions, or feature requests:
- Open an issue on GitHub
- Check existing documentation
- Review the code comments

---

## ⚡ Quick Tips

1. **Start with Quick Scan** - Test functionality before deep scans
2. **Configure Threads** - Adjust based on your system capabilities
3. **Use Verbose Mode** - For detailed debugging information
4. **Save Results** - All findings are automatically saved to database
5. **Export Data** - Use the statistics menu to view summaries
6. **Test Safely** - Always use test environments first

---

<div align="center">

**Built with ❤️ using Rust**

⭐ Star this repository if you find it useful!

</div>
