# CLAUDE.md

whattf.how — a hand-built static blog (Astro + Rust→WebAssembly demos). This
file is a map to where things live; prefer reading the pointed-to file over
inferring.

## What this is

A personal blog about the projects the author is building, deliberately _not_
using an off-the-shelf blog framework, so posts can embed interactive,
in-browser simulations (Red Blob Games / Gaffer On Games style). See
[`README.md`](README.md) for the overview.

## Where things live

| Path                     | What it is                                                                                                                                                                                                             |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `apps/site/`             | The Astro site. Blog content is a Content Collection under `apps/site/src/content/blog/` (`.md`/`.mdx`). Layouts in `src/layouts/`, components (including React islands) in `src/components/`, routes in `src/pages/`. |
| `apps/site/src/wasm/`    | **Generated** (gitignored) — `wasm-pack` output imported by the islands. Never edit by hand; produced by `npm run build:wasm`.                                                                                         |
| `crates/`                | Cargo workspace of example crates. Each is a `cdylib` exposing a small `wasm-bindgen` API and is compiled to WebAssembly.                                                                                              |
| `scripts/build-wasm.mjs` | Compiles every crate in `crates/` into `apps/site/src/wasm/<name>/`.                                                                                                                                                   |
| `.devcontainer/`         | Node + Rust (`wasm32-unknown-unknown` target + `wasm-pack`) + `wrangler`. Mirrors The Test Cabinet's modular devcontainer pattern, trimmed.                                                                            |
| `.github/workflows/`     | CI (build) and deploy (Cloudflare Pages). CI runs on GitHub; `origin` is Azure DevOps.                                                                                                                                 |

## Conventions

- **Static output.** The site must remain a static build (`astro build` →
  `apps/site/dist/`). No SSR.
- **Zero JS by default.** Interactivity is opt-in per component via Astro island
  directives (`client:visible` for demos, so they load lazily).
- **Rust for compute-heavy demos only.** Simple 2D diagrams can be plain
  TS/Canvas islands; reach for a wasm crate when the compute (physics, fluids,
  pathfinding) justifies the payload.
- **The wasm boundary stays thin.** Crates expose plain data (e.g. a flat
  `Vec<f32>` of positions) and let the JS island own the DOM/canvas — avoid
  pulling `web-sys` into Rust unless a demo truly needs it.

## Build & run

See [`README.md`](README.md#quick-start). In short: `npm install`, then
`npm run dev` (builds wasm, then serves) or `npm run build` (wasm, then static
site). `cargo test` runs the crates natively.
