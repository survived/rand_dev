#![doc = include_str!("../README.md")]

#[cfg(feature = "rand-v09")]
pub use rand;

use rand_chacha::ChaCha8Rng;
use rand_core::{CryptoRng, OsRng, RngCore, SeedableRng, TryRngCore};

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
            Ok(provided_seed) => const_hex::decode_to_slice(provided_seed, &mut seed)
                .expect("provided seed is not valid"),
            Err(std::env::VarError::NotUnicode(_)) => {
                panic!("provided seed is not a valid unicode")
            }
            Err(std::env::VarError::NotPresent) => OsRng
                .try_fill_bytes(&mut seed)
                .expect("system randomness unavailable"),
        }
        println!("RUST_TESTS_SEED={}", const_hex::encode(seed));

        DevRng(ChaCha8Rng::from_seed(seed))
    }

    /// Derives another randomness generator from this instance
    ///
    /// Uses `self` to generate a seed and constructs a new instance of `DevRng` from the seed.
    ///
    /// May be useful when you have several threads/futures/places where you need access the
    /// randomness generation, but you don't want to mess with ownership system.
    pub fn fork(&mut self) -> Self {
        let mut seed = [0u8; 32];
        self.fill_bytes(&mut seed);
        Self::from_seed(seed)
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
}

impl SeedableRng for DevRng {
    type Seed = <ChaCha8Rng as SeedableRng>::Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        DevRng(ChaCha8Rng::from_seed(seed))
    }

    fn seed_from_u64(state: u64) -> Self {
        DevRng(ChaCha8Rng::seed_from_u64(state))
    }

    fn from_rng(rng: &mut impl RngCore) -> Self {
        Self(ChaCha8Rng::from_rng(rng))
    }

    fn try_from_rng<R: TryRngCore>(rng: &mut R) -> Result<Self, R::Error> {
        ChaCha8Rng::try_from_rng(rng).map(Self)
    }

    fn from_os_rng() -> Self {
        Self(ChaCha8Rng::from_os_rng())
    }

    fn try_from_os_rng() -> Result<Self, getrandom::Error> {
        ChaCha8Rng::try_from_os_rng().map(Self)
    }
}

impl CryptoRng for DevRng {}
