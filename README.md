# git-rebrand

[![CI](https://img.shields.io/github/actions/workflow/status/aloshy-ai/git-rebrand/ci.yml?branch=main&label=CI)](https://github.com/aloshy-ai/git-rebrand/actions)
[![Crates.io](https://img.shields.io/crates/v/git-rebrand?label=Crates.io)](https://crates.io/crates/git-rebrand)

A Git extension to rewrite repository history for rebranding purposes. Safely update author information across your Git history while maintaining commit integrity.

## Features

- Pattern-based matching for author names and emails
- Automatic backup branch creation
- Dry-run capability to preview changes
- Support for both exact and partial matches
- Case-insensitive matching
- Comprehensive logging
- Safe history rewriting

## Installation

### Using Homebrew
```bash
brew install yourusername/tap/git-rebrand
```

### Using Cargo
```bash
cargo install git-rebrand
```

### From Source
```bash
git clone https://github.com/yourusername/git-rebrand.git
cd git-rebrand
cargo install --path .
```

## Usage

Basic usage:
```bash
# Show help
git rebrand --help

# Dry run to see what would change
git rebrand --dry-run /path/to/repo

# Perform the rewrite
git rebrand /path/to/repo
```

Using a configuration file:
```bash
git rebrand -c config.yml /path/to/repo
```

Example config.yml:
```yaml
new_author_name: "New Author"
new_author_email: "new@example.com"
patterns:
  - "old@example.com"
  - "Old Author"
  - "@oldcompany.com"
```

## Environment Variables

- `GIT_REBRAND_LOG`: Set log level (trace, debug, info, warn, error)
- `GIT_REBRAND_LOG_STYLE`: Control log output style (auto, always, never)

## Development

### Prerequisites
- Rust 1.70.0 or later
- Git 2.20.0 or later

### Setup
```bash
# Clone the repository
git clone https://github.com/yourusername/git-rebrand.git
cd git-rebrand

# Install development dependencies
./scripts/dev-setup.sh

# Build
cargo build

# Run tests
cargo test
```

### Running Tests
```bash
# Run all tests
cargo test

# Run with coverage
cargo llvm-cov

# Run specific tests
cargo test pattern_matching
```

## Contributing

Contributions are welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
