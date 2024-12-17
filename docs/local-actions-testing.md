# Testing git-rebrand GitHub Actions

This guide explains how to test the GitHub Actions workflows for `git-rebrand` locally using Act.

## Prerequisites

Ensure you're in the project's Nix development environment:

```bash
# If using direnv (recommended), it will activate automatically
cd git-rebrand

# Or manually enter nix shell
nix develop
```

## Testing Workflows

### CI Workflow

Test the main CI pipeline that runs tests and checks:

```bash
# Standard approach (clean environment, recommended for final verification)
act push -j test

# Development mode (faster, with preserved state)
act push -j test --reuse --preserve-volumes
```

The standard approach starts with a fresh environment each time, ensuring your tests work in the same conditions as the actual CI. Development mode is faster but should only be used during active development.

### Audit Workflow

Test the security audit workflow:

```bash
act push -j audit
```

### Release Workflow

Test the release build process (simulates creating a new version):

```bash
# Test with a sample version
act push --tag v1.0.0 -j build-release
```

Note: The actual release creation and asset upload steps are skipped in local testing.

## Configuration Files

### .actrc

```bash
--container-daemon-socket /var/run/docker.sock
--secret-file .env.ci
--artifact-server-path /tmp/artifacts
--container-architecture linux/amd64
-P ubuntu-latest=rust:latest
--bind
```

### .env.ci (create this file)

```env
# Use dummy tokens for local testing
GITHUB_TOKEN=dummy-token
ACTIONS_RUNTIME_TOKEN=dummy-token
CODECOV_TOKEN=dummy-token
```

## What's Being Tested

- **CI Workflow**: 
  - Rust compilation
  - Code formatting (cargo fmt)
  - Linting (clippy)
  - Unit tests
  - Note: macOS tests (`macos-test` job) will be skipped locally as Act only supports Linux-based containers

- **Audit Workflow**: Security vulnerability scanning
- **Release Workflow**: Build and packaging process for releases

## Troubleshooting

1. **Build Issues**:
   - Ensure you're in the Nix shell
   - Check if Rust toolchain is properly loaded
   - Verify all dependencies are available

2. **Test Failures**:
   - Use verbose output for more details:
     ```bash
     act -v push -j test
     ```
   - Check the test logs in the artifacts directory

3. **Docker Issues**:
   - Ensure Docker daemon is running
   - Check Docker socket permissions
   - If using development mode with preserved volumes, try the standard clean approach first

The actual GitHub operations (creating releases, uploading assets) are skipped in local testing via `if: ${{ !env.ACT }}` conditions.