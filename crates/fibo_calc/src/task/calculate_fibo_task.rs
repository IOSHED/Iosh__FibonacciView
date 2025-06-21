use crate::builder::FilterFn;
use crate::implementation::lineal::LinealFibo;
use crate::{FiboBuilder, FiboTaskResult};
use num_bigint::BigInt;
use rayon::prelude::*;

const CHUNK_SIZE: usize = 1000;


pub async fn calculate_fibo_task(builder: FiboBuilder, sender: crate::task::FiboTaskSender) {
    if builder.is_none_filter() {
        let _ = sender.send(FiboTaskResult::Result(vec![]));
        return;
    }

    let (start_nums, range) = match (builder.get_start_nums(), builder.get_range_by_id()) {
        (Some((n1, n2)), Some(range)) if range.start <= range.end => ((n1, n2), range),
        _ => {
            let _ = sender.send(FiboTaskResult::Result(vec![]));
            return;
        }
    };

    let total_items = range.end.saturating_sub(range.start);
    if total_items == 0 {
        let _ = sender.send(FiboTaskResult::Result(vec![]));
        return;
    }

    let mut result = Vec::with_capacity(total_items);
    let mut processed = 0;

    if range.start == 0 {
        result.push(start_nums.0.clone());
        processed += 1;
        send_progress(&sender, processed, total_items).await;
    }

    if range.start <= 1 && range.end > 1 {
        result.push(start_nums.1.clone());
        processed += 1;
        send_progress(&sender, processed, total_items).await;
    }

    if range.end > 2 {
        let implementation_fibo = LinealFibo::new(Some(start_nums));
        let skip_count = if range.start > 2 { range.start - 2 } else { 0 };
        let take_count = range.end - 2;

        for num in implementation_fibo.skip(skip_count).take(take_count) {
            result.push(num);
            processed += 1;

            if processed % 10 == 0 || processed == total_items {
                send_progress(&sender, processed, total_items).await;
            }
        }
    }

    let filters = builder.get_filters();
    let filtered_result = apply_filters_with_progress(&sender, result, filters).await;

    let _ = sender.send(FiboTaskResult::Result(filtered_result));
}

async fn apply_filters_with_progress(
    sender: &crate::task::FiboTaskSender, numbers: Vec<BigInt>, filters: &[FilterFn],
) -> Vec<BigInt> {
    let total_items = numbers.len();
    if total_items == 0 {
        return numbers;
    }

    if filters.is_empty() {
        send_progress(sender, total_items, total_items).await;
        return numbers;
    }

    let mut filtered = Vec::with_capacity(total_items);
    let mut processed = 0;

    for chunk in numbers.chunks(CHUNK_SIZE) {
        let filtered_chunk: Vec<BigInt> = chunk
            .par_iter()
            .filter(|num| filters.iter().all(|f| f(num)))
            .cloned()
            .collect();

        filtered.extend(filtered_chunk);
        processed += chunk.len();

        send_progress(sender, processed, total_items).await;
    }

    filtered
}

async fn send_progress(sender: &crate::task::FiboTaskSender, processed: usize, total_items: usize) {
    let progress = ((processed as f32 / total_items as f32) * 100.0).clamp(0.0, 100.0) as u8;
    let _ = sender.send(FiboTaskResult::Calculation(progress));
}
