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
        let n00 = &self.n00 * &other.n00 + &self.n01_and_n10 * &other.n01_and_n10;
        let n01_and_n10 = &self.n00 * &other.n01_and_n10 + &self.n01_and_n10 * &other.n11;
        let n11 = &self.n01_and_n10 * &other.n01_and_n10 + &self.n11 * &other.n11;
        
        self.n00 = n00;
        self.n01_and_n10 = n01_and_n10;
        self.n11 = n11;
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

            self.step = &self.step * &self.step;
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
                    self.fibo *= &self.step;
                    self.fibo.n00.clone()
                }
            }
        };
        self.count += 1;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;
    use test_case::test_case;

    #[test]
    fn test_default_initialization() {
        let fibo = MatmulFibo::default();
        assert_eq!(fibo.count, 1);
        assert_eq!(fibo.step.n00, BigInt::one());
        assert_eq!(fibo.step.n01_and_n10, BigInt::one());
        assert_eq!(fibo.step.n11, BigInt::zero());
    }

    #[test_case((2, 3), (3, 2, 1) ; "simple custom init")]
    #[test_case((5, 8), (8, 5, 3) ; "larger custom init")]
    fn test_custom_initialization(
        (a, b): (i32, i32),
        (expected_n00, expected_n01_n10, expected_n11): (i32, i32, i32)
    ) {
        let start_nums = Some((BigInt::from(a), BigInt::from(b)));
        let fibo = MatmulFibo::new(start_nums);
        assert_eq!(fibo.fibo.n00, BigInt::from(expected_n00));
        assert_eq!(fibo.fibo.n01_and_n10, BigInt::from(expected_n01_n10));
        assert_eq!(fibo.fibo.n11, BigInt::from(expected_n11));
    }

    #[test_case(5, 3 ; "5th fibonacci number")]
    #[test_case(6, 5 ; "6th fibonacci number")]
    #[test_case(10, 34 ; "10th fibonacci number")]
    #[test_case(15, 377 ; "15th fibonacci number")]
    fn test_calc_one(n: i32, expected: i32) {
        let mut fibo = MatmulFibo::default();
        assert_eq!(fibo.calc_one(BigInt::from(n)), BigInt::from(expected));
    }

    #[test_case((1, 1, 0), (1, 1, 0), (2, 1, 1) ; "basic matrix multiplication")]
    #[test_case((2, 1, 0), (1, 1, 0), (3, 2, 1) ; "matrix multiplication with different values")]
    fn test_matrix_multiplication(
        (m1_n00, m1_n01, m1_n11): (i32, i32, i32),
        (m2_n00, m2_n01, m2_n11): (i32, i32, i32),
        (expected_n00, expected_n01, expected_n11): (i32, i32, i32)
    ) {
        let m1 = M2x2::new(
            BigInt::from(m1_n00),
            BigInt::from(m1_n01),
            BigInt::from(m1_n11),
        );
        let m2 = M2x2::new(
            BigInt::from(m2_n00),
            BigInt::from(m2_n01),
            BigInt::from(m2_n11),
        );

        let result = &m1 * &m2;
        assert_eq!(result.n00, BigInt::from(expected_n00));
        assert_eq!(result.n01_and_n10, BigInt::from(expected_n01));
        assert_eq!(result.n11, BigInt::from(expected_n11));
    }

    #[test_case(&[3, 7, 10, 17, 27] ; "custom sequence starting with 3,7")]
    #[test_case(&[0, 1, 1, 2, 3, 5, 8, 13] ; "default fibonacci sequence")]
    fn test_sequence(expected: &[i32]) {
        let start_nums = if expected[0] != 0 {
            Some((BigInt::from(expected[0]), BigInt::from(expected[1])))
        } else {
            None
        };

        let mut fibo = if start_nums.is_some() {
            MatmulFibo::new(start_nums)
        } else {
            MatmulFibo::default()
        };

        for &expected_num in expected {
            let next_num = fibo.next().unwrap();
            assert_eq!(next_num, BigInt::from(expected_num));
        }
    }
}
