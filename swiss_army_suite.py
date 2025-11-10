#!/usr/bin/env python3
"""
Swiss Army Suite - Advanced Multi-Purpose Utility Tool
A comprehensive collection of utilities for various tasks.
"""

import argparse
import sys
import os
from pathlib import Path
from typing import List, Optional, Dict, Any
import json
import base64
import hashlib
import shutil
import re
import subprocess
import platform
import socket
import time
from datetime import datetime
import zipfile
import tarfile
import gzip
from urllib.parse import urlparse
import http.client
import ssl

# Color codes for terminal output
class Colors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

def print_colored(text: str, color: str = Colors.ENDC):
    """Print colored text to terminal."""
    print(f"{color}{text}{Colors.ENDC}")

def print_header(text: str):
    """Print formatted header."""
    print_colored(f"\n{'='*60}", Colors.BOLD)
    print_colored(f"  {text}", Colors.HEADER)
    print_colored(f"{'='*60}\n", Colors.BOLD)

def print_success(text: str):
    """Print success message."""
    print_colored(f"✓ {text}", Colors.OKGREEN)

def print_error(text: str):
    """Print error message."""
    print_colored(f"✗ {text}", Colors.FAIL)

def print_info(text: str):
    """Print info message."""
    print_colored(f"ℹ {text}", Colors.OKCYAN)

def print_warning(text: str):
    """Print warning message."""
    print_colored(f"⚠ {text}", Colors.WARNING)


# ============================================================================
# FILE OPERATIONS MODULE
# ============================================================================

class FileOperations:
    """Advanced file operations utilities."""
    
    @staticmethod
    def copy_file(src: str, dst: str, preserve_metadata: bool = True) -> bool:
        """Copy file with optional metadata preservation."""
        try:
            if preserve_metadata:
                shutil.copy2(src, dst)
            else:
                shutil.copy(src, dst)
            print_success(f"Copied: {src} -> {dst}")
            return True
        except Exception as e:
            print_error(f"Copy failed: {e}")
            return False
    
    @staticmethod
    def move_file(src: str, dst: str) -> bool:
        """Move file from source to destination."""
        try:
            shutil.move(src, dst)
            print_success(f"Moved: {src} -> {dst}")
            return True
        except Exception as e:
            print_error(f"Move failed: {e}")
            return False
    
    @staticmethod
    def delete_file(filepath: str, force: bool = False) -> bool:
        """Delete file with optional force flag."""
        try:
            if not os.path.exists(filepath) and not force:
                print_warning(f"File not found: {filepath}")
                return False
            if os.path.exists(filepath):
                os.remove(filepath)
                print_success(f"Deleted: {filepath}")
            return True
        except Exception as e:
            print_error(f"Delete failed: {e}")
            return False
    
    @staticmethod
    def search_files(directory: str, pattern: str, recursive: bool = True) -> List[str]:
        """Search for files matching pattern."""
        matches = []
        try:
            path = Path(directory)
            if recursive:
                matches = [str(p) for p in path.rglob(pattern)]
            else:
                matches = [str(p) for p in path.glob(pattern)]
            print_info(f"Found {len(matches)} matches for pattern '{pattern}'")
            return matches
        except Exception as e:
            print_error(f"Search failed: {e}")
            return []
    
    @staticmethod
    def batch_rename(directory: str, pattern: str, replacement: str, dry_run: bool = False) -> int:
        """Batch rename files matching pattern."""
        renamed = 0
        try:
            for filepath in Path(directory).iterdir():
                if filepath.is_file():
                    new_name = filepath.name.replace(pattern, replacement)
                    if new_name != filepath.name:
                        new_path = filepath.parent / new_name
                        if dry_run:
                            print_info(f"Would rename: {filepath.name} -> {new_name}")
                        else:
                            filepath.rename(new_path)
                            print_success(f"Renamed: {filepath.name} -> {new_name}")
                        renamed += 1
            return renamed
        except Exception as e:
            print_error(f"Batch rename failed: {e}")
            return renamed
    
    @staticmethod
    def get_file_info(filepath: str) -> Dict[str, Any]:
        """Get detailed file information."""
        try:
            stat = os.stat(filepath)
            return {
                'path': filepath,
                'size': stat.st_size,
                'modified': datetime.fromtimestamp(stat.st_mtime).isoformat(),
                'created': datetime.fromtimestamp(stat.st_ctime).isoformat(),
                'permissions': oct(stat.st_mode)[-3:],
                'is_file': os.path.isfile(filepath),
                'is_dir': os.path.isdir(filepath)
            }
        except Exception as e:
            print_error(f"Failed to get file info: {e}")
            return {}


