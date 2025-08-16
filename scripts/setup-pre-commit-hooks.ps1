# PowerShell script to setup pre-commit hooks

# Function to print colored log messages
function Write-LogInfo {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Cyan
}

function Write-LogSuccess {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-LogWarning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-LogError {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

# Check if we're in a git repository
if (-not (Test-Path ".git")) {
    Write-LogError "Not in a git repository. Please run this script from the root of your git repository."
    exit 1
}

Write-LogInfo "Setting up pre-commit hooks..."

# Create .git/hooks directory if it doesn't exist
if (-not (Test-Path ".git/hooks")) {
    Write-LogInfo "Creating .git/hooks directory..."
    New-Item -ItemType Directory -Path ".git/hooks" -Force | Out-Null
}

# Pre-commit hook content (PowerShell version for Windows)
$PreCommitHookContent = @'
#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${CYAN}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_info "Running pre-commit checks..."

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    log_error "Cargo is not installed or not in PATH"
    exit 1
fi

# Run cargo fmt check
log_info "Checking code formatting with cargo fmt..."
if ! cargo fmt --all -- --check; then
    log_error "Code formatting check failed. Please run 'cargo fmt' to fix formatting issues."
    exit 1
fi
log_success "Code formatting check passed"

# Run cargo clippy
log_info "Running cargo clippy..."
if ! cargo clippy --all-targets --all-features -- -D warnings -A dead_code; then
    log_error "Clippy check failed. Please fix the warnings and errors."
    exit 1
fi
log_success "Clippy check passed"

# Run tests
log_info "Running tests..."
if ! cargo test; then
    log_error "Tests failed. Please fix the failing tests."
    exit 1
fi
log_success "All tests passed"

log_success "All pre-commit checks passed!"
'@

# Write the pre-commit hook
Write-LogInfo "Writing pre-commit hook to .git/hooks/pre-commit..."
$PreCommitHookContent | Out-File -FilePath ".git/hooks/pre-commit" -Encoding UTF8 -NoNewline

# Verify the installation
if (Test-Path ".git/hooks/pre-commit") {
    Write-LogSuccess "Pre-commit hook successfully installed!"
    Write-LogInfo "The hook will run automatically before each commit."
    Write-LogInfo "To test the hook manually, run: .git/hooks/pre-commit"
    Write-LogWarning "Note: On Windows, you may need Git Bash or WSL to execute the hook properly."
} else {
    Write-LogError "Failed to install pre-commit hook"
    exit 1
}
