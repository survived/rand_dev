![License](https://img.shields.io/crates/l/rand_dev)
[![Docs](https://docs.rs/rand_dev/badge.svg)](https://docs.rs/rand_dev)
[![Crates io](https://img.shields.io/crates/v/rand_dev.svg)](https://crates.io/crates/rand_dev)

# Reproducible randomness source for tests

Having reproducible tests helps debugging problems that have probabilistic nature. This library provides
a random numbers generator `DevRng` compatible with [`rand`] crate (it implements `Rng`, `RngCore`, 
`SeedableRng` traits). When generator is constructed, its seed is printed to stdout. You can override a 
seed by setting `RUST_TESTS_SEED` env variable. Same seed leads to same randomness generated across all 
platforms.

## Usage
Reproducible source of randomness can be added in one line:

```rust,no_run
use rand::Rng;
use rand_dev::DevRng;

#[test]
fn it_works() {
    let mut rng = DevRng::new();
    assert!(rng.random_range(0..=10) < 10);
}
```

Then if test fails, you can observe seed of randomness generator in stdout:
```text
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/simple_usage-592d47155d40d1f7)

running 1 test
test tests::it_works ... FAILED

failures:

---- tests::it_works stdout ----
RUST_TESTS_SEED=fa48105a3c2ada139e0aa234f235a7af5c766cac4daefca97b57d73915c5b736
thread 'tests::it_works' panicked at 'assertion failed: rng.gen_range(0..=10) < 10', src/lib.rs:9:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_works

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Now you can fix the seed by setting env variable to reproduce and debug a failing test:
```text
$ export RUST_TESTS_SEED=cab4ab5c8471fa03691bb86d96c2febeb9b1099a78d164e8addbe7f83d107c78
$ cargo test
```

[`rand`]: https://docs.rs/rand

## Features
* `rand-v09` _(disabled by default)_ when enabled, library re-exports `rand v0.9` which will be accessible as `rand_dev::rand`.

## License

Licensed under either of

 * Apache License, Version 2.0
 * MIT license

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, 
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