# ============================================================================
# TEXT PROCESSING MODULE
# ============================================================================

class TextProcessing:
    """Advanced text processing utilities."""
    
    @staticmethod
    def find_replace(filepath: str, find: str, replace: str, regex: bool = False, 
                     case_sensitive: bool = True) -> int:
        """Find and replace text in file."""
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            if regex:
                flags = 0 if case_sensitive else re.IGNORECASE
                content = re.sub(find, replace, content, flags=flags)
            else:
                if case_sensitive:
                    content = content.replace(find, replace)
                else:
                    # Case-insensitive replacement
                    pattern = re.compile(re.escape(find), re.IGNORECASE)
                    content = pattern.sub(replace, content)
            
            if content != original_content:
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(content)
                replacements = len(re.findall(re.escape(find), original_content, 
                                             flags=0 if case_sensitive else re.IGNORECASE))
                print_success(f"Replaced {replacements} occurrence(s) in {filepath}")
                return replacements
            else:
                print_info(f"No matches found in {filepath}")
                return 0
        except Exception as e:
            print_error(f"Find/replace failed: {e}")
            return 0
    
    @staticmethod
    def convert_encoding(filepath: str, from_enc: str, to_enc: str) -> bool:
        """Convert file encoding."""
        try:
            with open(filepath, 'r', encoding=from_enc) as f:
                content = f.read()
            
            with open(filepath, 'w', encoding=to_enc) as f:
                f.write(content)
            
            print_success(f"Converted encoding: {from_enc} -> {to_enc}")
            return True
        except Exception as e:
            print_error(f"Encoding conversion failed: {e}")
            return False
    
    @staticmethod
    def extract_lines(filepath: str, pattern: str, regex: bool = False) -> List[str]:
        """Extract lines matching pattern."""
        matches = []
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                for line_num, line in enumerate(f, 1):
                    if regex:
                        if re.search(pattern, line):
                            matches.append(f"{line_num}: {line.rstrip()}")
                    else:
                        if pattern in line:
                            matches.append(f"{line_num}: {line.rstrip()}")
            print_info(f"Found {len(matches)} matching lines")
            return matches
        except Exception as e:
            print_error(f"Extract lines failed: {e}")
            return []


# ============================================================================
# SYSTEM INFORMATION MODULE
# ============================================================================

class SystemInfo:
    """System information and monitoring utilities."""
    
    @staticmethod
    def get_system_info() -> Dict[str, Any]:
        """Get comprehensive system information."""
        info = {
            'platform': platform.platform(),
            'system': platform.system(),
            'release': platform.release(),
            'version': platform.version(),
            'machine': platform.machine(),
            'processor': platform.processor(),
            'hostname': socket.gethostname(),
            'python_version': platform.python_version(),
        }
        
        # Try to get additional info
        try:
            if platform.system() == 'Linux':
                with open('/proc/cpuinfo', 'r') as f:
                    cpu_info = f.read()
                    info['cpu_cores'] = cpu_info.count('processor')
        except:
            pass
        
        return info
    
    @staticmethod
    def display_system_info():
        """Display formatted system information."""
        info = SystemInfo.get_system_info()
        print_header("System Information")
        for key, value in info.items():
            print(f"  {key.replace('_', ' ').title()}: {value}")


# ============================================================================
# NETWORK UTILITIES MODULE
# ============================================================================

