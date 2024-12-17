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
# Dry run to see what would happen
act -n push -j test

# Actually run the tests
act push -j test
```

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
-P ubuntu-latest=catthehacker/ubuntu:act-latest
```

### .env.ci (create this file)

```env
GITHUB_TOKEN=dummy-token
```

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

## What's Being Tested

- **CI Workflow**: Rust compilation, tests, and code quality checks
  - Note: macOS tests (`macos-test` job) will be skipped locally as Act only supports Linux-based containers
- **Audit Workflow**: Security vulnerability scanning
- **Release Workflow**: Build and packaging process for releases

The actual GitHub operations (creating releases, uploading assets) are skipped in local testing via `if: ${{ !env.ACT }}` conditions.
