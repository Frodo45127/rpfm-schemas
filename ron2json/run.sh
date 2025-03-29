#!/bin/bash

for file in schema_*.ron; do
    filename=$(basename "$file" .ron)
    echo "Converting $file to $filename.json"
    cargo run --release --manifest-path ./ron2json/Cargo.toml ${file}
    mv output.json ${filename}.json
    rm $file
done