class NetworkUtils:
    """Network utilities."""
    
    @staticmethod
    def ping(host: str, count: int = 4, timeout: int = 3) -> bool:
        """Ping a host."""
        print_info(f"Pinging {host}...")
        try:
            if platform.system() == 'Windows':
                result = subprocess.run(['ping', '-n', str(count), host], 
                                      capture_output=True, text=True, timeout=timeout*count)
            else:
                result = subprocess.run(['ping', '-c', str(count), host], 
                                      capture_output=True, text=True, timeout=timeout*count)
            
            print(result.stdout)
            return result.returncode == 0
        except Exception as e:
            print_error(f"Ping failed: {e}")
            return False
    
    @staticmethod
    def port_scan(host: str, ports: List[int], timeout: float = 1.0) -> Dict[int, bool]:
        """Scan ports on a host."""
        results = {}
        print_info(f"Scanning ports on {host}...")
        
        for port in ports:
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(timeout)
            try:
                result = sock.connect_ex((host, port))
                is_open = result == 0
                results[port] = is_open
                if is_open:
                    print_success(f"Port {port}: OPEN")
                else:
                    print_info(f"Port {port}: CLOSED")
            except Exception as e:
                results[port] = False
                print_error(f"Port {port}: ERROR - {e}")
            finally:
                sock.close()
        
        return results
    
    @staticmethod
    def http_request(url: str, method: str = 'GET', headers: Optional[Dict] = None) -> Dict[str, Any]:
        """Make HTTP request."""
        try:
            parsed = urlparse(url)
            host = parsed.netloc
            path = parsed.path or '/'
            
            conn = http.client.HTTPConnection(host)
            conn.request(method, path, headers=headers or {})
            response = conn.getresponse()
            
            result = {
                'status_code': response.status,
                'status_message': response.reason,
                'headers': dict(response.getheaders()),
                'body': response.read().decode('utf-8', errors='ignore')
            }
            
            print_success(f"HTTP {method} {url} - Status: {result['status_code']}")
            return result
        except Exception as e:
            print_error(f"HTTP request failed: {e}")
            return {}


# ============================================================================
# DATA CONVERSION MODULE
# ============================================================================

class DataConversion:
    """Data conversion utilities."""
    
    @staticmethod
    def json_pretty_print(filepath: str) -> bool:
        """Pretty print JSON file."""
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                data = json.load(f)
            
            with open(filepath, 'w', encoding='utf-8') as f:
                json.dump(data, f, indent=2, ensure_ascii=False)
            
            print_success(f"Pretty printed JSON: {filepath}")
            return True
        except Exception as e:
            print_error(f"JSON pretty print failed: {e}")
            return False
    
    @staticmethod
    def base64_encode(data: str, output_file: Optional[str] = None) -> str:
        """Encode string to base64."""
        encoded = base64.b64encode(data.encode('utf-8')).decode('utf-8')
        if output_file:
            with open(output_file, 'w') as f:
                f.write(encoded)
            print_success(f"Base64 encoded data written to {output_file}")
        return encoded
    
    @staticmethod
    def base64_decode(data: str, output_file: Optional[str] = None) -> str:
        """Decode base64 string."""
        try:
            decoded = base64.b64decode(data).decode('utf-8')
            if output_file:
                with open(output_file, 'w', encoding='utf-8') as f:
                    f.write(decoded)
                print_success(f"Base64 decoded data written to {output_file}")
            return decoded
        except Exception as e:
            print_error(f"Base64 decode failed: {e}")
            return ""


# ============================================================================
# ENCRYPTION MODULE
# ============================================================================

class Encryption:
    """Encryption and hashing utilities."""
    
    @staticmethod
    def hash_file(filepath: str, algorithm: str = 'sha256') -> str:
        """Calculate file hash."""
        try:
            hash_obj = hashlib.new(algorithm)
            with open(filepath, 'rb') as f:
                for chunk in iter(lambda: f.read(4096), b''):
                    hash_obj.update(chunk)
            hash_value = hash_obj.hexdigest()
            print_success(f"{algorithm.upper()} hash: {hash_value}")
            return hash_value
        except Exception as e:
            print_error(f"Hash calculation failed: {e}")
            return ""
    
    @staticmethod
    def hash_string(data: str, algorithm: str = 'sha256') -> str:
        """Calculate string hash."""
        hash_obj = hashlib.new(algorithm)
        hash_obj.update(data.encode('utf-8'))
        hash_value = hash_obj.hexdigest()
        print_success(f"{algorithm.upper()} hash: {hash_value}")
        return hash_value


