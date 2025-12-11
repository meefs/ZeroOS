#![no_std]

use foundation::ops::RandomOps;

#[cfg(feature = "lcg")]
mod lcg;

#[cfg(feature = "chacha")]
mod chacha;

#[cfg(test)]
mod tests;

#[cfg(feature = "lcg")]
pub const LCG_RNG_OPS: RandomOps = RandomOps {
    fill_bytes: lcg::fill_bytes,
    init_seed: lcg::init_seed,
};

#[cfg(feature = "chacha")]
pub const CHACHA_RNG_OPS: RandomOps = RandomOps {
    fill_bytes: chacha::fill_bytes,
    init_seed: chacha::init_seed,
};

#[cfg(all(feature = "lcg", not(feature = "chacha")))]
pub use lcg::{fill_bytes, init_seed};

#[cfg(feature = "chacha")]
pub use chacha::{fill_bytes, init_seed};
