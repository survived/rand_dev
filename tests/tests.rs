use rand_core::RngCore;
use rand_dev::DevRng;

const VAR_NAME: &str = "RUST_TESTS_SEED";

#[test]
fn reproducibility() {
    std::env::remove_var(VAR_NAME);

    let mut rng1 = DevRng::new();
    let mut rng2 = DevRng::new();
    assert_ne!(rng1.get_seed(), rng2.get_seed());

    std::env::set_var(VAR_NAME, hex::encode(rng1.get_seed()));
    let mut rng3 = DevRng::new();
    assert_eq!(rng1.get_seed(), rng3.get_seed());

    let mut randomness1 = [0u8; 50];
    let mut randomness2 = [0u8; 50];
    let mut randomness3 = [0u8; 50];

    rng1.fill_bytes(&mut randomness1);
    rng2.fill_bytes(&mut randomness2);
    rng3.fill_bytes(&mut randomness3);

    assert_eq!(randomness1, randomness3);
    assert_ne!(randomness1, randomness2);
}
