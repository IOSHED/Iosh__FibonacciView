use crate::{FiboBuilder, FiboTaskResult};
use num_bigint::BigInt;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub async fn calculate_fibo_task(builder: FiboBuilder, sender: crate::task::FiboTaskSender) {
    if builder.is_none_filter() {
        send_result(&sender, vec![]).await;
        return;
    }

    let (start_nums, range) = match validate_inputs(&builder) {
        Some(params) => params,
        None => {
            send_result(&sender, vec![]).await;
            return;
        }
    };

    let total_items = range.end.saturating_sub(range.start);
    if total_items == 0 {
        send_result(&sender, vec![]).await;
        return;
    }

    let result = calculate_fibonacci_sequence(start_nums, range, total_items, &sender).await;

    let filtered_result = apply_filters(&builder, result);
    send_progress(&sender, 100).await;
    send_result(&sender, filtered_result).await;
}

fn validate_inputs(builder: &FiboBuilder) -> Option<((BigInt, BigInt), std::ops::Range<usize>)> {
    match (builder.get_start_nums(), builder.get_range_by_id()) {
        (Some((n1, n2)), Some(range)) if range.start <= range.end => Some(((n1, n2), range)),
        _ => None,
    }
}

async fn calculate_fibonacci_sequence(
    start_nums: (BigInt, BigInt), range: std::ops::Range<usize>, total_items: usize,
    sender: &crate::task::FiboTaskSender,
) -> Vec<BigInt> {
    let mut result = Vec::with_capacity(total_items);
    let mut processed = 0;

    if range.start == 0 {
        result.push(start_nums.0.clone());
        processed += 1;
        send_progress(sender, calculate_progress(processed, total_items)).await;
    }

    if range.start <= 1 && range.end > 1 {
        result.push(start_nums.1.clone());
        processed += 1;
        send_progress(sender, calculate_progress(processed, total_items)).await;
    }

    if range.end > 2 {
        let implementation_fibo = crate::implementation::lineal::LinealFibo::new(Some(start_nums));
        let skip_count = if range.start > 2 { range.start - 2 } else { 0 };
        let take_count = range.end - 2;

        for num in implementation_fibo.skip(skip_count).take(take_count) {
            result.push(num);
            processed += 1;

            if processed % 10 == 0 || processed == total_items {
                send_progress(sender, calculate_progress(processed, total_items)).await;
            }
        }
    }

    result
}

fn apply_filters(builder: &FiboBuilder, result: Vec<BigInt>) -> Vec<BigInt> {
    let filters = builder.get_filters();
    result
        .into_iter()
        .filter(|n| filters.par_iter().all(|func| func(n)))
        .collect()
}

fn calculate_progress(processed: usize, total_items: usize) -> u8 {
    if total_items == 0 {
        return 0;
    }
    let progress = (processed as f64 / total_items as f64) * 100.0;
    progress.clamp(0.0, 100.0) as u8
}

async fn send_progress(sender: &crate::task::FiboTaskSender, progress: u8) {
    let _ = sender.send(FiboTaskResult::Calculation(progress));
}

async fn send_result(sender: &crate::task::FiboTaskSender, result: Vec<BigInt>) {
    let _ = sender.send(FiboTaskResult::Result(result));
}
