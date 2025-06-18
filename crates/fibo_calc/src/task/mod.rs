mod calculate_fibo_task;

pub use calculate_fibo_task::calculate_fibo_task;

use num_bigint::BigInt;
use tokio::sync::mpsc;

pub enum FiboTaskResult {
    /// Return % progress
    Calculation(u8),
    Result(Vec<BigInt>),
}

pub type FiboTaskSender = mpsc::UnboundedSender<FiboTaskResult>;

pub type FiboTaskReceiver = mpsc::UnboundedReceiver<FiboTaskResult>;
