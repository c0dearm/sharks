//! Fast, small and secure [Shamir's Secret Sharing](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing) library crate
//!
//! Usage example:
//! ```
//! use sharks::{ Sharks, Share };
//!
//! // Set a minimum threshold of 10 shares
//! let sharks = Sharks(10);
//! // Obtain an iterator over the shares for secret [1, 2, 3, 4]
//! let dealer = sharks.dealer(&[1, 2, 3, 4]);
//! // Get 10 shares
//! let shares: Vec<Share> = dealer.take(10).collect();
//! // Recover the original secret!
//! let secret = sharks.recover(shares.as_slice()).unwrap();
//! assert_eq!(secret, vec![1, 2, 3, 4]);
//! ```

mod field;
mod math;
mod share;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

use std::collections::HashSet;

use field::GF256;
pub use share::Share;

/// Tuple struct which implements methods to generate shares and recover secrets over a 256 bits Galois Field.
/// Its only parameter is the minimum shares threshold.
///
/// Usage example:
/// ```
/// # use sharks::{ Sharks, Share };
/// // Set a minimum threshold of 10 shares
/// let sharks = Sharks(10);
/// // Obtain an iterator over the shares for secret [1, 2, 3, 4]
/// let dealer = sharks.dealer(&[1, 2, 3, 4]);
/// // Get 10 shares
/// let shares: Vec<Share> = dealer.take(10).collect();
/// // Recover the original secret!
/// let secret = sharks.recover(shares.as_slice()).unwrap();
/// assert_eq!(secret, vec![1, 2, 3, 4]);
/// ```
pub struct Sharks(pub u8);

impl Sharks {
    /// Given a `secret` byte slice, returns an `Iterator` along new shares.
    /// The maximum number of shares that can be generated is 256.
    ///
    /// Example:
    /// ```
    /// # use sharks::{ Sharks, Share };
    /// # let sharks = Sharks(3);
    /// // Obtain an iterator over the shares for secret [1, 2]
    /// let dealer = sharks.dealer(&[1, 2]);
    /// // Get 3 shares
    /// let shares: Vec<Share> = dealer.take(3).collect();
    pub fn dealer(&self, secret: &[u8]) -> impl Iterator<Item = Share> {
        let mut polys = Vec::with_capacity(secret.len());

        for chunk in secret {
            polys.push(math::random_polynomial(GF256(*chunk), self.0))
        }

        math::get_evaluator(polys)
    }

    /// Given a slice of shares, recovers the original secret.
    /// If the number of distinct shares is less than the minimum threshold an `Err` is returned,
    /// otherwise an `Ok` containing the secret.
    ///
    /// Example:
    /// ```
    /// # use sharks::{ Sharks, Share };
    /// # let sharks = Sharks(3);
    /// # let mut shares: Vec<Share> = sharks.dealer(&[1]).take(3).collect();
    /// // Recover original secret from shares
    /// let mut secret = sharks.recover(&shares);
    /// // Secret correctly recovered
    /// assert!(secret.is_ok());
    /// // Remove shares for demonstration purposes
    /// shares.clear();
    /// secret = sharks.recover(&shares);
    /// // Not enough shares to recover secret
    /// assert!(secret.is_err());
    pub fn recover(&self, shares: &[Share]) -> Result<Vec<u8>, &str> {
        let shares_x: HashSet<u8> = shares.iter().map(|s| s.x.0).collect();

        if shares_x.len() < self.0 as usize {
            Err("Not enough shares to recover original secret")
        } else {
            Ok(math::interpolate(shares))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Share, Sharks};

    #[test]
    fn test_insufficient_shares_err() {
        let sharks = Sharks(255);
        let dealer = sharks.dealer(&[1]);
        let shares: Vec<Share> = dealer.take(254).collect();
        let secret = sharks.recover(&shares);
        assert!(secret.is_err());
    }

    #[test]
    fn test_duplicate_shares_err() {
        let sharks = Sharks(255);
        let dealer = sharks.dealer(&[1]);
        let mut shares: Vec<Share> = dealer.take(255).collect();
        shares[1] = Share {
            x: shares[0].x,
            y: shares[0].y.clone(),
        };
        let secret = sharks.recover(&shares);
        assert!(secret.is_err());
    }

    #[test]
    fn test_integration_works() {
        let sharks = Sharks(255);
        let dealer = sharks.dealer(&[1, 2, 3, 4]);
        let shares: Vec<Share> = dealer.take(255).collect();
        let secret = sharks.recover(&shares).unwrap();
        assert_eq!(secret, vec![1, 2, 3, 4]);
    }
}
