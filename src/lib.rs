//! Fast, small and secure [Shamir's Secret Sharing](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing) library crate
//!
//! # Usage example
//! ```
//! // Configure algorithm with minimum 3 shares to recover secret and security level 12
//! let shamir = sharks::SecretShares::new(3, 12).unwrap();
//! // Generate 3 shares for the 12345 secret
//! let shares = shamir.iter_shares(12345).unwrap().take(3).collect();
//! // Recover the secret from the shares
//! let secret = shamir.secret_from(&shares).unwrap();
//! assert_eq!(secret, 12345);
//! ```

use std::collections::HashMap;

mod math;
mod mersenne;

/// Generate new [Shamir's secret shares](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing) or recover secrets from them.
pub struct SecretShares {
    min_shares: usize,
    prime: u128,
}

impl SecretShares {
    /// Returns a result containing a`SecretShares` instance if parameters are reasonable.
    ///
    /// `security_level` is the index of the [Mersenne prime](https://en.wikipedia.org/wiki/Mersenne_prime) to use as the finite field prime modulo (the higher the more secure, but slower).
    /// Currently, only up to 12 is supported (`p=127, Mp = 2^127 - 1`).
    ///
    /// If `min_shares` is larger or equal to the Mersenne prime an error is returned, as this configuration would generate insecure shares.
    ///
    /// Example, create an instance with minimum 3 shares to recover a secret and 128 bits of security:
    /// ```
    /// let shamir = sharks::SecretShares::new(3, 12);
    /// assert!(shamir.is_ok());
    /// ```
    pub fn new(min_shares: usize, security_level: usize) -> Result<Self, &'static str> {
        let security_level = std::cmp::min(security_level - 1, mersenne::EXPONENTS.len() - 1);

        let prime = u128::pow(2, mersenne::EXPONENTS[security_level]) - 1;

        if (min_shares as u128) < prime {
            Ok(SecretShares { min_shares, prime })
        } else {
            Err("Minimum shares for recovery is too large for current security level")
        }
    }

    /// Given a `secret` returns a result with an iterator which generates shares `(x, f(x))` for x from [1, p).
    ///
    /// If `secret` is larger or equal than the Mersenne prime an error is returned, as it would be irrecoverable.
    ///  
    /// Example, generate 10 shares for secret `12345`:
    /// ```
    /// # use std::collections::HashMap;
    /// let shamir = sharks::SecretShares::new(3, 12).unwrap();
    /// let shares: HashMap<u128, u128> = shamir.iter_shares(12345).unwrap().take(10).collect();
    /// ```
    pub fn iter_shares(&self, secret: u128) -> Result<impl Iterator<Item = (u128, u128)>, &str> {
        if secret < self.prime {
            let (p, coeffs) = math::compute_coeffs(secret, self.min_shares, self.prime);
            Ok(math::get_evaluator(coeffs, p))
        } else {
            Err("Secret is too large for current security level")
        }
    }

    /// Given a set of distinct `shares`, returns a result with the recovered secret.
    ///
    /// If the number of `shares` is less than the number of minimum shares an error is returned as the secret is irrecoverable.
    ///
    /// Example, recover the `12345` secret:
    /// ```
    /// let shamir = sharks::SecretShares::new(3, 12).unwrap();
    /// let shares = shamir.iter_shares(12345).unwrap().take(3).collect();
    /// let secret = shamir.secret_from(&shares).unwrap();
    /// assert_eq!(secret, 12345);
    /// ```
    pub fn secret_from(&self, shares: &HashMap<u128, u128>) -> Result<u128, &str> {
        if shares.len() < self.min_shares {
            Err("Not enough shares to recover secret")
        } else {
            Ok(math::lagrange_root(shares, self.prime))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SecretShares;

    #[test]
    fn test_security_level_range() {
        let shamir = SecretShares::new(10, 1000).unwrap();
        assert_eq!(shamir.prime, u128::pow(2, 127) - 1);

        let shamir = SecretShares::new(2, 1).unwrap();
        assert_eq!(shamir.prime, 3);
    }

    #[test]
    fn test_min_shares_too_large() {
        let shamir = SecretShares::new(3, 1);
        assert!(shamir.is_err());
    }

    #[test]
    fn test_secret_too_large() {
        let shamir = SecretShares::new(2, 1).unwrap();
        let shares = shamir.iter_shares(3);
        assert!(shares.is_err());
    }

    #[test]
    fn test_insufficient_shares() {
        let shamir = SecretShares::new(2, 1).unwrap();
        let shares = shamir.iter_shares(2).unwrap().take(1).collect();
        let secret = shamir.secret_from(&shares);
        assert!(secret.is_err());
    }

    #[test]
    fn test_integration() {
        let shamir = SecretShares::new(10, 128).unwrap();
        let shares = shamir.iter_shares(12345).unwrap().take(100).collect();
        let secret = shamir.secret_from(&shares).unwrap();
        assert_eq!(secret, 12345);
    }
}
