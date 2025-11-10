#!/bin/bash
# Build script to create executable from Python script

echo "Building Swiss Army Suite executable..."

# Check if PyInstaller is installed
if ! command -v pyinstaller &> /dev/null; then
    echo "PyInstaller not found. Installing..."
    pip install pyinstaller
fi

# Build the executable
pyinstaller --onefile \
    --name SwissArmySuite \
    --add-data "README.md:." \
    --hidden-import argparse \
    --hidden-import json \
    --hidden-import base64 \
    --hidden-import hashlib \
    --hidden-import shutil \
    --hidden-import re \
    --hidden-import subprocess \
    --hidden-import platform \
    --hidden-import socket \
    --hidden-import time \
    --hidden-import datetime \
    --hidden-import zipfile \
    --hidden-import tarfile \
    --hidden-import gzip \
    --hidden-import urllib \
    --hidden-import http.client \
    --hidden-import ssl \
    --console \
    swiss_army_suite.py

if [ $? -eq 0 ]; then
    echo "✓ Build successful! Executable is in dist/SwissArmySuite"
    if [ "$(uname)" == "Linux" ]; then
        echo "  Linux executable: dist/SwissArmySuite"
    elif [ "$(uname)" == "Darwin" ]; then
        echo "  macOS executable: dist/SwissArmySuite"
    else
        echo "  Windows executable: dist/SwissArmySuite.exe"
    fi
else
    echo "✗ Build failed!"
    exit 1
fi
