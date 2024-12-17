# Development Guide

This document contains instructions for developing `git-rebrand`, a Git extension for rewriting repository history.

## Prerequisites

- Nix (for development environment)
- Direnv (for automatic environment loading)
- Docker (for testing GitHub Actions locally)

## Development Environment

The project uses Nix for reproducible development environments. All necessary tools (Rust, Cargo, etc.) are automatically provided.

### Setup

```bash
# Clone the repository
git clone https://github.com/aloshy-ai/git-rebrand.git
cd git-rebrand

# The development environment will be automatically loaded if using direnv
# Otherwise, enter the development shell manually:
nix develop
```

## Project Structure

```md
git-rebrand/
├── src/
│   ├── main.rs        # CLI entry point
│   ├── lib.rs         # Core functionality
│   └── logger.rs      # Logging implementation
├── tests/
│   ├── lib.rs         # Integration tests
│   └── common/        # Test utilities
└── docs/             # Documentation
```

## Building

```bash
# Build the project
cargo build

# Build for release
cargo build --release
```

## Testing

### Unit and Integration Tests

```bash
# Run all tests
cargo test

# Run with coverage
cargo llvm-cov

# Run specific tests.
# cargo test [REPLACE-WITH-OLD-AUTHOR-NAME-OR-EMAIL-OR-PATTERN]
cargo test old@email.com
```

### GitHub Actions Testing

See [docs/local-actions-testing.md](docs/local-actions-testing.md) for detailed instructions on testing GitHub Actions workflows locally.

## Code Quality

### Formatting and Linting

```bash
# Format code
cargo fmt

# Run clippy for linting
cargo clippy -- -D warnings
```

### Security Audit

```bash
# Run security audit
cargo audit
```

## Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md following the Keep a Changelog format
3. Create and push a new tag:

```bash
git tag -a v0.1.x -m "Release v0.1.x"
git push origin v0.1.x
```

The release workflow will automatically:

- Create a GitHub release
- Build binaries for Linux, macOS, and Windows
- Attach binaries to the release

## Environment Variables

- `GIT_REBRAND_LOG`: Set log level (trace, debug, info, warn, error)
- `GIT_REBRAND_LOG_STYLE`: Control log output style (auto, always, never)

## Documentation

- Update API documentation in source code using rustdoc comments
- Keep README.md updated with new features and changes
- Document breaking changes in CHANGELOG.md
- Add examples for new functionality

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed contribution guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
