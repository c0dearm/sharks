#![no_main]
use libfuzzer_sys::fuzz_target;

use arbitrary::Arbitrary;
use sharks::{Share, Sharks};

#[derive(Debug, Arbitrary)]
struct Parameters {
    pub threshold: u8,
    pub shares: Vec<Share>,
}

fuzz_target!(|params: Parameters| {
    let sharks = Sharks(params.threshold);
    let _secret = sharks.recover(&params.shares);
});
