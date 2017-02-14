#!/bin/bash
set -ex

DIR=`dirname $(readlink -f $0)`
cd $DIR/..

# Show version information.
rustc --version
cargo --version

# Build for release.
cargo build --release --verbose

mkdir release
echo hyperf v`./target/release/hyperf --version` > release/name
echo `./target/release/hyperf --version` > release/tag
cp target/release/hyperf release/artifact
