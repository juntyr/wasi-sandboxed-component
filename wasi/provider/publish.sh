#!/usr/bin/env bash
set -ex

cp wasi/provider/Cargo.toml.in wasi/provider/Cargo.toml
sed -i '' 's@# "wasi/provider",@"wasi/provider",@g' Cargo.toml

WASI_SANDBOXED_COMPONENT_PROVIDER="$(pwd)/wasi/provider/artefacts" cargo build -p wasi-sandboxed-component-builder

cargo publish -p wasi-sandboxed-component-provider

rm wasi/provider/artefacts/*.wasm

rm wasi/provider/Cargo.toml
sed -i '' 's@"wasi/provider",@# "wasi/provider",@g' Cargo.toml
