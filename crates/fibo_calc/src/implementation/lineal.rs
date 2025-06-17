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
