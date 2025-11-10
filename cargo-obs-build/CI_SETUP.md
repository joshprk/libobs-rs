# CI Environment Setup Guide

This guide explains how to properly configure `cargo-obs-build` in CI environments to avoid rate limiting and optimize build times.

## GitHub API Rate Limiting

The `cargo-obs-build` library uses the GitHub API to fetch OBS Studio release information. Without authentication, GitHub limits API requests to 60 per hour per IP address. In CI environments, this can quickly become a problem.

### Automatic API Response Caching

**Good news!** The library automatically caches GitHub API responses in `obs-build/.api-cache/` to minimize API calls. This means:
- Release information is cached after the first fetch
- Subsequent builds use the cached data instead of making new API requests
- Cache is automatically used when available

However, you should still provide a GitHub token for the initial requests and to ensure you don't hit rate limits.

### Solution: Provide a GitHub Token

Set the `GITHUB_TOKEN` environment variable to authenticate API requests, which increases the rate limit to 5,000 requests per hour.

**GitHub Actions:**
```yaml
- name: Build project
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  run: cargo build
```

## Caching the OBS Build Directory

To avoid re-downloading OBS binaries on every CI run, cache the `obs-build` directory. This directory contains:
- Downloaded OBS binaries (in version-specific subdirectories)
- GitHub API response cache (in `.api-cache/`)
- Lock files for preventing concurrent builds

**The library will automatically warn you** in CI environments if:
- The cache directory doesn't exist (indicating caching is not set up)
- `GITHUB_TOKEN` is not set (risking rate limits)

**GitHub Actions:**
```yaml
- name: Cache OBS binaries
  uses: actions/cache@v3
  with:
    path: obs-build
    key: obs-build-${{ runner.os }}-${{ hashFiles('**/Cargo.lock') }}
    restore-keys: |
      obs-build-${{ runner.os }}-

- name: Build project
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  run: cargo build
```

## Workspace Metadata Configuration
You can specify a fixed OBS version in your `Cargo.toml` to reduce API calls:
```toml
[workspace.metadata]
libobs-version = "30.2.2"  # Specific version to use, only set this if you know what you are doing
libobs-cache-dir = "obs-build"  # Cache directory location
```

When you specify a version, the library will:
- Skip checking for newer patch releases
- Make fewer API calls

## Troubleshooting

### Rate Limit Errors

If you see errors like "API rate limit exceeded":
1. Ensure `GITHUB_TOKEN` is set in your CI environment
2. Check that the token has appropriate permissions
3. Verify the token is being passed to the build process

### Cache Not Working

If the OBS binaries are re-downloaded on every CI run:
1. Verify the cache configuration in your CI setup
2. Check that the cache directory path matches your configuration
3. Ensure the cache key is stable between runs

### Build Fails in CI but Works Locally

Common causes:
1. Missing `GITHUB_TOKEN` in CI (rate limiting)
2. Cache directory not configured in CI
3. Network restrictions blocking GitHub API access
4. Different OS in CI vs local (currently only Windows is supported)

## Example Complete CI Configuration

**GitHub Actions (Recommended):**
```yaml
name: Build

on: [push, pull_request]

jobs:
  build:
    runs-on: windows-latest

    steps:
      - uses: actions/checkout@v3

      - name: Cache OBS binaries
        uses: actions/cache@v3
        with:
          path: obs-build
          key: obs-build-${{ runner.os }}-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            obs-build-${{ runner.os }}-

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: cargo build --release
```

## Best Practices

1. **Always set GITHUB_TOKEN in CI** - This prevents rate limiting issues
2. **Always cache the obs-build directory** - This saves time and reduces API calls
3. **Monitor your GitHub API rate limit** - Check [GitHub API rate limit status](https://api.github.com/rate_limit)

## Additional Resources

- [GitHub API Rate Limiting Documentation](https://docs.github.com/en/rest/overview/resources-in-the-rest-api#rate-limiting)
- [GitHub Actions Cache Documentation](https://docs.github.com/en/actions/using-workflows/caching-dependencies-to-speed-up-workflows)
- [cargo-obs-build README](README.md)
