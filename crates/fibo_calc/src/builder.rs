use num_bigint::BigInt;
use std::ops::Range;

pub struct FiboBuilder {
    start_nums: Option<(BigInt, BigInt)>,
    range_by_index: Option<Range<usize>>,
    other_filters: Vec<Box<dyn Fn(&BigInt) -> bool>>,
}

impl FiboBuilder {
    pub fn get_filters(&self) -> &[Box<dyn Fn(&BigInt) -> bool>] {
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


impl Default for FiboBuilder {
    fn default() -> Self {
        Self {
            start_nums: None,
            range_by_index: None,
            other_filters: vec![],
        }
    }
}
