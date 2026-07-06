//! A tiny bouncing-balls physics demo, compiled to WebAssembly and driven from a
//! React island on the site.
//!
//! The wasm boundary is deliberately thin: this crate owns the simulation state
//! and, each frame, hands JavaScript a flat `[x, y, r, x, y, r, ...]` buffer of
//! ball positions. The island reads that buffer and draws to a `<canvas>` — so
//! no `web-sys`/DOM dependency is needed here, and the crate stays testable
//! natively.
//!
//! Randomness uses a small hand-rolled xorshift PRNG rather than the `rand`
//! crate, which keeps the dependency graph to just `wasm-bindgen` and sidesteps
//! `getrandom`'s wasm configuration.

use wasm_bindgen::prelude::*;

/// A minimal, deterministic xorshift32 PRNG. Good enough for scattering demo
/// balls; not for anything cryptographic.
struct Rng(u32);

impl Rng {
    fn new(seed: u32) -> Self {
        // Avoid the zero state, which xorshift cannot escape.
        Rng(seed | 1)
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    /// A float in `[0, 1)`.
    fn next_f32(&mut self) -> f32 {
        (self.next_u32() >> 8) as f32 / (1u32 << 24) as f32
    }

    /// A float in `[lo, hi)`.
    fn range(&mut self, lo: f32, hi: f32) -> f32 {
        lo + (hi - lo) * self.next_f32()
    }
}

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    r: f32,
}

/// The simulation state. Construct with a ball count and canvas dimensions, then
/// call [`Simulation::tick`] once per animation frame and read
/// [`Simulation::positions`] to draw.
#[wasm_bindgen]
pub struct Simulation {
    balls: Vec<Ball>,
    width: f32,
    height: f32,
    gravity: f32,
    restitution: f32,
    // Reused each frame so `positions()` does not allocate after construction.
    scratch: Vec<f32>,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(count: usize, width: f32, height: f32, seed: u32) -> Simulation {
        let mut rng = Rng::new(seed);
        let balls = (0..count)
            .map(|_| {
                let r = rng.range(6.0, 18.0);
                Ball {
                    x: rng.range(r, width - r),
                    y: rng.range(r, height * 0.5),
                    vx: rng.range(-120.0, 120.0),
                    vy: rng.range(-60.0, 60.0),
                    r,
                }
            })
            .collect();
        Simulation {
            balls,
            width,
            height,
            gravity: 900.0,
            restitution: 0.82,
            scratch: vec![0.0; count * 3],
        }
    }

    /// Update the canvas size (e.g. after a resize) so balls bounce off the new
    /// walls.
    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    /// Advance the simulation by `dt` seconds.
    pub fn tick(&mut self, dt: f32) {
        // Clamp dt so a backgrounded tab that resumes with a huge delta doesn't
        // tunnel balls straight through the walls.
        let dt = dt.min(1.0 / 30.0);
        for b in &mut self.balls {
            b.vy += self.gravity * dt;
            b.x += b.vx * dt;
            b.y += b.vy * dt;

            if b.x - b.r < 0.0 {
                b.x = b.r;
                b.vx = -b.vx * self.restitution;
            } else if b.x + b.r > self.width {
                b.x = self.width - b.r;
                b.vx = -b.vx * self.restitution;
            }
            if b.y - b.r < 0.0 {
                b.y = b.r;
                b.vy = -b.vy * self.restitution;
            } else if b.y + b.r > self.height {
                b.y = self.height - b.r;
                b.vy = -b.vy * self.restitution;
            }
        }
    }

    /// A flat buffer of `[x, y, r]` triples — three floats per ball — for the
    /// island to draw. Backed by a reused allocation.
    pub fn positions(&mut self) -> Vec<f32> {
        for (i, b) in self.balls.iter().enumerate() {
            self.scratch[i * 3] = b.x;
            self.scratch[i * 3 + 1] = b.y;
            self.scratch[i * 3 + 2] = b.r;
        }
        self.scratch.clone()
    }

    /// The number of balls in the simulation.
    #[wasm_bindgen(getter)]
    pub fn count(&self) -> usize {
        self.balls.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balls_stay_within_bounds() {
        let mut sim = Simulation::new(50, 640.0, 360.0, 12345);
        for _ in 0..600 {
            sim.tick(1.0 / 60.0);
        }
        let pos = sim.positions();
        assert_eq!(pos.len(), 50 * 3);
        for chunk in pos.chunks(3) {
            let (x, y, r) = (chunk[0], chunk[1], chunk[2]);
            assert!(
                x >= r - 0.5 && x <= 640.0 - r + 0.5,
                "x={x} r={r} out of bounds"
            );
            assert!(
                y >= r - 0.5 && y <= 360.0 - r + 0.5,
                "y={y} r={r} out of bounds"
            );
        }
    }

    #[test]
    fn rng_is_deterministic() {
        let a = Simulation::new(10, 100.0, 100.0, 999).positions();
        let b = Simulation::new(10, 100.0, 100.0, 999).positions();
        assert_eq!(a, b);
    }
}
