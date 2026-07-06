#!/usr/bin/env bash
# Installs `wasm-pack`, which compiles the example crates under crates/ to
# WebAssembly and generates the JS glue the site imports (see
# scripts/build-wasm.mjs).
#
# We download a pinned prebuilt binary (statically linked against musl, so it
# runs on any Linux) rather than using the official rustwasm installer, which is
# stale and installs an old version. Must run AFTER the Rust install script — it
# installs into the cargo bin dir, which is on PATH.
set -euo pipefail

: "${WASM_PACK_VERSION:?WASM_PACK_VERSION must be set}"

# wasm-pack publishes musl Linux archives for x86_64 and aarch64; map the Debian
# architecture name onto the target triple it uses in its asset file names.
case "$(dpkg --print-architecture)" in
	amd64) readonly WP_TARGET="x86_64-unknown-linux-musl" ;;
	arm64) readonly WP_TARGET="aarch64-unknown-linux-musl" ;;
	*)
		echo "Unsupported architecture: $(dpkg --print-architecture)" >&2
		exit 1
		;;
esac

readonly ARCHIVE_NAME="wasm-pack-v${WASM_PACK_VERSION}-${WP_TARGET}.tar.gz"
readonly DOWNLOAD_URL="https://github.com/rustwasm/wasm-pack/releases/download/v${WASM_PACK_VERSION}/${ARCHIVE_NAME}"
readonly TAR_PATH="/tmp/$USERNAME/wasm-pack.tar.gz"
readonly BIN_PATH="$HOME/.cargo/bin/wasm-pack"

mkdir -p "/tmp/$USERNAME" "$HOME/.cargo/bin"
curl -fsSL -o "$TAR_PATH" "$DOWNLOAD_URL"
# The archive contains a single `wasm-pack-v<version>-<target>/` directory with
# the `wasm-pack` binary inside; strip it and extract just that binary.
tar -xzf "$TAR_PATH" -C "$HOME/.cargo/bin" --strip-components=1 \
	"wasm-pack-v${WASM_PACK_VERSION}-${WP_TARGET}/wasm-pack"
chmod +x "$BIN_PATH"
rm -f "$TAR_PATH"

echo "Installed $("$BIN_PATH" --version)"
