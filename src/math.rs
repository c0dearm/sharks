// A module that contains necessary algorithms to compute Shamir's shares and recover secrets

use std::collections::HashMap;

use num_bigint::{BigInt, BigUint};
use num_traits::cast::ToPrimitive;
use num_traits::Zero;
use rand::distributions::{Distribution, Uniform};

// Computes `num/(num - b) mod p`, a necessary step to compute the [root of the Lagrange polynomial](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing#Computationally_efficient_approach).
// To find the modulo multiplicative inverse of `num - b` the [Extended Euclidean Algorithm](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse#Computation) is used.
fn div_diff_mod(num: &u128, b: &u128, p: u128) -> u128 {
    let (mut m, mut x, mut inv, mut den) = (
        p,
        0i128,
        1i128,
        if num < b { p - (b - num) } else { num - b },
    );

    while den > 1 {
        inv -= ((den / m) as i128) * x;
        den %= m;
        std::mem::swap(&mut den, &mut m);
        std::mem::swap(&mut x, &mut inv);
    }

    let mut res = BigInt::from(inv);
    if inv < 0 {
        res += p
    }

    (num * res % p).to_u128().unwrap()
}

// Finds the [root of the Lagrange polynomial](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing#Computationally_efficient_approach).
pub fn lagrange_root(points: &HashMap<u128, u128>, p: u128) -> u128 {
    (points
        .iter()
        .enumerate()
        .map(|(j, (xj, yj))| {
            points
                .iter()
                .enumerate()
                .filter(|(m, _)| *m != j)
                .map(|(_, (xm, _))| div_diff_mod(xm, xj, p))
                .product::<BigUint>()
                * yj
                % p
        })
        .sum::<BigUint>()
        % p)
        .to_u128()
        .unwrap()
}

// Generates `k` polynomial coefficients, being the last one `s` and the others randomly generated between `[1, p)`.
// Coefficient degrees go from higher to lower in the returned vector order.
pub fn compute_coeffs(s: u128, k: usize, p: u128) -> (u128, Vec<u128>) {
    let mut coeffs = Vec::with_capacity(k);
    let between = Uniform::new(1, p);
    let mut rng = rand::thread_rng();

    for _ in 1..k {
        coeffs.push(between.sample(&mut rng));
    }
    coeffs.push(s);

    (p, coeffs)
}

// Given a set of polynomial coefficients `coeffs` and a modulus `p`, returns an iterator that computes a `(x, f(x) mod p)` point
// on each iteration. The iterator starts for `x = 1` and ends at `x = p-1`.
pub fn get_evaluator(coeffs: Vec<u128>, p: u128) -> impl Iterator<Item = (u128, u128)> {
    (1..p).map(move |x| {
        (
            x,
            coeffs
                .iter()
                .fold(BigUint::zero(), |acc, c| (acc * x + c) % p)
                .to_u128()
                .unwrap(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{compute_coeffs, div_diff_mod, get_evaluator, lagrange_root};

    #[test]
    fn div_diff_mod_works() {
        let res = div_diff_mod(&2, &1, 7);
        assert_eq!(res, 2);

        let res = div_diff_mod(&1, &2, 7);
        assert_eq!(res, 6);
    }

    #[test]
    fn lagrange_root_works() {
        let iter = get_evaluator(vec![3, 2, 1], 7);
        let values = iter.take(3).collect();
        let root = lagrange_root(&values, 7);
        assert_eq!(root, 1);

        let iter = get_evaluator(vec![3, 2, 5], 7);
        let values = iter.take(3).collect();
        let root = lagrange_root(&values, 7);
        assert_eq!(root, 5);
    }

    #[test]
    fn compute_coeffs_works() {
        let coeffs = compute_coeffs(1, 4, 7);
        assert_eq!(coeffs.0, 7);
        assert_eq!(coeffs.1.len(), 4);
        assert_eq!(coeffs.1[3], 1);
    }

    #[test]
    fn evaluator_works() {
        let iter = get_evaluator(vec![3, 2, 5], 7);
        let values: Vec<_> = iter.take(2).collect();
        assert_eq!(values, vec![(1, 3), (2, 0)]);
    }
}
