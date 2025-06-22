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
            start1: self
                .parse_expr_as_bigint(&self.input.start1.clone())
                .await?,
            start2: self
                .parse_expr_as_bigint(&self.input.start2.clone())
                .await?,
            range_start: self
                .parse_expr_as_usize(&self.input.range_start.clone())
                .await?,
            range_end: self
                .parse_expr_as_usize(&self.input.range_end.clone())
                .await?,
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


#[cfg(test)]
mod tests {
    use super::*;
    use fibo_calc::FiboTaskResult;
    use test_case::test_case;
    use tokio::sync::mpsc;


    fn create_test_state(
        results: Vec<BigInt>, viewport_start: usize, viewport_size: usize,
    ) -> AppState {
        let mut state = AppState::default();
        state.output.results = results;
        state.output.viewport_start = viewport_start;
        state.output.viewport_size = viewport_size;
        state
    }

    #[test_case(SPEED_SCROLLING, 2, 1 => 0; "viewport too small")]
    #[test_case(10, 9, 1 => SPEED_SCROLLING; "scroll down")]
    #[test_case(10, SPEED_SCROLLING * 2, -1 => SPEED_SCROLLING; "scroll up")]
    fn test_viewport_updates(viewport_size: usize, selected_index: usize, direction: i32) -> usize {
        let mut state = create_test_state(
            (0..20).map(|i| i.into()).collect(),
            if direction == -1 {
                SPEED_SCROLLING * 2
            } else {
                0
            },
            viewport_size,
        );

        state.update_viewport(selected_index, direction);
        state.output.viewport_start
    }

    #[test_case(14, 1 => 5; "scroll down at boundary")]
    #[test_case(0, -1 => 0; "scroll up at boundary")]
    fn test_viewport_boundaries(selected_index: usize, direction: i32) -> usize {
        let mut state = create_test_state(
            (0..15).map(|i| i.into()).collect(),
            if direction == 1 { 5 } else { 0 },
            10,
        );

        state.update_viewport(selected_index, direction);
        state.output.viewport_start
    }

    #[test_case(7, 1; "scroll within safe zone")]
    #[test_case(7, 0; "invalid direction")]
    fn test_viewport_unchanged(selected_index: usize, direction: i32) {
        let initial_viewport_start = 5;
        let mut state = create_test_state(
            (0..20).map(|i| i.into()).collect(),
            initial_viewport_start,
            10,
        );

        state.update_viewport(selected_index, direction);

        assert_eq!(
            state.output.viewport_start, initial_viewport_start,
            "Viewport position should remain unchanged"
        );
    }

    #[test]
    fn test_new_app_state() {
        let state = AppState::new();
        assert_eq!(state.count_use, 0);
        assert!(state.error.is_none());
        assert_eq!(state.output.list_state.selected(), Some(0));
    }

    #[test]
    fn test_clear_filters() {
        let mut state = AppState::new();
        state.filters.filters.push(Filter {
            filter_type: FilterType::Le,
            value: BigInt::from(5),
        });

        state.clear_filters();
        assert!(state.filters.filters.is_empty());
    }

    #[test]
    fn test_delete_filter() {
        let mut state = AppState::new();
        state.filters.filters.push(Filter {
            filter_type: FilterType::Le,
            value: BigInt::from(5),
        });

        state.delete_filter();
        assert!(state.filters.filters.is_empty());
    }

    #[test]
    fn test_scroll_results_empty() {
        let mut state = AppState::new();
        state.scroll_results(1);
        assert_eq!(state.output.list_state.selected(), Some(0));
    }

    #[test]
    fn test_scroll_results_forward() {
        let mut state = AppState::new();
        state.output.results = vec![BigInt::from(1), BigInt::from(2), BigInt::from(3)];
        state.output.list_state.select(Some(0));

        state.scroll_results(1);
        assert_eq!(state.output.list_state.selected(), Some(1));
    }

    #[test]
    fn test_scroll_results_backward() {
        let mut state = AppState::new();
        state.output.results = vec![BigInt::from(1), BigInt::from(2), BigInt::from(3)];
        state.output.list_state.select(Some(1));

        state.scroll_results(-1);
        assert_eq!(state.output.list_state.selected(), Some(0));
    }

    #[test]
    fn test_scroll_results_bounds() {
        let mut state = AppState::new();
        state.output.results = vec![BigInt::from(1), BigInt::from(2)];
        state.output.list_state.select(Some(0));

        state.scroll_results(1);
        state.scroll_results(1);
        assert_eq!(state.output.list_state.selected(), Some(1));

        state.scroll_results(-1);
        state.scroll_results(-1);
        assert_eq!(state.output.list_state.selected(), Some(0));
    }

    #[tokio::test]
    async fn test_add_filter_valid_expression() {
        let mut state = AppState::new();
        state.input.filter_value = "5".to_string();

        let result = state.add_filter().await;
        assert!(result.is_ok());
        assert_eq!(state.filters.filters.len(), 1);
    }

    #[tokio::test]
    async fn test_add_filter_invalid_expression() {
        let mut state = AppState::new();
        state.input.filter_value = "invalid".to_string();

        let result = state.add_filter().await;
        assert!(result.is_err());
    }

    #[test]
    fn test_update_progress_bar() {
        let mut state = AppState::new();
        let (sender, receiver) = mpsc::unbounded_channel();

        state.output.receiver = Some(receiver);

        sender
            .send(FiboTaskResult::Calculation(50.0 as u8))
            .unwrap();
        state.update_progress_bar();
        assert_eq!(state.output.progress, Some(50));
        assert!(state.output.results.is_empty());

        let result = vec![BigInt::from(1), BigInt::from(2)];
        sender.send(FiboTaskResult::Result(result.clone())).unwrap();
        state.update_progress_bar();
        assert_eq!(state.output.results, result);
        assert_eq!(state.output.progress, None);
        assert_eq!(state.output.list_state.selected(), Some(0));
    }

    #[tokio::test]
    async fn test_calculate_invalid_range() {
        let mut state = AppState::new();
        state.input.range_start = "10".to_string();
        state.input.range_end = "5".to_string();
        state.input.start1 = "0".to_string();
        state.input.start2 = "1".to_string();

        state.calculate().await;
        assert_eq!(state.error, Some("Range end must be > start".to_string()));
    }
}
