use crate::domain;
use fibo_calc::{FiboTaskReceiver, FiboTaskResult};
use num_bigint::BigInt;
use ratatui::widgets::ListState;
use std::fmt::Display;

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

struct CalculationParams {
    start1: BigInt,
    start2: BigInt,
    range_start: usize,
    range_end: usize,
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
    pub progress: Option<u8>,
    pub list_state: ListState,
    receiver: Option<FiboTaskReceiver>,
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
                start1: DEFAULT_START1.to_string(),
                start2: DEFAULT_START2.to_string(),
                range_start: DEFAULT_RANGE_START.to_string(),
                range_end: DEFAULT_RANGE_END.to_string(),
                filter_value: DEFAULT_FILTER_VALUE.to_string(),
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

    pub async fn add_filter(&mut self) -> Result<(), String> {
        let value = domain::calculate_expr(&self.input.filter_value).await?;

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

    pub async fn scroll_results(&mut self, direction: i32) {
        if self.output.results.is_empty() {
            return;
        }

        let selected = self.output.list_state.selected().unwrap_or(0);
        let new_index =
            Self::calculate_new_index(selected, direction, self.output.results.len()).await;
        self.output.list_state.select(Some(new_index));
    }

    pub fn update_progress_bar(&mut self) {
        let Some(receiver) = self.output.receiver.as_mut() else {
            return;
        };

        match receiver.try_recv() {
            Some(FiboTaskResult::Calculation(progress)) => {
                self.output.results.clear();
                self.output.progress = Some(progress);
            }
            Some(FiboTaskResult::Result(res)) => {
                self.output.results = res;
                self.output.list_state.select(Some(0));
                self.output.progress = None;
            }
            None => {}
        }
    }


    pub async fn calculate(&mut self) {
        self.count_use += 1;

        let calculation_params = self.parse_calculation_parameters().await;
        self.validate_range(&calculation_params).await;

        self.output.receiver = Some(
            domain::calculate_fibonacci(
                (calculation_params.start1, calculation_params.start2),
                calculation_params.range_start..calculation_params.range_end,
                &self.filters.filters,
            ).await,
        );
    }

    async fn parse_calculation_parameters(&mut self) -> CalculationParams {
        CalculationParams {
            start1: self.parse_big_int(&self.input.start1.clone()).await,
            start2: self.parse_big_int(&self.input.start2.clone()).await,
            range_start: self.parse_usize(&self.input.range_start.clone()).await,
            range_end: self.parse_usize(&self.input.range_end.clone()).await,
        }
    }

    async fn validate_range(&mut self, params: &CalculationParams) {
        if params.range_end <= params.range_start {
            self.error = Some("Range end must be > start".to_string());
        }
    }

    async fn calculate_new_index(current: usize, direction: i32, total: usize) -> usize {
        match direction {
            1 => (current + 1) % total,
            -1 => (current + total - 1) % total,
            _ => current,
        }
    }

    async fn parse_big_int(&mut self, input: &str) -> BigInt {
        domain::calculate_expr(input)
            .await
            .map(BigInt::from)
            .unwrap_or_else(|e| {
                self.error = Some(e);
                BigInt::from(0)
            })
    }

    async fn parse_usize(&mut self, input: &str) -> usize {
        domain::calculate_expr(input)
            .await
            .map(|n| n as usize)
            .unwrap_or_else(|e| {
                self.error = Some(e);
                0
            })
    }
}