# ============================================================================
# ARCHIVE MODULE
# ============================================================================

class ArchiveUtils:
    """Archive handling utilities."""
    
    @staticmethod
    def create_zip(archive_path: str, files: List[str]) -> bool:
        """Create ZIP archive."""
        try:
            with zipfile.ZipFile(archive_path, 'w', zipfile.ZIP_DEFLATED) as zipf:
                for file in files:
                    if os.path.isfile(file):
                        zipf.write(file, os.path.basename(file))
                    elif os.path.isdir(file):
                        for root, dirs, filenames in os.walk(file):
                            for filename in filenames:
                                file_path = os.path.join(root, filename)
                                arcname = os.path.relpath(file_path, os.path.dirname(file))
                                zipf.write(file_path, arcname)
            print_success(f"Created ZIP archive: {archive_path}")
            return True
        except Exception as e:
            print_error(f"ZIP creation failed: {e}")
            return False
    
    @staticmethod
    def extract_zip(archive_path: str, extract_to: str) -> bool:
        """Extract ZIP archive."""
        try:
            with zipfile.ZipFile(archive_path, 'r') as zipf:
                zipf.extractall(extract_to)
            print_success(f"Extracted ZIP archive to: {extract_to}")
            return True
        except Exception as e:
            print_error(f"ZIP extraction failed: {e}")
            return False
    
    @staticmethod
    def list_archive(archive_path: str) -> List[str]:
        """List contents of archive."""
        try:
            if archive_path.endswith('.zip'):
                with zipfile.ZipFile(archive_path, 'r') as zipf:
                    files = zipf.namelist()
            elif archive_path.endswith(('.tar', '.tar.gz', '.tgz')):
                with tarfile.open(archive_path, 'r:*') as tarf:
                    files = tarf.getnames()
            else:
                print_error(f"Unsupported archive format: {archive_path}")
                return []
            
            print_info(f"Archive contains {len(files)} file(s):")
            for file in files:
                print(f"  - {file}")
            return files
        except Exception as e:
            print_error(f"List archive failed: {e}")
            return []


# ============================================================================
# MAIN CLI INTERFACE
# ============================================================================

