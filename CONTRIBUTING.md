# Contributing to git-rebrand

First off, thank you for considering contributing to git-rebrand! It's people like you that make git-rebrand such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs
Before creating bug reports, please:
- Check the [existing issues](https://github.com/yourusername/git-rebrand/issues)
- Check if the issue still exists with the latest version
- Collect information about your environment (OS, Rust version, Git version)

When reporting bugs, please include:
- Exact steps to reproduce
- Expected vs actual behavior
- Version information (`git rebrand --version`)
- Error messages and logs (`GIT_REBRAND_LOG=debug`)

### Suggesting Enhancements
Enhancement suggestions are tracked as GitHub issues. Please include:
- Step-by-step description of the enhancement
- Specific examples to demonstrate the steps
- Expected outcome
- Why this enhancement would be useful

### Pull Requests

1. Fork the repo
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run the test suite (`cargo test`)
5. Run clippy (`cargo clippy`)
6. Format your code (`cargo fmt`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Create a Pull Request

## Development Environment Setup

1. Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repository:
```bash
git clone https://github.com/yourusername/git-rebrand.git
cd git-rebrand
```

3. Install development dependencies:
```bash
./scripts/dev-setup.sh
```

## Project Structure
```
git-rebrand/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs           # Core functionality
│   └── logger.rs        # Logging implementation
├── tests/
│   ├── lib.rs           # Integration tests
│   └── common/          # Test utilities
└── scripts/             # Development scripts
```

## Testing
```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin

# Run specific test
cargo test test_pattern_matching
```

## Style Guide

- Follow Rust standard formatting (`cargo fmt`)
- Use clippy to catch common mistakes (`cargo clippy`)
- Write descriptive commit messages
- Document public APIs
- Include tests for new functionality

## Documentation

- Use rustdoc comments for public APIs
- Update README.md for new features
- Update CHANGELOG.md
- Include examples in documentation

## Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create a tag:
```bash
git tag -a v0.1.x -m "Release v0.1.x"
git push origin v0.1.x
```

## Questions?

Feel free to:
- Open an issue
- Start a discussion
- Join our community chat

We'll be happy to help!
