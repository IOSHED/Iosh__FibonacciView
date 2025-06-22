const DEFAULT_START1: &str = "0";
const DEFAULT_START2: &str = "1";
const DEFAULT_RANGE_START: &str = "0";
const DEFAULT_RANGE_END: &str = "20";
const DEFAULT_FILTER_VALUE: &str = "10";

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Start1,
    Start2,
    RangeStart,
    RangeEnd,
    FilterValue,
}

pub struct InputFields {
    pub start1: String,
    pub start2: String,
    pub range_start: String,
    pub range_end: String,
    pub filter_value: String,
}


impl Default for InputFields {
    fn default() -> Self {
        Self {
            start1: DEFAULT_START1.to_string(),
            start2: DEFAULT_START2.to_string(),
            range_start: DEFAULT_RANGE_START.to_string(),
            range_end: DEFAULT_RANGE_END.to_string(),
            filter_value: DEFAULT_FILTER_VALUE.to_string(),
        }
    }
}
