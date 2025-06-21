use criterion::{Criterion, criterion_group, criterion_main};
use fibo_calc::{FiboBuilder, FiboCalc, FiboTaskResult};
use std::hint::black_box;
use tokio::runtime::Runtime;

fn bench_fibo_calc(c: &mut Criterion) {
    c.bench_function("fibo_calc 1000..1100", |b| {
        let rt = Runtime::new().unwrap();

        b.iter(|| {
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
            })
        })
    });
}

criterion_group!(benches, bench_fibo_calc);
criterion_main!(benches);
