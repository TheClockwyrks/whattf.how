---
title: "Shipping Rust to a static site with WebAssembly"
description: "The mental model: browser wasm is nothing like an embedded Wasmtime runtime."
pubDate: 2026-07-05
tags: ["webassembly", "rust", "astro"]
---

If you've only used WebAssembly through a host runtime like Wasmtime — as an
embedded scripting layer with WASI and host functions you define — putting wasm
on a static site can feel unfamiliar. It's simpler than it sounds.

## The browser is the runtime

On a static site there's no server executing your module. The browser's own
engine is the wasm runtime. You compile a Rust crate to a `.wasm` file, serve it
as a static asset next to your HTML, and the page instantiates it with
`WebAssembly.instantiateStreaming(fetch(...))`. `wasm-pack` + `wasm-bindgen`
generate the JavaScript glue so you never write that by hand.

## The Wasmtime difference

Under Wasmtime your module gets WASI and whatever host functions you register.
In the browser there's no WASI — your wasm reaches the DOM, `<canvas>`, and
`requestAnimationFrame` through generated JS glue. The pattern I like keeps the
Rust surface tiny: compute in Rust, hand a plain buffer to JS, and let JS own
the canvas.

## Where it earns its weight

Wasm is worth its payload for compute-heavy work — physics, fluids,
pathfinding — where Rust's speed shows. For a simple 2D diagram, a plain
JavaScript canvas island is lighter and I'll reach for that instead.
