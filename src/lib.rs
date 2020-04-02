//! Fast, small and secure [Shamir's Secret Sharing](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing) library crate
//!
//! Usage example (std):
//! ```
//! use sharks::{ Sharks, Share };
//!
//! // Set a minimum threshold of 10 shares
//! let sharks = Sharks(10);
//! // Obtain an iterator over the shares for secret [1, 2, 3, 4]
//! # #[cfg(feature = "std")]
//! # {
//! let dealer = sharks.dealer(&[1, 2, 3, 4]);
//! // Get 10 shares
//! let shares: Vec<Share> = dealer.take(10).collect();
//! // Recover the original secret!
//! let secret = sharks.recover(shares.as_slice()).unwrap();
//! assert_eq!(secret, vec![1, 2, 3, 4]);
//! # }
//! ```
//!
//! Usage example (no std):
//! ```
//! use sharks::{ Sharks, Share };
//! use rand_chacha::rand_core::SeedableRng;
//!
//! // Set a minimum threshold of 10 shares
//! let sharks = Sharks(10);
//! // Obtain an iterator over the shares for secret [1, 2, 3, 4]
//! let mut rng = rand_chacha::ChaCha8Rng::from_seed([0x90; 32]);
//! let dealer = sharks.dealer_rng(&[1, 2, 3, 4], &mut rng);
//! // Get 10 shares
//! let shares: Vec<Share> = dealer.take(10).collect();
//! // Recover the original secret!
//! let secret = sharks.recover(shares.as_slice()).unwrap();
//! assert_eq!(secret, vec![1, 2, 3, 4]);
//! ```
#![no_std]

mod field;
mod math;
mod share;

extern crate alloc;

use alloc::vec::Vec;
use hashbrown::HashSet;

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
/// # #[cfg(feature = "std")]
/// # {
/// let dealer = sharks.dealer(&[1, 2, 3, 4]);
/// // Get 10 shares
/// let shares: Vec<Share> = dealer.take(10).collect();
/// // Recover the original secret!
/// let secret = sharks.recover(shares.as_slice()).unwrap();
/// assert_eq!(secret, vec![1, 2, 3, 4]);
/// # }
/// ```
pub struct Sharks(pub u8);

impl Sharks {
    /// This method is useful when `std` is not available. For typical usage
    /// see the `dealer` method.
    ///
    /// Given a `secret` byte slice, returns an `Iterator` along new shares.
    /// The maximum number of shares that can be generated is 256.
    /// A random number generator has to be provided.
    ///
    /// Example:
    /// ```
    /// # use sharks::{ Sharks, Share };
    /// # use rand_chacha::rand_core::SeedableRng;
    /// # let sharks = Sharks(3);
    /// // Obtain an iterator over the shares for secret [1, 2]
    /// let mut rng = rand_chacha::ChaCha8Rng::from_seed([0x90; 32]);
    /// let dealer = sharks.dealer_rng(&[1, 2], &mut rng);
    /// // Get 3 shares
    /// let shares: Vec<Share> = dealer.take(3).collect();
    pub fn dealer_rng<R: rand::Rng>(
        &self,
        secret: &[u8],
        rng: &mut R,
    ) -> impl Iterator<Item = Share> {
        let mut polys = Vec::with_capacity(secret.len());

        for chunk in secret {
            polys.push(math::random_polynomial(GF256(*chunk), self.0, rng))
        }

        math::get_evaluator(polys)
    }

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
    #[cfg(feature = "std")]
    pub fn dealer(&self, secret: &[u8]) -> impl Iterator<Item = Share> {
        let mut rng = rand::thread_rng();
        self.dealer_rng(secret, &mut rng)
    }

    /// Given an iterable collection of shares, recovers the original secret.
    /// If the number of distinct shares is less than the minimum threshold an `Err` is returned,
    /// otherwise an `Ok` containing the secret.
    ///
    /// Example:
    /// ```
    /// # use sharks::{ Sharks, Share };
    /// # use rand_chacha::rand_core::SeedableRng;
    /// # let sharks = Sharks(3);
    /// # let mut rng = rand_chacha::ChaCha8Rng::from_seed([0x90; 32]);
    /// # let mut shares: Vec<Share> = sharks.dealer_rng(&[1], &mut rng).take(3).collect();
    /// // Recover original secret from shares
    /// let mut secret = sharks.recover(&shares);
    /// // Secret correctly recovered
    /// assert!(secret.is_ok());
    /// // Remove shares for demonstration purposes
    /// shares.clear();
    /// secret = sharks.recover(&shares);
    /// // Not enough shares to recover secret
    /// assert!(secret.is_err());
    pub fn recover<'a, T>(&self, shares: T) -> Result<Vec<u8>, &str>
    where
        T: IntoIterator<Item = &'a Share>,
        T::IntoIter: Iterator<Item = &'a Share>,
    {
        let (keys, shares) = shares
            .into_iter()
            .map(|s| {
                (
                    s.x.0,
                    Share {
                        x: s.x,
                        y: s.y.clone(),
                    },
                )
            })
            .unzip::<u8, Share, HashSet<u8>, Vec<Share>>();

        if keys.len() < self.0 as usize {
            Err("Not enough shares to recover original secret")
        } else {
            Ok(math::interpolate(shares.as_slice()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Share, Sharks};
    use alloc::vec::Vec;
    #[cfg(not(feature = "std"))]
    use rand_chacha::rand_core::SeedableRng;

    #[test]
    fn test_insufficient_shares_err() {
        let sharks = Sharks(255);

        #[cfg(not(feature = "std"))]
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0x90; 32]);

        #[cfg(feature = "std")]
        let dealer = sharks.dealer(&[1]);
        #[cfg(not(feature = "std"))]
        let dealer = sharks.dealer_rng(&[1], &mut rng);

        let shares: Vec<Share> = dealer.take(254).collect();
        let secret = sharks.recover(&shares);
        assert!(secret.is_err());
    }

    #[test]
    fn test_duplicate_shares_err() {
        let sharks = Sharks(255);

        #[cfg(not(feature = "std"))]
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0x90; 32]);

        #[cfg(feature = "std")]
        let dealer = sharks.dealer(&[1]);
        #[cfg(not(feature = "std"))]
        let dealer = sharks.dealer_rng(&[1], &mut rng);

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

        #[cfg(not(feature = "std"))]
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0x90; 32]);

        #[cfg(feature = "std")]
        let dealer = sharks.dealer(&[1, 2, 3, 4]);
        #[cfg(not(feature = "std"))]
        let dealer = sharks.dealer_rng(&[1, 2, 3, 4], &mut rng);

        let shares: Vec<Share> = dealer.take(255).collect();
        let secret = sharks.recover(&shares).unwrap();
        assert_eq!(secret, alloc::vec![1, 2, 3, 4]);
    }
}
