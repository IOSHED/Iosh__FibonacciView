extern crate alloc;

mod builder;
mod calculator;
mod implementation;
mod task;
mod error;

pub use builder::FiboBuilder;
pub use calculator::FiboCalc;
pub use task::FiboTaskResult;