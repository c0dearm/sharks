# Sharks

Fast, small and secure [Shamir's Secret Sharing](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing) library crate

Documentation:
-    [API reference (docs.rs)](https://docs.rs/sharks)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sharks = "0.1"
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

Currently only finite fields with modulus up to 128 bits (12th Mersenne prime) are supported. This means:
-    Only up to `2^128` shares can be generated.
-    Maximum secret length is 128 bits.

This is imposed by the Rust maximum unsigned integer length, which is `u128`.
Going around this limitation would mean using crates like `num-bigint` in most of the computations, reducing performance drastically.

## Testing

This crate contains both unit and benchmark tests (as well as the examples included in the docs).
You can run them with `cargo test` and `cargo bench`.

### Benchmark results [min mean max]

| CPU                                       | obtain_shares_iterator          | step_shares_iterator            | recover_secret                  |
| ----------------------------------------- | ------------------------------- | ------------------------------- | ------------------------------- |
| Intel(R) Core(TM) i7-8550U CPU @ 1.80GHz  | [14.023 us 14.087 us 14.146 us] | [413.19 us 414.90 us 416.60 us] | [24.978 ms 25.094 ms 25.226 ms] |

# Contributing

If you find a vulnerability, bug or would like a new feature, [open a new issue](https://github.com/c0dearm/sharks/issues/new).

To introduce your changes into the codebase, submit a Pull Request.

Many thanks!

# License

Sharks is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