def main():
    parser = argparse.ArgumentParser(
        description='Swiss Army Suite - Advanced Multi-Purpose Utility Tool',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # File operations
  python swiss_army_suite.py file copy src.txt dst.txt
  python swiss_army_suite.py file search /path/to/dir "*.txt"
  python swiss_army_suite.py file batch-rename /path/to/dir old new
  
  # Text processing
  python swiss_army_suite.py text find-replace file.txt "old" "new"
  python swiss_army_suite.py text extract-lines file.txt "pattern"
  
  # System info
  python swiss_army_suite.py system info
  
  # Network
  python swiss_army_suite.py network ping google.com
  python swiss_army_suite.py network port-scan localhost 80 443 8080
  
  # Data conversion
  python swiss_army_suite.py data json-pretty file.json
  python swiss_army_suite.py data base64-encode "Hello World"
  
  # Encryption
  python swiss_army_suite.py crypto hash-file file.txt sha256
  
  # Archives
  python swiss_army_suite.py archive create archive.zip file1.txt file2.txt
  python swiss_army_suite.py archive extract archive.zip output_dir
        """
    )
    
    subparsers = parser.add_subparsers(dest='module', help='Module to use')
    
    # File operations
    file_parser = subparsers.add_parser('file', help='File operations')
    file_subparsers = file_parser.add_subparsers(dest='operation')
    
    copy_parser = file_subparsers.add_parser('copy', help='Copy file')
    copy_parser.add_argument('source', help='Source file')
    copy_parser.add_argument('destination', help='Destination file')
    
    move_parser = file_subparsers.add_parser('move', help='Move file')
    move_parser.add_argument('source', help='Source file')
    move_parser.add_argument('destination', help='Destination file')
    
    delete_parser = file_subparsers.add_parser('delete', help='Delete file')
    delete_parser.add_argument('filepath', help='File to delete')
    
    search_parser = file_subparsers.add_parser('search', help='Search files')
    search_parser.add_argument('directory', help='Directory to search')
    search_parser.add_argument('pattern', help='File pattern')
    search_parser.add_argument('--recursive', '-r', action='store_true', help='Recursive search')
    
    batch_rename_parser = file_subparsers.add_parser('batch-rename', help='Batch rename files')
    batch_rename_parser.add_argument('directory', help='Directory')
    batch_rename_parser.add_argument('pattern', help='Pattern to replace')
    batch_rename_parser.add_argument('replacement', help='Replacement text')
    batch_rename_parser.add_argument('--dry-run', action='store_true', help='Dry run')
    
    info_parser = file_subparsers.add_parser('info', help='Get file info')
    info_parser.add_argument('filepath', help='File path')
    
    # Text processing
    text_parser = subparsers.add_parser('text', help='Text processing')
    text_subparsers = text_parser.add_subparsers(dest='operation')
    
    find_replace_parser = text_subparsers.add_parser('find-replace', help='Find and replace')
    find_replace_parser.add_argument('filepath', help='File path')
    find_replace_parser.add_argument('find', help='Text to find')
    find_replace_parser.add_argument('replace', help='Replacement text')
    find_replace_parser.add_argument('--regex', action='store_true', help='Use regex')
    find_replace_parser.add_argument('--case-insensitive', '-i', action='store_true', help='Case insensitive')
    
    extract_parser = text_subparsers.add_parser('extract-lines', help='Extract matching lines')
    extract_parser.add_argument('filepath', help='File path')
    extract_parser.add_argument('pattern', help='Pattern to match')
    extract_parser.add_argument('--regex', action='store_true', help='Use regex')
    
    # System info
    system_parser = subparsers.add_parser('system', help='System information')
    system_subparsers = system_parser.add_subparsers(dest='operation')
    system_subparsers.add_parser('info', help='Display system information')
    
    # Network
    network_parser = subparsers.add_parser('network', help='Network utilities')
    network_subparsers = network_parser.add_subparsers(dest='operation')
    
    ping_parser = network_subparsers.add_parser('ping', help='Ping host')
    ping_parser.add_argument('host', help='Host to ping')
    ping_parser.add_argument('--count', '-c', type=int, default=4, help='Number of pings')
    
    port_scan_parser = network_subparsers.add_parser('port-scan', help='Scan ports')
    port_scan_parser.add_argument('host', help='Host to scan')
    port_scan_parser.add_argument('ports', nargs='+', type=int, help='Ports to scan')
    
    # Data conversion
    data_parser = subparsers.add_parser('data', help='Data conversion')
    data_subparsers = data_parser.add_subparsers(dest='operation')
    
    json_parser = data_subparsers.add_parser('json-pretty', help='Pretty print JSON')
    json_parser.add_argument('filepath', help='JSON file path')
    
    base64_encode_parser = data_subparsers.add_parser('base64-encode', help='Base64 encode')
    base64_encode_parser.add_argument('data', help='Data to encode')
    base64_encode_parser.add_argument('--output', '-o', help='Output file')
    
    base64_decode_parser = data_subparsers.add_parser('base64-decode', help='Base64 decode')
    base64_decode_parser.add_argument('data', help='Data to decode')
    base64_decode_parser.add_argument('--output', '-o', help='Output file')
    
    # Encryption
    crypto_parser = subparsers.add_parser('crypto', help='Encryption and hashing')
    crypto_subparsers = crypto_parser.add_subparsers(dest='operation')
    
    hash_file_parser = crypto_subparsers.add_parser('hash-file', help='Hash file')
    hash_file_parser.add_argument('filepath', help='File path')
    hash_file_parser.add_argument('algorithm', nargs='?', default='sha256', 
                                  choices=['md5', 'sha1', 'sha256', 'sha512'],
                                  help='Hash algorithm')
    
    hash_string_parser = crypto_subparsers.add_parser('hash-string', help='Hash string')
    hash_string_parser.add_argument('data', help='String to hash')
    hash_string_parser.add_argument('algorithm', nargs='?', default='sha256',
                                   choices=['md5', 'sha1', 'sha256', 'sha512'],
                                   help='Hash algorithm')
    
    # Archives
    archive_parser = subparsers.add_parser('archive', help='Archive operations')
    archive_subparsers = archive_parser.add_subparsers(dest='operation')
    
    create_archive_parser = archive_subparsers.add_parser('create', help='Create archive')
    create_archive_parser.add_argument('archive_path', help='Archive path')
    create_archive_parser.add_argument('files', nargs='+', help='Files to archive')
    
    extract_archive_parser = archive_subparsers.add_parser('extract', help='Extract archive')
    extract_archive_parser.add_argument('archive_path', help='Archive path')
    extract_archive_parser.add_argument('extract_to', help='Extraction directory')
    
    list_archive_parser = archive_subparsers.add_parser('list', help='List archive contents')
    list_archive_parser.add_argument('archive_path', help='Archive path')
    
    args = parser.parse_args()
    
    if not args.module:
        parser.print_help()
        return
    
    # Execute operations
    try:
        if args.module == 'file':
            if args.operation == 'copy':
                FileOperations.copy_file(args.source, args.destination)
            elif args.operation == 'move':
                FileOperations.move_file(args.source, args.destination)
            elif args.operation == 'delete':
                FileOperations.delete_file(args.filepath)
            elif args.operation == 'search':
                matches = FileOperations.search_files(args.directory, args.pattern, args.recursive)
                for match in matches:
                    print(f"  {match}")
            elif args.operation == 'batch-rename':
                FileOperations.batch_rename(args.directory, args.pattern, args.replacement, args.dry_run)
            elif args.operation == 'info':
                info = FileOperations.get_file_info(args.filepath)
                print_header("File Information")
                for key, value in info.items():
                    print(f"  {key.replace('_', ' ').title()}: {value}")
        
        elif args.module == 'text':
            if args.operation == 'find-replace':
                TextProcessing.find_replace(args.filepath, args.find, args.replace, 
                                          args.regex, not args.case_insensitive)
            elif args.operation == 'extract-lines':
                matches = TextProcessing.extract_lines(args.filepath, args.pattern, args.regex)
                for match in matches:
                    print(f"  {match}")
        
        elif args.module == 'system':
            if args.operation == 'info':
                SystemInfo.display_system_info()
        
        elif args.module == 'network':
            if args.operation == 'ping':
                NetworkUtils.ping(args.host, args.count)
            elif args.operation == 'port-scan':
                NetworkUtils.port_scan(args.host, args.ports)
        
        elif args.module == 'data':
            if args.operation == 'json-pretty':
                DataConversion.json_pretty_print(args.filepath)
            elif args.operation == 'base64-encode':
                encoded = DataConversion.base64_encode(args.data, args.output)
                if not args.output:
                    print(encoded)
            elif args.operation == 'base64-decode':
                decoded = DataConversion.base64_decode(args.data, args.output)
                if not args.output:
                    print(decoded)
        
        elif args.module == 'crypto':
            if args.operation == 'hash-file':
                Encryption.hash_file(args.filepath, args.algorithm)
            elif args.operation == 'hash-string':
                Encryption.hash_string(args.data, args.algorithm)
        
        elif args.module == 'archive':
            if args.operation == 'create':
                ArchiveUtils.create_zip(args.archive_path, args.files)
            elif args.operation == 'extract':
                ArchiveUtils.extract_zip(args.archive_path, args.extract_to)
            elif args.operation == 'list':
                ArchiveUtils.list_archive(args.archive_path)
    
    except KeyboardInterrupt:
        print_warning("\nOperation cancelled by user")
        sys.exit(1)
    except Exception as e:
        print_error(f"Error: {e}")
        sys.exit(1)


if __name__ == '__main__':
    main()
