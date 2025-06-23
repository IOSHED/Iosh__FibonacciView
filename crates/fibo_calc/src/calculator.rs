use crate::builder::FiboBuilder;
use crate::implementation::matmul::MatmulFibo;
use crate::task;
use crate::task::FiboTaskReceiver;
use num_bigint::BigInt;
use crate::implementation::lineal::LinealFibo;

pub trait ImplementationFibo: Iterator<Item = BigInt> {
    fn new(start_nums: Option<(BigInt, BigInt)>) -> Self;
}

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
            task::calculate_fibo_task::<LinealFibo>(self.builder, sender).await;
        });

        receiver
    }

    pub fn calc_one_number(self, n: BigInt) -> BigInt {
        MatmulFibo::new(self.builder.get_start_nums()).calc_one(n)
    }
}
