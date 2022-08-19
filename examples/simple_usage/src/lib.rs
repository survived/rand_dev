#[cfg(test)]
mod tests {
    use rand::Rng;
    use rand_dev::DevRng;

    #[test]
    fn it_works() {
        let mut rng = DevRng::new();
        assert!(rng.gen_range(0..=10) < 10);
    }
}
