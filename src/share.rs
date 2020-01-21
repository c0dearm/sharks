use crate::GF256;

/// A share used for to reconstruct the secret. Can be serialized to and from a byte array for transmission.
///
/// Example:
/// ```
/// # use std::borrow::Borrow;
/// # fn send_to_printer(_: Vec<u8>) { }
/// # fn ask_shares() -> Vec<Vec<u8>> { vec![] }
///
/// # use sharks::{ Sharks, Share };
/// let sharks = Sharks(3);
/// // Obtain an iterator over the shares for secret [1, 2]
/// let dealer = sharks.dealer(&[1, 2, 3]);
///
/// # let mut shares: Vec<Vec<u8>> = Vec::with_capacity(5);
/// // Get 5 shares and print paper keys
/// for s in dealer.take(5) {
///     println!("test");
///     # shares.push(s.clone().into());
///     send_to_printer(s.into());
/// };
///
/// # let shares = vec![shares[0].clone(), shares[2].clone(), shares[4].clone()];
///
/// // Get 3 shares from users and get secret
/// let shares_serialized: Vec<Vec<u8>> = ask_shares();
/// # let shares_serialized = shares;
///
/// let shares: Vec<Share> = shares_serialized.iter().map(|s| s.as_slice().into()).collect();
///
/// let secret = sharks.recover(&shares).expect("we should have at leats 3 shares");
///
/// assert_eq!(secret, vec![1, 2, 3]);
#[derive(Debug, Clone)]
pub struct Share {
    pub x: GF256,
    pub y: Vec<GF256>,
}

impl From<Share> for Vec<u8> {
    fn from(s: Share) -> Vec<u8> {
        let mut serialized: Vec<u8> = Vec::with_capacity(s.y.len() + 1);
        serialized.push(s.x.0);

        serialized.append(&mut s.y.iter().map(|p| p.0).collect());
        serialized
    }
}

impl From<&[u8]> for Share {
    fn from(s: &[u8]) -> Share {
        let x = GF256(s[0]);
        let y = s[1..].iter().map(|p| GF256(*p)).collect();
        Share { x, y }
    }
}
