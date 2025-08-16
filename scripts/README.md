# Scripts

This directory contains various utility scripts for the project.

## Pre-commit Hooks Setup

The following scripts automatically install pre-commit hooks that run code quality checks before each commit:

### Available Scripts

- **`setup-pre-commit-hooks.sh`** - For Linux/macOS (Bash)
- **`setup-pre-commit-hooks.ps1`** - For Windows (PowerShell)
- **`setup-pre-commit-hooks.bat`** - For Windows (Command Prompt)

### Usage

Choose the appropriate script for your platform:

#### Linux/macOS

```bash
chmod +x scripts/setup-pre-commit-hooks.sh
./scripts/setup-pre-commit-hooks.sh
```

#### Windows PowerShell

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
.\scripts\setup-pre-commit-hooks.ps1
```

#### Windows Command Prompt

```cmd
scripts\setup-pre-commit-hooks.bat
```

### What the Pre-commit Hook Does

The installed hook will automatically run the following checks before each commit:

1. **Code Formatting** - Runs `cargo fmt --check` to ensure code is properly formatted
2. **Linting** - Runs `cargo clippy --all-targets --all-features -- -D warnings -A dead_code` to catch common mistakes and improve code quality (allows dead code)
3. **Tests** - Runs `cargo test` to ensure all tests pass

If any of these checks fail, the commit will be blocked until the issues are resolved.

### Features

- **Colored Output** - Uses ANSI colors to distinguish different types of log messages:
  - `[INFO]` - Cyan
  - `[SUCCESS]` - Green
  - `[WARNING]` - Yellow
  - `[ERROR]` - Red
- **Cross-platform** - Works on Linux, macOS, and Windows
- **Automatic Installation** - Creates `.git/hooks/pre-commit` with proper permissions

### Manual Testing

To test the pre-commit hook manually without making a commit:

```bash
.git/hooks/pre-commit
```

### Troubleshooting

- **Windows Users**: The hook script is written in Bash, so you'll need Git Bash, WSL, or similar to execute it properly
- **Permission Issues**: On Unix-like systems, ensure the script has execute permissions
- **Cargo Not Found**: Make sure Rust and Cargo are installed and in your PATH
