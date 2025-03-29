# libOBS build tool
This tool is used to build libOBS and its dependencies. It automatically prepares the environment by putting the required libraries in the target directory. The libOBS version should be stored in the Cargo.toml project file like so:
```toml
# Other stuff

[package.metadata] # Can also be [workspace.metadata]
libobs-version="30.2.2"
libobs-cache-dir="../obs-build" # Optional, defaults to "obs-build", relative to the Cargo.toml file
```