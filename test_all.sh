#!/bin/bash
for m in day??/Cargo.toml; do
    cargo test --release --manifest-path=$m
    [[ $? -eq 0 ]] || exit 1
done
