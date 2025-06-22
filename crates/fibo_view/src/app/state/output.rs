use fibo_calc::FiboTaskReceiver;
use num_bigint::BigInt;
use ratatui::widgets::ListState;

#[derive(Default)]
pub struct OutputState {
    pub results: Vec<BigInt>,
    pub progress: Option<u8>,
    pub list_state: ListState,
    pub viewport_start: usize,
    pub viewport_size: usize,
    pub receiver: Option<FiboTaskReceiver>,
}
