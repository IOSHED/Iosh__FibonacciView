use num_bigint::BigInt;
use fibo_calc::{FiboBuilder, FiboCalc, FiboTaskResult};
use tokio;

#[tokio::main]
async fn main() {
    println!("=== Fibonacci Calculator Example ===\n");

    let mut builder = FiboBuilder::default();
    builder
        .set_start_nums(Some((
            num_bigint::BigInt::from(0),
            num_bigint::BigInt::from(1),
        )))
        .set_range_by_id(Some(0..50));

    builder.add_filter(|num| num > &BigInt::from(20_000));

    println!("Начинаем расчет чисел Фибоначчи (0..50, только четные)...");
    println!("Задача запущена в фоновом режиме.\n");

    let calc = FiboCalc::new(builder);
    let mut receiver = calc.calc_background();

    while let Some(result) = receiver.recv().await {
        match result {
            FiboTaskResult::Calculation(progress) => {
                println!("Прогресс: {}%", progress);
            }
            FiboTaskResult::Result(numbers) => {
                println!("\n✅ Расчет завершен!");
                println!("Найдено {} четных чисел Фибоначчи:", numbers.len());

                let display_count = numbers.len().min(10);
                for (i, num) in numbers.iter().take(display_count).enumerate() {
                    println!("  {}: {}", i + 1, num);
                }

                if numbers.len() > 10 {
                    println!("  ... и еще {} чисел", numbers.len() - 10);
                }
                break;
            }
        }
    }

    println!("\n🎉 Фоновая задача успешно завершена!");
}
