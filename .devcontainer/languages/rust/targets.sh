#!/usr/bin/env bash
# Adds extra Rust compilation targets.
set -euo pipefail

# The wasm target backs the interactive demos: each crate under crates/ compiles
# to wasm32-unknown-unknown, which wasm-pack then packages for the browser.
"$HOME/.cargo/bin/rustup" target add wasm32-unknown-unknown
