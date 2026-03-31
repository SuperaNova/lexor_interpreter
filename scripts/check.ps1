$ErrorActionPreference = "Stop"

Write-Host "--- 1. Formatting Rust Code ---" -ForegroundColor Cyan
cargo fmt --all

Write-Host "`n--- 2. Running Lints ---" -ForegroundColor Cyan
cargo clippy --workspace
cargo clippy -p lexor_core -- -D warnings

Write-Host "`n--- 3. Running Workspace Tests ---" -ForegroundColor Cyan
cargo test --workspace

Write-Host "`n--- 4. Checking Licenses & Vulnerabilities ---" -ForegroundColor Cyan
cargo deny check

Write-Host "`nAll checks passed successfully! Safe to push to GitHub!" -ForegroundColor Green
