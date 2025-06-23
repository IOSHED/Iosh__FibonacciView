use crate::calculator::ImplementationFibo;
use num_bigint::BigInt;
use num_traits::{One, Zero};

use std::ops::{Mul, MulAssign};

struct M2x2 {
    n00: BigInt,
    n01_and_n10: BigInt,
    n11: BigInt,
}

impl M2x2 {
    fn new(n00: BigInt, n01_and_n10: BigInt, n11: BigInt) -> Self {
        Self { n00, n01_and_n10, n11 }
    }
}

impl Mul<&M2x2> for &M2x2 {
    type Output = M2x2;

    fn mul(self, other: &M2x2) -> M2x2 {
        M2x2 {
            n00: &self.n00 * &other.n00 + &self.n01_and_n10 * &other.n01_and_n10,
            n01_and_n10: &self.n00 * &other.n01_and_n10 + &self.n01_and_n10 * &other.n11,
            n11: &self.n01_and_n10 * &other.n01_and_n10 + &self.n11 * &other.n11,
        }
    }
}

impl MulAssign<&M2x2> for M2x2 {
    fn mul_assign(&mut self, other: &M2x2) {
        *self = &*self * other;
    }
}

pub struct MatmulFibo {
    fibo: M2x2,
    step: M2x2,
    count: usize,
}

impl ImplementationFibo for MatmulFibo {
    fn new(start_nums: Option<(BigInt, BigInt)>) -> Self {
        if let Some((a, b)) = start_nums {
            let n00 = b.clone();
            let n01_and_n10 = a.clone();
            let n11 = &b - &a;

            return Self {
                fibo: M2x2::new(n00, n01_and_n10, n11),
                ..Default::default()
            };
        }

        Default::default()
    }
}

impl MatmulFibo {
    pub fn calc_one(&mut self, n: BigInt) -> BigInt {
        let mut n = n - 2;

        while &n > &BigInt::zero() {
            if (&n & &BigInt::one()) != BigInt::zero() {
                self.fibo *= &self.step;
            }

            self.step *= &self.step;
            n >>= 1;
        }

        self.fibo.n00.clone()
    }
}

impl Default for MatmulFibo {
    fn default() -> Self {
        Self {
            step: M2x2::new(BigInt::one(), BigInt::one(), BigInt::zero()),
            fibo: M2x2::new(BigInt::one(), BigInt::zero(), BigInt::one()),
            count: 1,
        }
    }
}

impl Iterator for MatmulFibo {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let result = {
            match self.count {
                1 => self.fibo.n01_and_n10.clone(),
                2 => self.fibo.n00.clone(),
                _ => {
                    self.fibo.mul(&self.step);
                    self.fibo.n00.clone()
                }
            }
        };
        self.count += 1;
        Some(result)
    }
}
