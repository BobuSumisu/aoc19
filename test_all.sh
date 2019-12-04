#!/bin/bash
for m in day??/Cargo.toml; do cargo test --manifest-path=$m; done
