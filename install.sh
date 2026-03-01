#!/bin/bash

set -e

echo "Web Radio TUI - Installation Script"
echo "===================================="
echo ""

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed."
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust installed successfully!"
else
    echo "✅ Rust is already installed"
fi

# Detect OS and check for audio libraries
echo ""
echo "Checking system dependencies..."

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "🐧 Detected Linux"
    
    # Check for ALSA
    if ! pkg-config --exists alsa; then
        echo "❌ ALSA development libraries not found"
        echo ""
        
        # Detect package manager
        if command -v dnf &> /dev/null; then
            echo "📦 Installing ALSA libraries with dnf..."
            sudo dnf install -y alsa-lib-devel
        elif command -v apt-get &> /dev/null; then
            echo "📦 Installing ALSA libraries with apt..."
            sudo apt-get update && sudo apt-get install -y libasound2-dev
        elif command -v pacman &> /dev/null; then
            echo "📦 Installing ALSA libraries with pacman..."
            sudo pacman -S --noconfirm alsa-lib
        elif command -v zypper &> /dev/null; then
            echo "📦 Installing ALSA libraries with zypper..."
            sudo zypper install -y alsa-devel
        else
            echo "⚠️  Could not detect package manager."
            echo "Please install ALSA development libraries manually:"
            echo "  - Fedora/RHEL: sudo dnf install alsa-lib-devel"
            echo "  - Debian/Ubuntu: sudo apt-get install libasound2-dev"
            echo "  - Arch: sudo pacman -S alsa-lib"
            exit 1
        fi
        
        echo "✅ ALSA libraries installed!"
    else
        echo "✅ ALSA libraries are installed"
    fi
    
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "🍎 Detected macOS"
    echo "✅ No additional dependencies needed (using CoreAudio)"
    
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    echo "🪟 Detected Windows"
    echo "✅ No additional dependencies needed (using WASAPI)"
else
    echo "⚠️  Unknown OS: $OSTYPE"
fi

echo ""
echo "Building Web Radio TUI..."
cargo build --release

echo ""
echo "✅ Build successful!"
echo ""
echo "To run the application:"
echo "  cargo run --release"
echo ""
echo "Or install it globally:"
echo "  cargo install --path ."
echo "  web-radio"
echo ""
