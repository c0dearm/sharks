# Sharks

[![CI](https://github.com/c0dearm/sharks/workflows/Continuous%20Integration/badge.svg?branch=master)](https://github.com/c0dearm/sharks/actions)
[![Codecov](https://codecov.io/gh/c0dearm/sharks/branch/master/graph/badge.svg)](https://codecov.io/gh/c0dearm/sharks)
[![Crate](https://img.shields.io/crates/v/sharks.svg)](https://crates.io/crates/sharks)
[![Docs](https://docs.rs/sharks/badge.svg)](https://docs.rs/sharks)

Fast, small and secure [Shamir's Secret Sharing](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing) library crate

Documentation:
-    [API reference (docs.rs)](https://docs.rs/sharks)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sharks = "0.3"
```

To get started using Sharks, see the [Rust docs](https://docs.rs/sharks)

## Features

### Developer friendly
The API is simple and to the point, with minimal configuration.

### Fast and small
The code is as idiomatic and clean as possible, with minimum external dependencies.

### Secure by design
The implementation forbids the user to choose parameters that would result in an insecure application,
like generating more shares than what's allowed by the finite field length.

## Limitations

Because the Galois finite field it uses is [GF256](https://en.wikipedia.org/wiki/Finite_field#GF(p2)_for_an_odd_prime_p),
only up to 255 shares can be generated for a given secret. A larger number would be insecure as shares would start duplicating.
Nevertheless, the secret can be arbitrarily long as computations are performed on single byte chunks.

## Testing

This crate contains both unit and benchmark tests (as well as the examples included in the docs).
You can run them with `cargo test` and `cargo bench`.

### Benchmark results [min mean max]

| CPU                                       | obtain_shares_dealer            | step_shares_dealer              | recover_secret                  | share_from_bytes                | share_to_bytes                  |
| ----------------------------------------- | ------------------------------- | ------------------------------- | ------------------------------- | ------------------------------- | ------------------------------- |
| Intel(R) Core(TM) i7-8550U CPU @ 1.80GHz  | [1.4321 us 1.4339 us 1.4357 us] | [1.3385 ns 1.3456 ns 1.3552 ns] | [228.77 us 232.17 us 236.23 us] | [24.688 ns 25.083 ns 25.551 ns] | [22.832 ns 22.910 ns 22.995 ns] |

# Contributing

If you find a vulnerability, bug or would like a new feature, [open a new issue](https://github.com/c0dearm/sharks/issues/new).

To introduce your changes into the codebase, submit a Pull Request.

Many thanks!

# License

Sharks is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
