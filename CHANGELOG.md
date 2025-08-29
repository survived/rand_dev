## v0.2.0
* BREAKING: update rand_core dep to latest `v0.9`
* POSSIBLY BREAKING: the seed is now printed to stderr instead of stdout
* Replace dependency on `hex` with better-maintained `const-hex v1`
* Add optional feature `rand-v09`: when it's enabled, library re-exports `rand v0.9` that will be
  accessible as `rand_dev::rand`
* Improve CI workflow

See [#4](https://github.com/survived/rand_dev/pull/4)

## v0.1.1
* Add `DevRng::fork` method that derives a new randomness generator from existing one \
  May be useful when you have several threads/futures/places where you need access the randomness
  generation, but you don't want to mess with ownership system.
* Changes format of seed printed to stdout \
  Old format:
  ```text
  Tests seed: {seed}
  ```
  New format:
  ```text
  RUST_TESTS_SEED={seed}
  ```
  New format makes it easier to copy-paste env var when you want to reproduce the tests.
* Add Github Actions

See [#2](https://github.com/survived/rand_dev/pull/2)

# v0.1.0
First release of the library
