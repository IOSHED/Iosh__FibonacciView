use crate::builder::FiboBuilder;
use crate::task;
use crate::task::FiboTaskReceiver;

pub struct FiboCalc {
    builder: FiboBuilder,
}

impl FiboCalc {
    pub fn new(builder: FiboBuilder) -> Self {
        Self { builder }
    }

    pub fn calc_background(self) -> FiboTaskReceiver {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(async move {
            task::calculate_fibo_task(self.builder, sender).await;
        });

        receiver
    }
}
