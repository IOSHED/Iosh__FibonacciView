use crate::domain;
use num_bigint::BigInt;
use ratatui::widgets::ListState;
use std::fmt::Display;

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Start1,
    Start2,
    RangeStart,
    RangeEnd,
    FilterValue,
}

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

impl Display for FilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FilterType::Ge => "≥",
            FilterType::Le => "≤",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

pub struct AppState {
    // Input fields
    pub start1: String,
    pub start2: String,
    pub range_start: String,
    pub range_end: String,
    pub filter_value: String,

    // Filters
    pub filters: Vec<Filter>,
    pub filter_type: FilterType,

    // Output
    pub results: Vec<BigInt>,
    pub list_state: ListState,

    // UI state
    pub input_mode: InputMode,
    pub count_use: usize,
    pub error: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            start1: "0".to_string(),
            start2: "1".to_string(),
            range_start: "0".to_string(),
            range_end: "20".to_string(),
            filter_value: "10".to_string(),
            filter_type: FilterType::Ge,
            filters: Vec::new(),
            results: Vec::new(),
            list_state: ListState::default(),
            input_mode: InputMode::Normal,
            count_use: 0,
            error: None,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            list_state,
            ..Default::default()
        }
    }

    pub fn add_filter(&mut self) {
        match domain::calculate_expr(&self.filter_value) {
            Ok(n) => {
                self.filters.push(Filter {
                    filter_type: self.filter_type.clone(),
                    value: BigInt::from(n),
                });
            },
            Err(e) => {
                self.error = Some(e);
                return;
            }
        }
    }

    pub fn delete_filter(&mut self) {
        if !self.filters.is_empty() {
            self.filters.pop();
        }
    }

    pub fn scroll_results(&mut self, direction: i32) {
        if self.results.is_empty() {
            return;
        }

        let selected = self.list_state.selected().unwrap_or(0);
        let new_index = match direction {
            1 => (selected + 1) % self.results.len(),
            -1 => (selected + self.results.len() - 1) % self.results.len(),
            _ => selected,
        };
        self.list_state.select(Some(new_index));
    }

    pub fn calculate(&mut self) {
        self.error = None;
        self.count_use += 1;

        let start1 = match domain::calculate_expr(&self.start1) {
            Ok(n) => BigInt::from(n),
            Err(e) => {
                self.error = Some(e);
                return;
            }
        };

        let start2 = match domain::calculate_expr(&self.start2) {
            Ok(n) => BigInt::from(n),
            Err(e) => {
                self.error = Some(e);
                return;
            }
        };

        let range_start = match domain::calculate_expr(&self.range_start) {
            Ok(n) => n as usize,
            Err(e) => {
                self.error = Some(e);
                return;
            }
        };

        let range_end = match domain::calculate_expr(&self.range_end) {
            Ok(n) => n as usize,
            Err(e) => {
                self.error = Some(e);
                return;
            }
        };

        if range_end <= range_start {
            self.error = Some("Range end must be > start".to_string());
            return;
        }

        self.results =
            domain::calculate_fibonacci((start1, start2), range_start..range_end, &self.filters);
        self.list_state.select(Some(0));
    }
}
