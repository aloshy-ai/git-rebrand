# git-rebrand

[![CI](https://img.shields.io/github/actions/workflow/status/aloshy-ai/git-rebrand/ci.yml?branch=main&label=CI)](https://github.com/aloshy-ai/git-rebrand/actions)
[![Coverage](https://codecov.io/gh/aloshy-ai/git-rebrand/branch/main/graph/badge.svg)](https://codecov.io/gh/aloshy-ai/git-rebrand)
[![macOS Support](https://img.shields.io/badge/macOS-Intel%20%7C%20ARM-brightgreen?logo=apple)](https://github.com/aloshy-ai/git-rebrand/releases)
[![License](https://img.shields.io/github/license/aloshy-ai/git-rebrand?label=License)](https://github.com/aloshy-ai/git-rebrand/blob/main/LICENSE)
[![Release](https://img.shields.io/github/v/release/aloshy-ai/git-rebrand?label=Release)](https://github.com/aloshy-ai/git-rebrand/releases/tag/main)

A Git extension to safely rewrite repository history for rebranding purposes. Update author information across your Git history while maintaining commit integrity and timestamps.

## Features

- **Flexible Pattern Matching**:
  - Match by exact email address
  - Match by email domain
  - Match by full author name
  - Match by partial name
  - Case-insensitive matching

- **Safety Features**:
  - Automatic backup branch creation (can be disabled)
  - Dry-run mode to preview changes
  - Validation of repository state
  - Protection against uncommitted changes
  - Email format validation

- **Configuration Options**:
  - YAML configuration file support
  - Interactive configuration mode
  - Multiple pattern support per run

- **Comprehensive Logging**:
  - Configurable log levels
  - Detailed operation tracking
  - Timestamp precision in logs

## Installation

### Using Homebrew

```bash
brew install aloshy-ai/tap/git-rebrand
```

### Using Cargo

```bash
cargo install git-rebrand
```

### From Source
```bash
git clone https://github.com/aloshy-ai/git-rebrand.git
cd git-rebrand
cargo install --path .
```

## Usage

### Basic Commands

```bash
# Show help and available options
git rebrand --help

# Perform a dry run to preview changes
git rebrand --dry-run /path/to/repo

# Run with verbose logging
git rebrand -v /path/to/repo

# Skip backup branch creation (use with caution)
git rebrand --no-backup /path/to/repo

# Use a configuration file
git rebrand -c config.yml /path/to/repo
```

### Configuration File

Example `config.yml`:

```yaml
# New author information
new_author_name: "New Author"
new_author_email: "new@example.com"

# Patterns to match (can include multiple)
patterns:
  - "old@example.com"          # Exact email match
  - "@oldcompany.com"         # Domain match
  - "Old Author"              # Full name match
  - "John"                    # Partial name match
```

### Environment Variables

- `GIT_REBRAND_LOG`: Set log level (trace, debug, info, warn, error)
- `GIT_REBRAND_LOG_STYLE`: Control log output style (auto, always, never)

### Safety Features

1. **Backup Creation**:
   - Automatically creates a backup branch before rewriting
   - Backup branches are timestamped (e.g., `backup_20240321123456`)
   - Can be disabled with `--no-backup` flag

2. **Validation Checks**:
   - Verifies repository isn't empty
   - Checks for uncommitted changes
   - Validates email formats
   - Confirms pattern matches exist

3. **Dry Run Mode**:
   - Shows affected commits without making changes
   - Displays matched patterns
   - Previews new author information

## Development

### Prerequisites

- Rust toolchain
- Nix (for development environment)
- Direnv (recommended)

### Setup

```bash
# Clone the repository
git clone https://github.com/aloshy-ai/git-rebrand.git
cd git-rebrand

# Install dependencies and build project
cargo fetch
cargo build

# Run tests
cargo test
cargo llvm-cov  # For coverage report
```

### Local Actions Testing

For testing GitHub Actions locally, see [Local Actions Testing Guide](docs/local-actions-testing.md).

## Contributing

Contributions are welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
