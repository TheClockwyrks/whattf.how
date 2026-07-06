#!/usr/bin/env bash
# Installs `wasm-pack`, which compiles the example crates under crates/ to
# WebAssembly and generates the JS glue the site imports (see
# scripts/build-wasm.mjs). Uses the official installer, which drops a prebuilt
# binary into ~/.cargo/bin — much faster than `cargo install wasm-pack`.
#
# Must run AFTER the Rust install script (it installs into the cargo bin dir).
set -euo pipefail

curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

echo "Installed $("$HOME/.cargo/bin/wasm-pack" --version)"
