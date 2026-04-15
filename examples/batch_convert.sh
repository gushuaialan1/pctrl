#!/usr/bin/env bash
set -euo pipefail

# batch_convert.sh
# Batch-converts all .txt files in an input directory to JSON pinyin output.

INPUT_DIR="${1:-texts}"
OUTPUT_DIR="${2:-output}"

if [ ! -d "$INPUT_DIR" ]; then
    echo "Error: input directory '$INPUT_DIR' does not exist."
    echo "Usage: $0 [input_dir] [output_dir]"
    exit 1
fi

mkdir -p "$OUTPUT_DIR"

files=("$INPUT_DIR"/*.txt)
total=${#files[@]}
count=0

for f in "${files[@]}"; do
    [ -e "$f" ] || { echo "No .txt files found in $INPUT_DIR"; exit 1; }
    basename=$(basename "$f" .txt)
    pctrl convert --file "$f" --format json > "$OUTPUT_DIR/${basename}.json"
    count=$((count + 1))
    printf '\rProgress: %d/%d (%s)' "$count" "$total" "$basename"
done

printf '\nDone. Output written to %s\n' "$OUTPUT_DIR"
