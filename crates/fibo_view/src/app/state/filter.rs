use num_bigint::BigInt;
use std::fmt::Display;

#[derive(Clone)]
pub struct Filter {
    pub filter_type: FilterType,
    pub value: BigInt,
}

#[derive(Clone)]
pub enum FilterType {
    Ge,
    Le,
}

impl Default for FilterType {
    fn default() -> Self {
        Self::Ge
    }
}

impl Display for FilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            FilterType::Ge => "≥",
            FilterType::Le => "≤",
        })
    }
}

#[derive(Default)]
pub struct FilterState {
    pub filters: Vec<Filter>,
    pub filter_type: FilterType,
}
