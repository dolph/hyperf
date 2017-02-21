#!/bin/bash
set -ex

DIR=`dirname $(readlink -f $0)`
cd $DIR/..

# Show version information.
rustc --version
cargo --version

# Run clippy from nightly/unstable.
# FIXME: clippy does not build reliably.
# rustup run nightly cargo clippy

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
