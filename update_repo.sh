#!/bin/bash
set -e

SRC_DIR="../gd32-rs/gd32f4/src"
DEST_DIR="./src"

# Remove the destination src directory if it exists
if [ -d "$DEST_DIR" ]; then
    rm -rf "$DEST_DIR"
fi

# Copy the source directory to the destination
cp -r "$SRC_DIR" "$DEST_DIR"