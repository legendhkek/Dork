# Swiss Army Suite - Advanced Multi-Purpose Utility Tool

A comprehensive, advanced command-line utility suite providing a wide range of tools for file operations, text processing, system information, network utilities, data conversion, encryption, and archive handling.

## Features

### 🔧 File Operations
- **Copy/Move/Delete**: Advanced file manipulation with metadata preservation
- **File Search**: Recursive pattern-based file searching
- **Batch Rename**: Bulk file renaming with pattern matching
- **File Information**: Detailed file metadata and statistics

### 📝 Text Processing
- **Find & Replace**: Advanced text replacement with regex support
- **Line Extraction**: Extract lines matching patterns
- **Encoding Conversion**: Convert between different text encodings
- **Regex Support**: Full regular expression capabilities

### 💻 System Information
- **System Details**: Comprehensive system information display
- **Platform Detection**: Cross-platform system information
- **Resource Monitoring**: System resource details

### 🌐 Network Utilities
- **Ping**: Network connectivity testing
- **Port Scanner**: TCP port scanning capabilities
- **HTTP Client**: Make HTTP requests and inspect responses

### 🔄 Data Conversion
- **JSON Processing**: Pretty printing and formatting
- **Base64 Encoding/Decoding**: Data encoding utilities
- **Format Conversion**: Various data format conversions

### 🔐 Encryption & Hashing
- **File Hashing**: Calculate MD5, SHA1, SHA256, SHA512 hashes
- **String Hashing**: Hash text strings
- **Checksum Verification**: File integrity checking

### 📦 Archive Handling
- **ZIP Creation/Extraction**: Create and extract ZIP archives
- **Archive Listing**: List archive contents
- **Multi-format Support**: Support for various archive formats

## Installation

### Prerequisites
- Python 3.7 or higher
- Standard Python libraries (no external dependencies required for basic functionality)

### Setup
```bash
# Clone or download the repository
# Make the script executable (Linux/Mac)
chmod +x swiss_army_suite.py

# Or run directly with Python
python swiss_army_suite.py --help
```

## Usage

### General Syntax
```bash
python swiss_army_suite.py <module> <operation> [arguments]
```

### File Operations

#### Copy File
```bash
python swiss_army_suite.py file copy source.txt destination.txt
```

#### Move File
```bash
python swiss_army_suite.py file move source.txt destination.txt
```

#### Delete File
```bash
python swiss_army_suite.py file delete file.txt
```

#### Search Files
```bash
# Search recursively
python swiss_army_suite.py file search /path/to/dir "*.txt" --recursive

# Search in current directory only
python swiss_army_suite.py file search . "*.py"
```

#### Batch Rename
```bash
# Dry run (preview changes)
python swiss_army_suite.py file batch-rename /path/to/dir "old" "new" --dry-run

# Execute rename
python swiss_army_suite.py file batch-rename /path/to/dir "old" "new"
```

#### File Information
```bash
python swiss_army_suite.py file info file.txt
```

### Text Processing

#### Find and Replace
```bash
# Simple replacement
python swiss_army_suite.py text find-replace file.txt "old" "new"

# Regex replacement
python swiss_army_suite.py text find-replace file.txt "pattern" "replacement" --regex

# Case-insensitive replacement
python swiss_army_suite.py text find-replace file.txt "old" "new" --case-insensitive
```

#### Extract Matching Lines
```bash
# Simple pattern matching
python swiss_army_suite.py text extract-lines file.txt "error"

# Regex pattern matching
python swiss_army_suite.py text extract-lines file.txt "error|warning" --regex
```

### System Information

#### Display System Info
```bash
python swiss_army_suite.py system info
```

### Network Utilities

#### Ping Host
```bash
python swiss_army_suite.py network ping google.com
python swiss_army_suite.py network ping 8.8.8.8 --count 10
```

#### Port Scan
```bash
python swiss_army_suite.py network port-scan localhost 80 443 8080
python swiss_army_suite.py network port-scan example.com 20 21 22 23 25 80 443
```

### Data Conversion

#### JSON Pretty Print
```bash
python swiss_army_suite.py data json-pretty data.json
```

#### Base64 Encode
```bash
# Output to console
python swiss_army_suite.py data base64-encode "Hello World"

# Output to file
python swiss_army_suite.py data base64-encode "Hello World" --output encoded.txt
```

#### Base64 Decode
```bash
python swiss_army_suite.py data base64-decode "SGVsbG8gV29ybGQ="
python swiss_army_suite.py data base64-decode "SGVsbG8gV29ybGQ=" --output decoded.txt
```

### Encryption & Hashing

#### Hash File
```bash
# SHA256 (default)
python swiss_army_suite.py crypto hash-file file.txt

# MD5
python swiss_army_suite.py crypto hash-file file.txt md5

# SHA512
python swiss_army_suite.py crypto hash-file file.txt sha512
```

#### Hash String
```bash
python swiss_army_suite.py crypto hash-string "Hello World"
python swiss_army_suite.py crypto hash-string "Hello World" sha256
```

### Archive Operations

#### Create ZIP Archive
```bash
python swiss_army_suite.py archive create archive.zip file1.txt file2.txt
python swiss_army_suite.py archive create backup.zip /path/to/directory
```

#### Extract Archive
```bash
python swiss_army_suite.py archive extract archive.zip output_directory
```

#### List Archive Contents
```bash
python swiss_army_suite.py archive list archive.zip
```

## Advanced Features

### Color-Coded Output
The tool provides color-coded terminal output for better readability:
- ✓ Green: Success messages
- ✗ Red: Error messages
- ℹ Cyan: Information messages
- ⚠ Yellow: Warning messages

### Error Handling
Comprehensive error handling with informative error messages and graceful failure handling.

### Cross-Platform Support
Works on Windows, Linux, and macOS with platform-specific optimizations.

## Building an Executable

To create a standalone executable (optional):

```bash
# Install PyInstaller
pip install pyinstaller

# Build executable
pyinstaller --onefile --name SwissArmySuite swiss_army_suite.py

# The executable will be in the dist/ directory
```

## Examples

### Example 1: Batch Process Files
```bash
# Find all Python files
python swiss_army_suite.py file search . "*.py" --recursive

# Replace import statements in all files
for file in $(python swiss_army_suite.py file search . "*.py" --recursive); do
    python swiss_army_suite.py text find-replace "$file" "import old" "import new"
done
```

### Example 2: Verify File Integrity
```bash
# Calculate hash of downloaded file
python swiss_army_suite.py crypto hash-file downloaded_file.zip sha256

# Compare with expected hash
```

### Example 3: Network Diagnostics
```bash
# Check connectivity
python swiss_army_suite.py network ping google.com

# Scan common ports
python swiss_army_suite.py network port-scan localhost 22 80 443 3306 5432
```

## Contributing

This is an advanced utility suite designed to be extensible. New modules and features can be easily added by extending the existing class structure.

## License

This tool is provided as-is for utility purposes.

## Version

Version 2.0 - Advanced Edition
