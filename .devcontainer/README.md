# Devcontainer

A VS Code devcontainer for developing whattf.how. It provides Node.js, the Rust
toolchain (with `rustfmt`, `clippy`, and the `wasm32-unknown-unknown` target),
`wasm-pack` for compiling the example crates to WebAssembly, the Cloudflare
`wrangler` CLI for deploys, and `gh` + `lazygit` for git.

It follows the same modular structure as
[The Test Cabinet's devcontainer](https://github.com/TheClockwyrks/TheTestCabinet),
trimmed to what a static-site-plus-wasm repo needs (no Docker-in-Docker, no
Kubernetes tooling, no Tauri system libraries).

## Opening it

Open the repository folder in VS Code and choose **Reopen in Container**. No
first-time file copying is required — the defaults suit a standard Docker host
with UID/GID 1000.

Once inside:

```sh
npm install
npm run dev      # builds the wasm crates, then serves at http://localhost:4321
```

Port 4321 (Astro's dev server) is forwarded automatically.

## Hosts that need a different GID (e.g. rootless Podman)

The container user defaults to UID/GID `1000:1000`. On a rootless Podman host the
primary group is commonly GID `100` (`users`). Create a `.devcontainer/.env`
(gitignored) to override before building:

```sh
DEVCONTAINER_GID=100
```

## SSH / git

VS Code forwards your host SSH agent into the container automatically, so
`git push` to the Azure `origin` and the GitHub `gh` remote works with your host
keys. Authenticate `gh` with `gh auth login` and `wrangler` with
`wrangler login` (or environment tokens) as needed — no credentials are baked
into the image.

## What's where

| Path                               | Purpose                                                                                                                         |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `whattf.dockerfile`                | Image definition; runs the scripts below in order.                                                                              |
| `docker-compose.yml`               | The `dev` service, plus the `cargo-target` named volume that keeps Rust build output off the host-mounted (virtiofs) workspace. |
| `system/`                          | Base OS: apt packages, locale, and container-user creation.                                                                     |
| `languages/node`, `languages/rust` | Language toolchains. `rust/targets.sh` adds the wasm target.                                                                    |
| `tools/`                           | `gh`, `lazygit`, `wrangler`, and `wasm-pack` installers.                                                                        |
| `ai/claude.sh`                     | Installs the Claude Code CLI.                                                                                                   |
| `post-install.sh`                  | Installs the managed `~/.bashrc` / `~/.tmux.conf` blocks.                                                                       |
