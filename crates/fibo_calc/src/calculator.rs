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

    pub async fn calc(self) -> Vec<BigInt> {
        if self.builder.is_none_filter() {
            return vec![];
        }

        let (start_nums, Range { start, end }) = match (
            self.builder.get_start_nums(),
            self.builder.get_range_by_id(),
        ) {
            (Some((n1, n2)), Some(range)) => ((n1, n2), range),
            _ => return vec![],
        };

        let mut result = Vec::with_capacity(end.saturating_sub(start));

        if start == 0 {
            result.push(start_nums.0.clone());
        }
        if start <= 1 && end > 1 {
            result.push(start_nums.1.clone());
        }

        if end > 2 {
            let implementation_fibo = LinealFibo::new(Some(start_nums));
            result.extend(implementation_fibo.take(end - 2));
        }

        let filters = self.builder.get_filters();
        result
            .into_iter()
            .skip(start)
            .filter(|n| filters.iter().all(|func| func(n)))
            .collect()
    }
}
