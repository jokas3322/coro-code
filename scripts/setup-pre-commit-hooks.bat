@echo off
setlocal enabledelayedexpansion

:: ANSI color codes for Windows 10+ Command Prompt
set "RED=[91m"
set "GREEN=[92m"
set "YELLOW=[93m"
set "BLUE=[94m"
set "CYAN=[96m"
set "NC=[0m"

:: Function to print colored log messages
goto :main

:log_info
echo %CYAN%[INFO]%NC% %~1
goto :eof

:log_success
echo %GREEN%[SUCCESS]%NC% %~1
goto :eof

:log_warning
echo %YELLOW%[WARNING]%NC% %~1
goto :eof

:log_error
echo %RED%[ERROR]%NC% %~1
goto :eof

:main
:: Check if we're in a git repository
if not exist ".git" (
    call :log_error "Not in a git repository. Please run this script from the root of your git repository."
    exit /b 1
)

call :log_info "Setting up pre-commit hooks..."

:: Create .git/hooks directory if it doesn't exist
if not exist ".git\hooks" (
    call :log_info "Creating .git/hooks directory..."
    mkdir ".git\hooks"
)

:: Create the pre-commit hook content
call :log_info "Writing pre-commit hook to .git/hooks/pre-commit..."

(
echo #!/bin/bash
echo.
echo # Colors for output
echo RED='\033[0;31m'
echo GREEN='\033[0;32m'
echo YELLOW='\033[1;33m'
echo BLUE='\033[0;34m'
echo CYAN='\033[0;36m'
echo NC='\033[0m' # No Color
echo.
echo log_info^(^) {
echo     echo -e "${CYAN}[INFO]${NC} $1"
echo }
echo.
echo log_success^(^) {
echo     echo -e "${GREEN}[SUCCESS]${NC} $1"
echo }
echo.
echo log_warning^(^) {
echo     echo -e "${YELLOW}[WARNING]${NC} $1"
echo }
echo.
echo log_error^(^) {
echo     echo -e "${RED}[ERROR]${NC} $1"
echo }
echo.
echo log_info "Running pre-commit checks..."
echo.
echo # Check if cargo is available
echo if ! command -v cargo ^&^> /dev/null; then
echo     log_error "Cargo is not installed or not in PATH"
echo     exit 1
echo fi
echo.
echo # Run cargo fmt check
echo log_info "Checking code formatting with cargo fmt..."
echo if ! cargo fmt --all -- --check; then
echo     log_error "Code formatting check failed. Please run 'cargo fmt' to fix formatting issues."
echo     exit 1
echo fi
echo log_success "Code formatting check passed"
echo.
echo # Run cargo clippy
echo log_info "Running cargo clippy..."
echo if ! cargo clippy --all-targets --all-features -- -D warnings -A dead_code; then
echo     log_error "Clippy check failed. Please fix the warnings and errors."
echo     exit 1
echo fi
echo log_success "Clippy check passed"
echo.
echo # Run tests
echo log_info "Running tests..."
echo if ! cargo test; then
echo     log_error "Tests failed. Please fix the failing tests."
echo     exit 1
echo fi
echo log_success "All tests passed"
echo.
echo log_success "All pre-commit checks passed!"
) > ".git\hooks\pre-commit"

:: Verify the installation
if exist ".git\hooks\pre-commit" (
    call :log_success "Pre-commit hook successfully installed!"
    call :log_info "The hook will run automatically before each commit."
    call :log_info "To test the hook manually, run: .git/hooks/pre-commit"
    call :log_warning "Note: On Windows, you may need Git Bash or WSL to execute the hook properly."
) else (
    call :log_error "Failed to install pre-commit hook"
    exit /b 1
)
