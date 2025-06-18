
use crate::builder::FiboBuilder;
use crate::task;
use crate::task::FiboTaskResult;

pub struct FiboCalc {
    builder: FiboBuilder,
}

impl FiboCalc {
    pub fn new(builder: FiboBuilder) -> Self {
        Self { builder }
    }

    pub async fn calc(self) -> FiboTaskResult {
        let result = task::calculate_fibonacci_task(self.builder).await;
        FiboTaskResult::Result(result)
    }
}
