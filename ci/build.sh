#!/bin/bash
set -ex

DIR=`dirname $(readlink -f $0)`
cd $DIR/..

# Show version information.
rustc --version
cargo --version

# Run linter.
# TODO(dolph): Clippy is unstable and doesn't actually build. Re-enable it when
# it's stable: https://github.com/Manishearth/rust-clippy
# cargo install clippy
# cargo clippy

# Build and deny warnings.
cargo rustc -- -D warnings

# Test the project.
cargo test --verbose

# Smoke test the result.
export RUST_LOG=debug
cargo run http://example.com/
./target/debug/hyperf --help
./target/debug/hyperf --version
./target/debug/hyperf http://example.com/
./target/debug/hyperf --verbose http://example.com/
./target/debug/hyperf --requests 10 http://example.com/
./target/debug/hyperf -n 10 http://example.com/
./target/debug/hyperf -n 10 --concurrency 10 http://example.com/
./target/debug/hyperf -n 10 -c 10 http://example.com/
