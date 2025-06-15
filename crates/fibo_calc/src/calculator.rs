use crate::builder::FiboBuilder;
use crate::implementation::lineal::LinealFibo;
use num_bigint::BigInt;
use std::ops::Range;

pub struct FiboCalc {
    builder: FiboBuilder,
}

impl FiboCalc {
    pub fn new(builder: FiboBuilder) -> Self {
        Self { builder }
    }

    pub fn calc(self) -> Vec<BigInt> {
        if self.builder.is_none_filter() {
            return vec![];
        }

        let implementation_fibo = LinealFibo::new(self.builder.get_start_nums());

        let Range { start, end } = self.builder.get_range_by_id().unwrap();

        let mut result_fibo: Vec<BigInt> = implementation_fibo.skip(start).take(end).collect();

        let mut filters = self.builder.get_filters();
        self.merge_result_fibo_and_start_nums(&mut result_fibo)
            .into_iter()
            .filter(|n| filters.all(|func| func(n)))
            .collect()
    }

    fn merge_result_fibo_and_start_nums(&self, result_fibo: &mut Vec<BigInt>) -> Vec<BigInt> {
        let mut start_nums_vec = vec![];
        if let Some((start_num1, start_num2)) = self.builder.get_start_nums() {
            start_nums_vec.append(&mut vec![start_num1, start_num2])
        }

        start_nums_vec.append(result_fibo);
        start_nums_vec
    }
}
