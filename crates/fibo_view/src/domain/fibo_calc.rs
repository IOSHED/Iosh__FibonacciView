use crate::app::{Filter, FilterType};
use fibo_calc::{FiboBuilder, FiboCalc, FiboTaskReceiver};
use num_bigint::BigInt;

pub async fn calculate_fibonacci(
    start_nums: (BigInt, BigInt), range: std::ops::Range<usize>, filters: &[Filter],
) -> FiboTaskReceiver {
    let mut builder = FiboBuilder::default();

    builder
        .set_start_nums(Some(start_nums))
        .set_range_by_id(Some(range.start..range.end));

    for filter in filters {
        let value = filter.value.clone();
        match filter.filter_type {
            FilterType::Ge => builder.add_filter(move |num| num >= &value),
            FilterType::Le => builder.add_filter(move |num| num <= &value),
        };
    }

    let calc = FiboCalc::new(builder);
    let receiver = calc.calc_background();

    receiver
}
