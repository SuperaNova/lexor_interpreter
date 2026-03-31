#!/bin/bash
set -e # Exit immediately if any command fails

echo -e "\033[1;36m--- 1. Formatting Rust Code ---\033[0m"
cargo fmt --all

echo -e "\n\033[1;36m--- 2. Running Lints ---\033[0m"
cargo clippy --workspace

echo -e "\n\033[1;36m--- 3. Running Workspace Tests ---\033[0m"
cargo test --workspace

echo -e "\n\033[1;36m--- 4. Checking Licenses & Vulnerabilities ---\033[0m"
cargo deny check

echo -e "\n\033[1;32mAll checks passed successfully! Safe to push to GitHub!\033[0m"
