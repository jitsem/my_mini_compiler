#!/bin/bash

# Define paths and file names
INPUT_FILE="data/$1.scrpt"
GCC_OUTPUT="target/debug/a.out"

# Ensure the target directory exists
mkdir -p target/debug

# Run the Rust program with specified input file and target, then pipe its output to gcc
cargo run -- -t C "$INPUT_FILE" | gcc -x c - -o "$GCC_OUTPUT"

# Check if gcc compilation was successful
if [ $? -eq 0 ]; then
    echo "Compilation successful. Running the binary..."
    ./"$GCC_OUTPUT"
else
    echo "Compilation failed."
fi
