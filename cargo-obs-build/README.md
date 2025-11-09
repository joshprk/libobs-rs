# cargo-obs-build
This tool is used to build libOBS and its dependencies. It automatically prepares the environment by putting the required libraries in the target directory. This binary automatically selects the correct version of libOBS to download based on the version of `libobs`.
You can also specify a custom version like so:
```toml
# Other stuff

[package.metadata] # Can also be [workspace.metadata]
libobs-version="30.2.2"
libobs-cache-dir="../obs-build" # Optional, defaults to "obs-build", relative to the Cargo.toml file
```

A Github token can be provided via the `GITHUB_TOKEN` environment variable to increase the rate limit when downloading releases from GitHub. This is especially useful for CI environments.