#!/usr/bin/env node
// Compiles every crate under crates/ to WebAssembly with wasm-pack and emits the
// generated JS glue + .wasm into apps/site/src/wasm/<crate>/, where the site's
// island components import it. That output directory is gitignored — it is a
// build artifact, regenerated from the crates.
//
// Usage:
//   node scripts/build-wasm.mjs            # release build (default)
//   node scripts/build-wasm.mjs --dev      # faster, unoptimized build
//
// Requires `wasm-pack` on PATH (installed by .devcontainer/tools/wasm-pack.sh)
// and the wasm32-unknown-unknown target (rust-toolchain.toml pins it).

import { execFileSync } from "node:child_process";
import { readdirSync, existsSync, rmSync } from "node:fs";
import { fileURLToPath } from "node:url";
import path from "node:path";

const repoRoot = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "..",
);
const cratesDir = path.join(repoRoot, "crates");
const outRoot = path.join(repoRoot, "apps", "site", "src", "wasm");

const dev = process.argv.includes("--dev");
const profileFlag = dev ? "--dev" : "--release";

function fail(message) {
  console.error(`build-wasm: ${message}`);
  process.exit(1);
}

// Confirm wasm-pack is available up front so the error is obvious.
try {
  execFileSync("wasm-pack", ["--version"], { stdio: "ignore" });
} catch {
  fail(
    "`wasm-pack` not found on PATH. Install it (see .devcontainer/tools/wasm-pack.sh) " +
      "or run `cargo install wasm-pack`.",
  );
}

if (!existsSync(cratesDir)) fail(`no crates/ directory at ${cratesDir}`);

const crates = readdirSync(cratesDir, { withFileTypes: true })
  .filter(
    (e) =>
      e.isDirectory() && existsSync(path.join(cratesDir, e.name, "Cargo.toml")),
  )
  .map((e) => e.name);

if (crates.length === 0) {
  console.log("build-wasm: no crates found; nothing to build.");
  process.exit(0);
}

for (const name of crates) {
  const crateDir = path.join(cratesDir, name);
  const outDir = path.join(outRoot, name);
  console.log(
    `build-wasm: ${name} (${dev ? "dev" : "release"}) -> src/wasm/${name}/`,
  );

  // Clear stale output so a renamed/removed export can't linger.
  rmSync(outDir, { recursive: true, force: true });

  execFileSync(
    "wasm-pack",
    [
      "build",
      crateDir,
      profileFlag,
      "--target",
      "web",
      "--out-dir",
      outDir,
      "--out-name",
      name,
      // No npm package.json is needed — the site imports the ES module directly.
      "--no-pack",
    ],
    { stdio: "inherit", cwd: repoRoot },
  );
}

console.log(`build-wasm: built ${crates.length} crate(s).`);
