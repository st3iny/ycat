# ycat

Concatenate multiple YAML files into a single YAML stream and write it to stdout.

I built this to easily apply Kubernetes manifests in bulk.

### Example

`ycat manifests/*.yaml | kubectl apply -f -`

With a modern shell (e.g. zsh):  
`kubectl apply -f <(ycat manifests/*.yaml)`

Save a bundled manifest for later use:  
`ycat manifest/*.yaml > release.yaml`

## Usage

`ycat FILE [FILE ...]`

The concatenated YAML stream is written to stdout and can be piped to another program or a file.

## Build

Requirements:
- Rust compiler (tested with >= 1.68.2, older versions will probably work too)
- Cargo

Run tests: `cargo test`

Build a debug binary: `cargo build`

Build a release binary: `cargo build --release`

Build and install to `~/.cargo/bin`: `cargo install --path=.`
