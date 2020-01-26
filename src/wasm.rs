use wasm_bindgen::prelude::*;

use crate::{ Sharks, Share };

#[wasm_bindgen]
pub fn generate_shares(n_shares: u8, threshold: u8, secret: &[u8]) -> JsValue {
    let sharks = Sharks(threshold);
    let dealer = sharks.dealer(secret);
    let shares: Vec<Vec<u8>> = dealer.take(n_shares as usize).map(|s| (&s).into()).collect();

    JsValue::from_serde(&shares).expect("A Vec<Vec<u8>> should always be JSON serializable.")
}

#[wasm_bindgen]
pub fn recover(threshold: u8, shares: JsValue) -> Vec<u8> {
    let sharks = Sharks(threshold);

    let shares: Vec<Vec<u8>> = shares.into_serde().expect("will implement proper error handling later");

    let shares: Vec<Share> = shares.iter().map(|s| s.as_slice().into()).collect();
    sharks.recover(&shares).expect("will implement proper error handling later").into()
}