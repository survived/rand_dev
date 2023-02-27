//! # Reproducible randomness source for tests
//!
//! Having reproducible tests helps debugging problems that have probabilistic nature. This library provides
//! a random numbers generator [`DevRng`] compatible with [`rand`] crate (it implements [`Rng`],
//! [`RngCore`], [`SeedableRng`] traits). When generator is constructed, its seed is printed to stdout.
//! You can override a seed by setting `RUST_TESTS_SEED` env variable. Same seed leads to same randomness
//! generated across all platforms.
//!
//! [`Rng`]: rand::Rng
//! [`RngCore`]: rand::RngCore
//! [`SeedableRng`]: rand::SeedableRng
//!
//! ## Usage
//! Reproducible source of randomness can be added in one line:
//!
//! ```rust,ignore
//! use rand::Rng;
//! use rand_dev::DevRng;
//!
//! #[test]
//! fn it_works() {
//!     let mut rng = DevRng::new();
//!     assert!(rng.gen_range(0..=10) < 10);
//! }
//! ```
//!
//! Then if test fails, you can observe seed of randomness generator in stdout:
//! ```text
//! $ cargo test
//!     Finished test [unoptimized + debuginfo] target(s) in 0.00s
//!      Running unittests (target/debug/deps/simple_usage-592d47155d40d1f7)
//!
//! running 1 test
//! test tests::it_works ... FAILED
//!
//! failures:
//!
//! ---- tests::it_works stdout ----
//! Tests seed: cab4ab5c8471fa03691bb86d96c2febeb9b1099a78d164e8addbe7f83d107c78
//! thread 'tests::it_works' panicked at 'assertion failed: rng.gen_range(0..=10) < 10', src/lib.rs:9:9
//! note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//!
//!
//! failures:
//!     tests::it_works
//!
//! test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//! ```
//!
//! Now you can fix the seed by setting env variable to reproduce and debug a failing test:
//! ```text
//! $ export RUST_TESTS_SEED=cab4ab5c8471fa03691bb86d96c2febeb9b1099a78d164e8addbe7f83d107c78
//! $ cargo test
//! ```

use rand_chacha::ChaCha8Rng;
use rand_core::{CryptoRng, OsRng, RngCore, SeedableRng};

/// Reproducible random generator for tests
#[derive(Debug, Clone)]
pub struct DevRng(ChaCha8Rng);

impl DevRng {
    const VAR_NAME: &'static str = "RUST_TESTS_SEED";

    /// Constructs randomness generator
    ///
    /// Reads a seed from env variable `RUST_TESTS_SEED` or generates a random seed if env variable is not set.
    /// Prints seed to stdout.
    ///
    /// Panics if `RUST_TESTS_SEED` contains invalid value.
    #[track_caller]
    pub fn new() -> Self {
        let mut seed = [0u8; 32];
        match std::env::var(Self::VAR_NAME) {
            Ok(provided_seed) => {
                hex::decode_to_slice(provided_seed, &mut seed).expect("provided seed is not valid")
            }
            Err(std::env::VarError::NotUnicode(_)) => {
                panic!("provided seed is not a valid unicode")
            }
            Err(std::env::VarError::NotPresent) => OsRng.fill_bytes(&mut seed),
        }
        println!("Tests seed: {}", hex::encode(seed));

        DevRng(ChaCha8Rng::from_seed(seed))
    }

    /// Retrieves generator seed
    pub fn get_seed(&self) -> [u8; 32] {
        self.0.get_seed()
    }
}

impl Default for DevRng {
    fn default() -> Self {
        Self::new()
    }
}

impl RngCore for DevRng {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.0.try_fill_bytes(dest)
    }
}

impl SeedableRng for DevRng {
    type Seed = <ChaCha8Rng as SeedableRng>::Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        DevRng(ChaCha8Rng::from_seed(seed))
    }

    fn seed_from_u64(state: u64) -> Self {
        DevRng(ChaCha8Rng::seed_from_u64(state))
    }

    fn from_rng<R: RngCore>(rng: R) -> Result<Self, rand_core::Error> {
        ChaCha8Rng::from_rng(rng).map(DevRng)
    }

    fn from_entropy() -> Self {
        DevRng(ChaCha8Rng::from_entropy())
    }
}

impl CryptoRng for DevRng {}
