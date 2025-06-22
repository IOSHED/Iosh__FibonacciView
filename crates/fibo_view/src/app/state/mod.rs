mod calculation_params;
mod filter;
mod input;
mod output;

use crate::domain;
use calculation_params::CalculationParams;
use fibo_calc::FiboTaskResult;
pub use filter::FilterType;
pub use filter::{Filter, FilterState};
pub use input::{InputFields, InputMode};
use num_bigint::BigInt;
use output::OutputState;

const PADDING_SCROLLING: usize = 1;
const SPEED_SCROLLING: usize = 1;


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
            input: InputFields::default(),
            filters: FilterState::default(),
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

    pub fn scroll_results(&mut self, direction: i32) {
        if self.output.results.is_empty() {
            return;
        }

        let selected = self.output.list_state.selected().unwrap_or(0);

        let new_selected = match direction {
            1 => (selected + 1).min(self.output.results.len() - 1),
            -1 => selected.saturating_sub(1),
            _ => selected,
        };

        self.output.list_state.select(Some(new_selected));
        self.update_viewport(new_selected, direction);
    }

    fn update_viewport(&mut self, selected_index: usize, direction: i32) {
        let total_items = self.output.results.len();

        if self.output.viewport_size <= SPEED_SCROLLING + 1 {
            return;
        }

        let viewport_end = self.output.viewport_start + self.output.viewport_size;
        let scroll_threshold_down = viewport_end.saturating_sub(PADDING_SCROLLING + 1);
        let scroll_threshold_up = self.output.viewport_start + PADDING_SCROLLING;

        match direction {
            1 if selected_index > scroll_threshold_down => {
                self.output.viewport_start = (self.output.viewport_start + SPEED_SCROLLING)
                    .min(total_items.saturating_sub(self.output.viewport_size));
            }
            -1 if selected_index < scroll_threshold_up => {
                self.output.viewport_start =
                    self.output.viewport_start.saturating_sub(SPEED_SCROLLING);
            }
            _ => {}
        }
    }

    pub fn update_progress_bar(&mut self) {
        let Some(receiver) = self.output.receiver.as_mut() else {
            return;
        };

        if let Ok(msg) = receiver.try_recv() {
            match msg {
                FiboTaskResult::Calculation(progress) => {
                    self.output.results.clear();
                    self.output.progress = Some(progress);
                    self.output.viewport_start = 0;
                }
                FiboTaskResult::Result(res) => {
                    self.output.results = res;
                    self.output.list_state.select(Some(0));
                    self.output.progress = None;
                    self.output.viewport_start = 0;
                }
            }
        }
    }

    pub async fn calculate(&mut self) {
        self.count_use += 1;

        let calculation_params = match self.parse_calculation_parameters().await {
            Ok(params) => params,
            Err(_) => return,
        };

        if calculation_params.range_end <= calculation_params.range_start {
            self.error = Some("Range end must be > start".to_string());
            return;
        }

        self.output.receiver = Some(
            domain::calculate_fibonacci(
                (calculation_params.start1, calculation_params.start2),
                calculation_params.range_start..calculation_params.range_end,
                &self.filters.filters,
            )
            .await,
        );
    }

    async fn parse_calculation_parameters(&mut self) -> Result<CalculationParams, ()> {
        Ok(CalculationParams {
            start1: self.parse_expr_as_bigint(&self.input.start1.clone()).await?,
            start2: self.parse_expr_as_bigint(&self.input.start2.clone()).await?,
            range_start: self.parse_expr_as_usize(&self.input.range_start.clone()).await?,
            range_end: self.parse_expr_as_usize(&self.input.range_end.clone()).await?,
        })
    }

    async fn parse_expr_as_bigint(&mut self, input: &str) -> Result<BigInt, ()> {
        match domain::calculate_expr(input).await {
            Ok(value) => Ok(BigInt::from(value)),
            Err(e) => {
                self.error = Some(e);
                Err(())
            }
        }
    }

    async fn parse_expr_as_usize(&mut self, input: &str) -> Result<usize, ()> {
        match domain::calculate_expr(input).await {
            Ok(value) => Ok(value as usize),
            Err(e) => {
                self.error = Some(e);
                Err(())
            }
        }
    }
}
