# whattf.how

A blog about the various projects I'm building — with a bias toward the kind of
posts that teach by _showing_: embedded videos and in-browser, playable
simulations, in the spirit of [Red Blob Games](https://www.redblobgames.com/)
and [Gaffer On Games](https://gafferongames.com/).

The site is a hand-built static site (no off-the-shelf blog framework). It uses
[Astro](https://astro.build/) so that pages ship as static HTML with **zero
JavaScript by default**, and only the interactive widgets ("islands") hydrate —
and only when scrolled into view. Heavy, compute-bound demos (physics, fluids,
pathfinding) are written as Rust crates compiled to **WebAssembly** and embedded
as islands.

## Layout

| Path                                               | What it is                                                                                                                 |
| -------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| [`apps/site/`](apps/site/)                         | The Astro site — content, layouts, components, and the React islands that host demos.                                      |
| [`crates/`](crates/)                               | A Cargo workspace of example crates, each compiled to WebAssembly for embedding on the site.                               |
| [`scripts/build-wasm.mjs`](scripts/build-wasm.mjs) | Builds every crate in `crates/` with `wasm-pack` into the site's `src/wasm/` directory.                                    |
| [`.devcontainer/`](.devcontainer/)                 | The VS Code devcontainer: Node, the Rust toolchain (+ `wasm32` target and `wasm-pack`), and the Cloudflare `wrangler` CLI. |

## Quick start

Inside the devcontainer (see [`.devcontainer/README.md`](.devcontainer/README.md)):

```sh
npm install          # install the JS workspace
npm run dev          # builds the wasm crates, then starts Astro at http://localhost:4321
```

Other commands:

```sh
npm run build        # build the wasm crates, then the static site into apps/site/dist
npm run build:wasm   # (re)build just the wasm crates
npm run preview      # preview the production build locally
cargo test           # test the example crates natively
```

## How a demo works

1. Write a crate in `crates/<name>/` that exposes a small `wasm-bindgen` API
   (e.g. a `Simulation` that steps physics and returns positions).
2. `npm run build:wasm` compiles it to `apps/site/src/wasm/<name>/` (generated,
   gitignored).
3. A React island under `apps/site/src/components/` imports the generated module,
   calls `await init()`, and drives the simulation — drawing to a `<canvas>`.
4. Drop the island into any `.mdx` post with `client:visible` so it only loads
   when the reader reaches it.

See [`crates/demo-bouncing/`](crates/demo-bouncing/) and
[`apps/site/src/components/BouncingBalls.tsx`](apps/site/src/components/BouncingBalls.tsx)
for the reference example, wired into the starter post at
[`apps/site/src/content/blog/hello-world.mdx`](apps/site/src/content/blog/hello-world.mdx).

## Deploying

The site is static and deploys to **Cloudflare Pages** via `wrangler` (the
[`.github/workflows/deploy.yml`](.github/workflows/deploy.yml) workflow). See that
file's header for the one-time secret/variable setup.

## Remotes

`origin` is the Azure DevOps repo (primary); the repo is also mirrored to GitHub
(add it as a `gh` remote), where CI runs.
