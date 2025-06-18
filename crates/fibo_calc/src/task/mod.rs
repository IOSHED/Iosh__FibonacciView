mod calculate;

pub use calculate::calculate_fibonacci_task;

use num_bigint::BigInt;
use crate::error::FiboError;

pub enum FiboTaskResult {
    /// Return % progress
    Calculation(u8),
    Result(Vec<BigInt>),
    Error(FiboError)
}
