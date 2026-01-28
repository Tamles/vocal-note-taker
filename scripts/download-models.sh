#!/bin/bash
# Download Whisper models for vocal-note-taker
#
# This script downloads the whisper.cpp compatible model files.
# Models are stored in ~/.local/share/vocal-note-taker/models/
#
# Usage: ./scripts/download-models.sh

set -e

# Configuration
MODEL_NAME="ggml-large-v3.bin"
MODEL_URL="https://huggingface.co/ggerganov/whisper.cpp/resolve/main/${MODEL_NAME}"
MODEL_SIZE="~3GB"

# Determine model directory based on OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    MODEL_DIR="$HOME/Library/Application Support/vocal-note-taker/models"
else
    MODEL_DIR="$HOME/.local/share/vocal-note-taker/models"
fi

MODEL_PATH="$MODEL_DIR/$MODEL_NAME"

echo "==================================="
echo "vocal-note-taker Model Downloader"
echo "==================================="
echo ""
echo "Model: $MODEL_NAME ($MODEL_SIZE)"
echo "Target: $MODEL_PATH"
echo ""

# Check if model already exists
if [[ -f "$MODEL_PATH" ]]; then
    echo "Model already exists at $MODEL_PATH"
    echo "Delete it manually if you want to re-download."
    exit 0
fi

# Create directory
echo "Creating directory: $MODEL_DIR"
mkdir -p "$MODEL_DIR"

# Check for wget or curl
if command -v wget &> /dev/null; then
    echo "Downloading with wget..."
    wget -O "$MODEL_PATH" "$MODEL_URL" --show-progress
elif command -v curl &> /dev/null; then
    echo "Downloading with curl..."
    curl -L -o "$MODEL_PATH" "$MODEL_URL" --progress-bar
else
    echo "Error: Neither wget nor curl found. Please install one of them."
    exit 1
fi

# Verify download
if [[ -f "$MODEL_PATH" ]]; then
    SIZE=$(du -h "$MODEL_PATH" | cut -f1)
    echo ""
    echo "==================================="
    echo "Download complete!"
    echo "Model: $MODEL_PATH"
    echo "Size: $SIZE"
    echo "==================================="
    echo ""
    echo "You can now start vocal-note-taker."
else
    echo "Error: Download failed. Model file not found."
    exit 1
fi
