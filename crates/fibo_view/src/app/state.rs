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
pub struct InputFields {
    pub start1: String,
    pub start2: String,
    pub range_start: String,
    pub range_end: String,
    pub filter_value: String,
}

#[derive(Default)]
pub struct FilterState {
    pub filters: Vec<Filter>,
    pub filter_type: FilterType,
}

#[derive(Default)]
pub struct OutputState {
    pub results: Vec<BigInt>,
    pub list_state: ListState,
}

pub struct AppState {
    pub input: InputFields,
    pub filters: FilterState,
    pub output: OutputState,
    pub input_mode: InputMode,
    pub count_use: usize,
    pub error: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            input: InputFields {
                start1: "0".to_string(),
                start2: "1".to_string(),
                range_start: "0".to_string(),
                range_end: "20".to_string(),
                filter_value: "10".to_string(),
            },
            filters: FilterState {
                filters: Vec::new(),
                filter_type: FilterType::Ge,
            },
            output: OutputState::default(),
            input_mode: InputMode::Normal,
            count_use: 0,
            error: None,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        let mut state = Self::default();
        state.output.list_state.select(Some(0));
        state
    }

    pub fn add_filter(&mut self) -> Result<(), String> {
        let value = domain::calculate_expr(&self.input.filter_value)
            .map_err(|e| e)?;

        self.filters.filters.push(Filter {
            filter_type: self.filters.filter_type.clone(),
            value: BigInt::from(value),
        });
        Ok(())
    }

    pub fn delete_filter(&mut self) {
        self.filters.filters.pop();
    }

    pub fn clear_filters(&mut self) {
        self.filters.filters.clear();
    }

    pub fn scroll_results(&mut self, direction: i32) {
        if self.output.results.is_empty() {
            return;
        }

        let selected = self.output.list_state.selected().unwrap_or(0);
        let new_index = calculate_new_index(selected, direction, self.output.results.len());
        self.output.list_state.select(Some(new_index));
    }

    pub fn calculate(&mut self) -> Result<(), String> {
        self.count_use += 1;

        let calculation_params = self.parse_calculation_parameters()?;
        self.validate_range(&calculation_params)?;

        self.output.results = domain::calculate_fibonacci(
            (calculation_params.start1, calculation_params.start2),
            calculation_params.range_start..calculation_params.range_end,
            &self.filters.filters,
        );

        self.output.list_state.select(Some(0));
        Ok(())
    }

    fn parse_calculation_parameters(&self) -> Result<CalculationParams, String> {
        Ok(CalculationParams {
            start1: parse_big_int(&self.input.start1)?,
            start2: parse_big_int(&self.input.start2)?,
            range_start: parse_usize(&self.input.range_start)?,
            range_end: parse_usize(&self.input.range_end)?,
        })
    }

    fn validate_range(&self, params: &CalculationParams) -> Result<(), String> {
        if params.range_end <= params.range_start {
            return Err("Range end must be > start".to_string());
        }
        Ok(())
    }
}

struct CalculationParams {
    start1: BigInt,
    start2: BigInt,
    range_start: usize,
    range_end: usize,
}

fn calculate_new_index(current: usize, direction: i32, total: usize) -> usize {
    match direction {
        1 => (current + 1) % total,
        -1 => (current + total - 1) % total,
        _ => current,
    }
}

fn parse_big_int(input: &str) -> Result<BigInt, String> {
    domain::calculate_expr(input)
        .map(BigInt::from)
        .map_err(|e| e)
}

fn parse_usize(input: &str) -> Result<usize, String> {
    domain::calculate_expr(input)
        .map(|n| n as usize)
        .map_err(|e| e)
}
