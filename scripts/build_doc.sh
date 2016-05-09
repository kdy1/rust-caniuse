#!/bin/sh -e
cd caniuse
cargo doc --no-deps -p caniuse -p caniuse_shared
# Move to $root/target, so deploy_docs.sh can recognize this.
mkdir -p "../target"
mv "./target/doc" "../target/doc"
