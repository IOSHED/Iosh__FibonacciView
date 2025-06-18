use num_bigint::BigInt;
use std::ops::Range;


type FilterFn = Box<dyn Fn(&BigInt) -> bool>;

#[derive(Default)]
pub struct FiboBuilder {
    start_nums: Option<(BigInt, BigInt)>,
    range_by_index: Option<Range<usize>>,
    other_filters: Vec<FilterFn>,
}

impl FiboBuilder {
    pub fn get_filters(&self) -> &[FilterFn] {
        &self.other_filters
    }

    pub fn get_range_by_id(&self) -> Option<Range<usize>> {
        self.range_by_index.clone()
    }

    pub fn get_start_nums(&self) -> Option<(BigInt, BigInt)> {
        self.start_nums.clone()
    }

    pub fn is_none_filter(&self) -> bool {
        self.range_by_index.is_none() && self.other_filters.is_empty()
    }

    pub fn add_filter(&mut self, filter: impl Fn(&BigInt) -> bool + 'static) -> &mut Self {
        self.other_filters.push(Box::new(filter));
        self
    }

    pub fn set_range_by_id(&mut self, new_range: Option<Range<usize>>) -> &mut Self {
        self.range_by_index = new_range;
        self
    }

    pub fn set_start_nums(&mut self, start_num: Option<(BigInt, BigInt)>) -> &mut Self {
        self.start_nums = start_num;
        self
    }
}
