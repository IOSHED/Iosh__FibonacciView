use crate::calculator::ImplementationFibo;
use num_bigint::BigInt;
use num_traits::{One, Zero};

#[derive(Debug)]
struct M2x2 {
    n00: BigInt,
    n01_and_n10: BigInt,
    n11: BigInt,
}

impl M2x2 {
    fn new(n00: BigInt, n01_and_n10: BigInt, n11: BigInt) -> Self {
        Self {
            n00,
            n01_and_n10,
            n11,
        }
    }

    fn mul(&mut self, other: &Self) {
        let n00 = &self.n00 * &other.n00 + &self.n01_and_n10 * &other.n01_and_n10;
        let n01_and_n10 = &self.n00 * &other.n01_and_n10 + &self.n01_and_n10 * &other.n11;
        let n11 = &self.n01_and_n10 * &other.n01_and_n10 + &self.n11 * &other.n11;

        self.n00 = n00;
        self.n01_and_n10 = n01_and_n10;
        self.n11 = n11;
    }
}

#[derive(Debug)]
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

impl Default for MatmulFibo {
    fn default() -> Self {
        Self {
            step: M2x2::new(BigInt::from(3), BigInt::from(2), BigInt::from(1)),
            fibo: M2x2::new(BigInt::one(), BigInt::zero(), BigInt::one()),
            count: 1,
        }
    }
}

impl Iterator for MatmulFibo {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        match self.count % 3 {
            1 => {
                self.fibo.mul(&self.step);
                Some(self.fibo.n11.clone())
            }
            0 => Some(self.fibo.n00.clone()),
            2 => Some(self.fibo.n01_and_n10.clone()),
            _ => None,
        }
    }
}
