use num_bigint::BigInt;

pub struct LinealFibo {
    last: BigInt,
    pre_last: BigInt,
}

impl LinealFibo {
    pub fn new(start_nums: Option<(BigInt, BigInt)>) -> Self {
        if let Some((start_num1, start_num2)) = start_nums {
            return Self {
                last: start_num2,
                pre_last: start_num1,
            };
        }
        Default::default()
    }
}

impl Default for LinealFibo {
    fn default() -> Self {
        Self {
            pre_last: BigInt::from(-1),
            last: BigInt::from(1),
        }
    }
}

impl Iterator for LinealFibo {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let res = &self.last + &self.pre_last;
        self.pre_last = self.last.clone();
        self.last = res.clone();
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;
    use test_case::test_case;

    #[test]
    fn test_default_values() {
        let fibo = LinealFibo::default();
        assert_eq!(fibo.pre_last, BigInt::from(-1));
        assert_eq!(fibo.last, BigInt::from(1));
    }

    #[test]
    fn test_default_builder_in_new_method() {
        let fibo = LinealFibo::new(None);
        assert_eq!(fibo.pre_last, BigInt::from(-1));
        assert_eq!(fibo.last, BigInt::from(1));
    }

    #[test_case((BigInt::from(0), BigInt::from(1)), 1, 1; "classic start")]
    #[test_case((BigInt::from(2), BigInt::from(3)), 3, 5; "custom start")]
    #[test_case((BigInt::from(-5), BigInt::from(8)), 8, 3; "negative start")]
    fn test_new_with_start_nums(start: (BigInt, BigInt), expected_last: i32, expected_next: i32) {
        let mut fibo = LinealFibo::new(Some((start.0.clone(), start.1.clone())));
        assert_eq!(fibo.pre_last, start.0);
        assert_eq!(fibo.last, start.1);
        let next = fibo.next().unwrap();
        assert_eq!(
            next,
            BigInt::from(expected_last + expected_next - expected_last)
        );
    }

    #[test]
    fn test_sequence() {
        let mut fibo = LinealFibo::new(Some((BigInt::from(0), BigInt::from(1))));
        let expected = vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
        for val in expected {
            assert_eq!(fibo.next().unwrap(), BigInt::from(val));
        }
    }

    #[test]
    fn test_iterator_trait() {
        let fibo = LinealFibo::default();
        let vals: Vec<BigInt> = fibo.take(5).collect();
        let expected = vec![
            BigInt::from(0),
            BigInt::from(1),
            BigInt::from(1),
            BigInt::from(2),
            BigInt::from(3),
        ];
        assert_eq!(vals, expected);
    }
}
