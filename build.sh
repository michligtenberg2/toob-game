#!/bin/bash
# Build script for Battle of Culiacán RTS

echo "🚀 Building Battle of Culiacán RTS..."
echo "📅 Historical simulation of October 17, 2019"
echo ""

# Ensure Rust environment is loaded
source $HOME/.cargo/env 2>/dev/null || true

# Development build
echo "🔧 Building development version..."
cargo build

if [ $? -eq 0 ]; then
    echo "✅ Development build successful!"
    echo ""
    
    # Release build
    echo "🎯 Building optimized release version..."
    cargo build --release
    
    if [ $? -eq 0 ]; then
        echo "✅ Release build successful!"
        echo ""
        echo "📦 Executable locations:"
        echo "  Development: ./target/debug/culiacan-rts"
        echo "  Release:     ./target/release/culiacan-rts"
        echo ""
        echo "�� To run the game:"
        echo "  cargo run                    (development)"
        echo "  ./target/release/culiacan-rts  (optimized)"
        echo ""
        echo "🎯 Game Controls:"
        echo "  SPACE - Deploy roadblock"
        echo "  R     - Government retreat pressure"  
        echo "  ESC   - End simulation"
    else
        echo "❌ Release build failed"
        exit 1
    fi
else
    echo "❌ Development build failed"
    exit 1
fi
