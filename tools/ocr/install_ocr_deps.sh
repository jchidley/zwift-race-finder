#!/usr/bin/env bash

set -euo pipefail

echo "Installing OCR system dependencies..."

sudo apt-get update

echo "Installing Tesseract and Leptonica..."
sudo apt-get install -y \
    libleptonica-dev \
    libtesseract-dev \
    tesseract-ocr \
    tesseract-ocr-eng || {
    echo "Failed to install Tesseract/Leptonica. Trying to fix..."
    sudo apt-get install -f -y
}

echo "Installing OpenCV and dependencies..."
sudo apt-get install -y \
    libzstd-dev \
    libopencv-dev \
    clang || {
    echo "Failed to install some packages. Checking what's available..."
    apt-cache policy libzstd-dev libopencv-dev
}


echo ""
echo "Checking installed packages..."
echo "=========================="

if pkg-config --exists lept; then
    echo "✓ Leptonica: $(pkg-config --modversion lept)"
else
    echo "✗ Leptonica not found"
fi

if pkg-config --exists tesseract; then
    echo "✓ Tesseract: $(pkg-config --modversion tesseract)"
else
    echo "✗ Tesseract not found"
fi

if command -v tesseract &> /dev/null; then
    echo "✓ Tesseract CLI: $(tesseract --version 2>&1 | head -n1)"
else
    echo "✗ Tesseract CLI not found"
fi

if pkg-config --exists opencv4; then
    echo "✓ OpenCV: $(pkg-config --modversion opencv4)"
elif pkg-config --exists opencv; then
    echo "✓ OpenCV: $(pkg-config --modversion opencv)"
else
    echo "⚠ OpenCV not found (optional)"
fi

echo ""
echo "Installation complete!"
echo ""
echo "You can now:"
echo "1. Run Python OCR: mask test"
echo "2. Build Rust OCR: mask rust-build"
echo "3. Compare both: mask compare screenshot.jpg"