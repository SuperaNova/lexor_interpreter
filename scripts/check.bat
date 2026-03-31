@echo off
setlocal

echo --- 1. Formatting Rust Code ---
cargo fmt --all
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%

echo.
echo --- 2. Running Lints ---
cargo clippy --workspace
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%

echo.
echo --- 3. Running Workspace Tests ---
cargo test --workspace
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%

echo.
echo --- 4. Checking Licenses ^& Vulnerabilities ---
cargo deny check
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%

echo.
echo All checks passed successfully! Safe to push to GitHub!
