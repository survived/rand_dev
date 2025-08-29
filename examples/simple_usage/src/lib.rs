#[cfg(test)]
mod tests {
    use rand_dev::{DevRng, rand::Rng};

    #[test]
    fn it_works() {
        let mut rng = DevRng::new();
        assert!(rng.random_range(0..=10) < 10);
    }
}
