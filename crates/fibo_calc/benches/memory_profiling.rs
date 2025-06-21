use fibo_calc::{FiboBuilder, FiboCalc, FiboTaskResult};
use std::hint::black_box;
use std::fs;
use tokio::runtime::Runtime;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;


fn main() {
    let output_dir = "../../target/profiling";
    fs::create_dir_all(output_dir).expect("Failed to create directory");

    let _profiler = dhat::Profiler::builder()
        .file_name(format!("{}/dhat-heap.json", output_dir))
        .build();

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mut builder = FiboBuilder::default();
        builder.set_range_by_id(Some(1000..1100));

        let calc = FiboCalc::new(builder);
        let mut receiver = calc.calc_background();

        while let Some(result) = receiver.recv().await {
            if let FiboTaskResult::Result(numbers) = result {
                black_box(numbers);
                break;
            }
        }
    });
}
