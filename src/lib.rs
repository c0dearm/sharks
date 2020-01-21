//! Fast, small and secure [Shamir's Secret Sharing](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing) library crate
//!
//! Usage example:
//! ```
//! use sharks::Sharks;
//!
//! // Set a minimum threshold of 10 shares
//! let sharks = Sharks(10);
//! // Obtain an iterator over the shares for secret [1, 2, 3, 4]
//! let dealer = sharks.dealer(&[1, 2, 3, 4]);
//! // Get 10 shares
//! let shares = dealer.take(10).collect();
//! // Recover the original secret!
//! let secret = sharks.recover(&shares).unwrap();
//! assert_eq!(secret, vec![1, 2, 3, 4]);
//! ```

mod field;
mod math;

use std::collections::HashMap;

use field::GF256;

/// Tuple struct which implements methods to generate shares and recover secrets over a 256 bits Galois Field.
/// Its only parameter is the minimum shares threshold.
///
/// Usage example:
/// ```
/// # use sharks::Sharks;
/// // Set a minimum threshold of 10 shares
/// let sharks = Sharks(10);
/// // Obtain an iterator over the shares for secret [1, 2, 3, 4]
/// let dealer = sharks.dealer(&[1, 2, 3, 4]);
/// // Get 10 shares
/// let shares = dealer.take(10).collect();
/// // Recover the original secret!
/// let secret = sharks.recover(&shares).unwrap();
/// assert_eq!(secret, vec![1, 2, 3, 4]);
/// ```
pub struct Sharks(pub u8);

impl Sharks {
    /// Given a `secret` byte slice, returns an `Iterator` along new shares.
    /// The maximum number of shares that can be generated is 256.
    ///
    /// Example:
    /// ```
    /// # use std::collections::HashMap;
    /// # use sharks::Sharks;
    /// # let sharks = Sharks(3);
    /// // Obtain an iterator over the shares for secret [1, 2]
    /// let dealer = sharks.dealer(&[1, 2]);
    /// // Get 3 shares
    /// let shares: HashMap<_, _> = dealer.take(3).collect();
    pub fn dealer(&self, secret: &[u8]) -> impl Iterator<Item = (GF256, Vec<GF256>)> {
        let mut polys = Vec::with_capacity(secret.len());

        for chunk in secret {
            polys.push(math::random_polynomial(GF256(*chunk), self.0))
        }

        math::get_evaluator(polys)
    }

    /// Given a `HashMap` of shares, recovers the original secret.
    /// If the number of shares is less than the minimum threshold an `Err` is returned,
    /// otherwise an `Ok` containing the secret.
    ///
    /// Example:
    /// ```
    /// # use sharks::Sharks;
    /// # let sharks = Sharks(3);
    /// # let mut shares = sharks.dealer(&[1]).take(3).collect();
    /// // Revover original secret from shares
    /// let mut secret = sharks.recover(&shares);
    /// // Secret correctly recovered
    /// assert!(secret.is_ok());
    /// // Remove shares for demonstrastion purposes
    /// shares.clear();
    /// secret = sharks.recover(&shares);
    /// // Not enough shares to recover secret
    /// assert!(secret.is_err());
    pub fn recover(&self, shares: &HashMap<GF256, Vec<GF256>>) -> Result<Vec<u8>, &str> {
        if shares.len() < self.0 as usize {
            Err("Not enough shares to recover original secret")
        } else {
            Ok(math::interpolate(shares))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Sharks, GF256};

    #[test]
    fn test_insufficient_shares_err() {
        let sharks = Sharks(255);
        let dealer = sharks.dealer(&[1]);
        let shares = dealer.take(254).collect();
        let secret = sharks.recover(&shares);
        assert!(secret.is_err());
    }

    #[test]
    fn test_integration_works() {
        let sharks = Sharks(255);
        let dealer = sharks.dealer(&[1, 2, 3, 4]);
        let shares: std::collections::HashMap<GF256, Vec<GF256>> = dealer.take(255).collect();
        let secret = sharks.recover(&shares).unwrap();
        assert_eq!(secret, vec![1, 2, 3, 4]);
    }
}
